# 기술 스택

## 백엔드

| 항목 | 선택 | 이유 |
|------|------|------|
| 언어 | Rust | 컴파일 타임 안전성. `Result<T, E>` 강제로 에러 처리 누락 불가 |
| 웹 프레임워크 | Axum 0.7 | Tokio 기반 비동기. 타입 안전 라우터 |
| ORM/쿼리 | sqlx 0.7 | 컴파일 타임 SQL 검증. 런타임 ORM 오버헤드 없음 |
| DB | PostgreSQL 16 | 트랜잭션, JSONB, uuid 네이티브 지원 |
| 크롤러 | reqwest + scraper | 쿠팡 내부 리뷰 JSON API 직접 호출 |
| AI 분석 | Gemini API (gemini-2.5-flash) | free tier 호출 가능 + 한국어 리뷰 분석 품질 |
| 인증 | JWT (jsonwebtoken 9) + argon2 | 업계 표준 |

## 프론트엔드

| 항목 | 선택 | 이유 |
|------|------|------|
| 프레임워크 | Next.js 14 (App Router) | 서버 컴포넌트로 초기 로딩 최적화 |
| 언어 | TypeScript | 프론트-백 타입 계약 명시적 관리 |
| 스타일 | Tailwind CSS | 디자인 시스템 없이 빠른 UI 구성 |
| 클라이언트 상태 | Zustand | Redux 대비 보일러플레이트 최소화 |
| 서버 상태 | TanStack Query | 캐시, 폴링(분석 완료 대기), 낙관적 업데이트 |
| UI 컴포넌트 | shadcn/ui | Tailwind 기반, 커스터마이징 용이 |

## 테스트

| 항목 | 선택 | 이유 |
|------|------|------|
| 백엔드 통합 | testcontainers + TestApp | `cargo test`만으로 격리 PostgreSQL 자동 관리 |
| 프론트엔드 단위 | Vitest + Testing Library | Jest 호환, Next.js 통합 자연스러움 |
| E2E | Playwright | 크로스 브라우저, CI 안정적 |
| CI | GitHub Actions | 표준 |

## 리포지토리 구조

단일 레포(monorepo)로 관리한다.

```
service-for-small-business-taeyoon/
├── backend/    # Rust + Axum
└── frontend/   # Next.js 14
```

## 크롤링 전략

공식 API 없음 → 쿠팡 내부 리뷰 JSON 엔드포인트 직접 호출.

- **리스크**: 쿠팡 API 변경 시 수정 필요
- **대응**: 크롤러 모듈(`backend/src/adapters/coupang/`) 격리. Port trait으로 교체 용이
