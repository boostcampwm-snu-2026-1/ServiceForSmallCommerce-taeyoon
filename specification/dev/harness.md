# 테스트 하네스 전략

## 테스트 계층

```
컴파일타임 (cargo check / tsc --noEmit)
    └─ 타입 에러, 미구현 trait, 없는 함수 → 즉시 검출

단위 테스트 (cargo test --lib / vitest)
    └─ application/, adapters/ 각 모듈의 #[cfg(test)] 블록
    └─ Mock Port 주입으로 외부 의존성 없이 비즈니스 로직 검증

통합 테스트 (cargo test --tests / vitest)
    └─ TestApp + testcontainers PostgreSQL
    └─ HTTP 요청 → DB 저장 → 응답 전체 흐름
    └─ 크롤러/Gemini API는 Mock Adapter 사용

E2E (Playwright)
    └─ 실제 브라우저 + 실제 서버
    └─ 핵심 사용자 플로우만
```

---

## TestApp 패턴 (testcontainers 기반)

### 핵심 특징

- `cargo test` 실행 시 자동으로 PostgreSQL 컨테이너 스핀업
- 로컬 PostgreSQL 설치 불필요
- 프로세스당 컨테이너 1개 공유 (`OnceCell`) → 성능 최적화
- 각 TestApp은 격리된 `test_<uuid>` DB → 테스트 간 상태 오염 없음
- TestApp Drop 시 test DB 자동 삭제

### 사용 예시

```rust
#[tokio::test]
async fn test_create_analysis() {
    let app = TestApp::spawn().await;
    let client = reqwest::Client::new();

    // 1. 회원가입
    let register_res = client
        .post(&format!("{}/api/v1/auth/register", app.address))
        .json(&serde_json::json!({
            "email": "test@example.com",
            "password": "password123"
        }))
        .send().await.unwrap();
    assert_eq!(register_res.status(), 201);
    let body: serde_json::Value = register_res.json().await.unwrap();
    let token = body["token"].as_str().unwrap();

    // 2. 분석 요청
    let analysis_res = client
        .post(&format!("{}/api/v1/analyses", app.address))
        .bearer_auth(token)
        .json(&serde_json::json!({
            "urls": ["https://www.coupang.com/vp/products/12345"],
            "review_limit": 50
        }))
        .send().await.unwrap();
    assert_eq!(analysis_res.status(), 202);
}
```

### Mock Adapter 주입

```rust
// 크롤러 Mock
struct MockCrawler;
#[async_trait]
impl CoupangCrawler for MockCrawler {
    async fn fetch_reviews(&self, url: &str, limit: u32) -> Result<Vec<Review>> {
        Ok(vec![
            Review { text: "포장이 너무 허술해요".to_string(), rating: 2 },
            Review { text: "가격 대비 품질 좋아요".to_string(), rating: 5 },
        ])
    }
}

// TestApp에 주입
impl TestApp {
    pub async fn spawn_with_mock() -> Self {
        // MockCrawler, MockAiAnalyzer 주입
    }
}
```

---

## 로컬 개발 명령어

```bash
# 백엔드 (testcontainers 자동 처리)
cd backend
make test           # cargo test
make run            # docker-compose DB + migrate + cargo run

# 프론트엔드
cd frontend
npm test -- --run   # vitest
npx playwright test # E2E
```

---

## AI Agent 코드 수정 규칙

1. **컴파일 먼저**: `cargo check` / `tsc --noEmit` 통과 없이 작업 완료 불가
2. **테스트 통과**: `cargo test` / `npm test -- --run` 전체 통과 필수
3. **API 계약 유지**: 기존 응답 필드 제거/타입 변경 금지 (추가만 허용)
4. **테스트 동반**: 새 엔드포인트/기능 추가 시 통합 테스트 필수
5. **스펙 동기화**: 기능/설계 변경 시 `specification/` 업데이트 필수

---

## CI 파이프라인

```
push/PR → GitHub Actions
    ├─ cargo check
    ├─ cargo fmt --check
    ├─ cargo clippy -- -D warnings
    ├─ cargo test (testcontainers 사용)
    ├─ tsc --noEmit
    ├─ npm run lint
    ├─ vitest run
    └─ playwright test
```
