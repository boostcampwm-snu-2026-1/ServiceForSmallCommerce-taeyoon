# 백엔드 구조

> 위치: `backend/` (monorepo). Cargo 패키지명 `coupang-review-ai-backend`, lib 이름 `coupang_review_ai_backend`.

## 아키텍처: Hexagonal Architecture (Ports & Adapters)

> 아래는 v1 목표 구조다. 초기 스캐폴드는 health 엔드포인트 + 빈 모듈로 시작하며,
> 기능 구현 시 `.claude/code-rules.md`의 "새 API 엔드포인트 추가 순서"를 따라 채운다.

```
backend/src/
├── lib.rs
├── config.rs
├── error.rs
│
├── domain/                    # Zero external deps
│   ├── mod.rs
│   ├── models/                # 핵심 엔티티
│   │   ├── mod.rs
│   │   ├── user.rs
│   │   └── analysis.rs
│   └── ports/                 # trait (port) 정의
│       ├── mod.rs
│       ├── user_repository.rs
│       ├── analysis_repository.rs
│       ├── crawler.rs
│       └── ai_analyzer.rs
│
├── application/               # 비즈니스 로직
│   ├── mod.rs
│   ├── auth_service.rs
│   └── analysis_service.rs
│
├── adapters/                  # 외부 의존성 구현체
│   ├── mod.rs
│   ├── postgres/
│   │   ├── mod.rs
│   │   ├── user_repo.rs
│   │   └── analysis_repo.rs
│   ├── coupang/               # 쿠팡 리뷰 크롤러
│   │   ├── mod.rs
│   │   └── crawler.rs
│   └── gemini/                # Gemini AI 분석
│       ├── mod.rs
│       └── analyzer.rs
│
└── http/                      # Axum 진입점
    ├── mod.rs
    ├── macros.rs
    ├── state.rs
    ├── router.rs
    └── handlers/
        ├── mod.rs
        ├── health.rs
        ├── auth.rs
        └── analysis.rs
```

---

## 레이어 역할

| 레이어 | 역할 | 외부 의존성 |
|--------|------|------------|
| `domain/` | 엔티티 + Port trait 정의 | 없음 |
| `application/` | 비즈니스 로직, Port 사용 | domain만 |
| `adapters/` | Port 구현체 (DB, 크롤러, AI) | sqlx, reqwest, Gemini API |
| `http/` | Axum 핸들러, AppState, 라우터 | axum, application |

---

## AppState 패턴

```rust
#[derive(Clone)]
pub struct AppState {
    pub db_pool: PgPool,
    pub config: Config,
    pub user_service: Arc<dyn UserService>,
    pub analysis_service: Arc<dyn AnalysisService>,
}
```

- 앱 시작 시 한 번 초기화
- `Arc<dyn Trait>`로 서비스 보관 → 테스트 시 Mock 주입 가능
- Axum이 매 요청마다 Clone

---

## 에러 처리 패턴

```rust
#[derive(Debug, Error)]
pub enum AppError {
    NotFound,
    Unauthorized,
    Forbidden(String),     // 플랜 초과 등
    BadRequest(String),
    Database(#[from] sqlx::Error),
    Internal(#[from] anyhow::Error),
}

impl IntoResponse for AppError { ... }
pub type AppResult<T> = Result<T, AppError>;
```

---

## DB 스키마

```sql
-- 사용자
CREATE TABLE users (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email       TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    plan        TEXT NOT NULL DEFAULT 'free',  -- free | starter | pro
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 분석 작업
CREATE TABLE analyses (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id     UUID NOT NULL REFERENCES users(id),
    urls        TEXT[] NOT NULL,
    review_limit INT NOT NULL DEFAULT 100,
    status      TEXT NOT NULL DEFAULT 'pending',  -- pending|crawling|analyzing|completed|failed
    result      JSONB,
    error       TEXT,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    completed_at TIMESTAMPTZ
);
```

---

## 분석 흐름 (비동기)

```
POST /analyses
    └─ analysis_service.create_analysis()
         └─ DB에 status=pending으로 저장
         └─ tokio::spawn() → 백그라운드 태스크
              ├─ status=crawling
              ├─ crawler.fetch_reviews(urls) → Vec<Review>
              ├─ status=analyzing
              ├─ ai_analyzer.analyze(reviews) → AnalysisResult
              ├─ status=completed, result 저장
              └─ (실패 시) status=failed, error 저장

GET /analyses/:id (프론트엔드 폴링)
    └─ status 확인 → completed이면 result 반환
```

---

## 크롤러 전략

- 쿠팡 내부 리뷰 JSON API: `https://www.coupang.com/vp/product/reviews?productId=...`
- User-Agent 설정 필수
- 요청 간 랜덤 딜레이 (500ms ~ 2s)
- 최대 재시도 3회
- Port trait `CoupangCrawler`로 격리 → 테스트 시 Mock 데이터 사용

---

## Gemini API 분석 프롬프트 전략

- 엔드포인트: `POST generativelanguage.googleapis.com/v1beta/models/{model}:generateContent` (인증: `x-goog-api-key` 헤더)
- 리뷰 배치 처리 (100개씩 분할)
- 한국어 프롬프트
- 구조화된 JSON 출력 요청 (`generationConfig.responseMimeType = application/json`)
- 모델: gemini-2.5-flash (env `GEMINI_MODEL` 로 override, `GEMINI_API_KEY` 미설정 시 MockAiAnalyzer 폴백)

---

## 테스트 전략

- `testcontainers`: `cargo test` 실행 시 PostgreSQL 컨테이너 자동 관리
- `TestApp`: 격리된 test_\<uuid\> DB + 랜덤 포트 서버
- 크롤러/Gemini API: Mock Adapter 주입
- 상세: `specification/dev/harness.md`
