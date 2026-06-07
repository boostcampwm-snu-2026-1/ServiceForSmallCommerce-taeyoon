# BE 분석 서비스·API

- **날짜**: 2026-06-07
- **플랜**: docs/plans/20260607-be-analysis-api.md
- **로드맵**: docs/plans/20260607-feature-roadmap.md (단위 3/6)

## 작업 요약

분석 비동기 파이프라인 서비스, 인증 추출기, 분석/사용량 HTTP 엔드포인트를 구현해 백엔드 MVP를 완성했다. 플랜 제한 강제는 제외.

## 변경 내용

### 1. 서비스 (application/)
- `analysis_service.rs` — `AnalysisService` trait + `StandardAnalysisService`. `create_analysis`(입력 검증 → repo.create(pending) → `tokio::spawn(process_analysis)`), 분리된 `process_analysis`(crawl→aggregate→analyze→set_result, 실패 시 set_error), 순수 `aggregate_summary`. 단위 6개.
- `user_service.rs` — `UserService` + `StandardUserService`(user_repo + analysis_repo), `get_me`. 단위 2개.

### 2. HTTP
- `extractors.rs` — `AuthUser(Uuid)` FromRequestParts (Bearer + verify_token, Rejection=AppError)
- `handlers/analysis.rs` — POST /analyses(202), GET /analyses/:id(소유권), GET /analyses(목록, Query) — 전부 수동, user_id/review_limit 미노출 뷰
- `handlers/user.rs` — GET /users/me (analyses_limit=null)
- `state.rs` — analysis_service/user_service 필드, `router.rs` — 라우트 3개

### 3. 설정·조립
- `config.rs` — claude_api_key(Option)/claude_model 추가
- `main.rs` — HttpCoupangCrawler + (키 있으면 Claude 아니면 Mock) 분석기 조립
- `tests/helpers/app.rs` — Mock 크롤러/분석기 주입(결정론·무네트워크), register_and_token 헬퍼

### 4. 테스트·스펙
- `tests/analysis_test.rs` — 통합 4개(생성/폴링/목록/사용량, 401, 400, 404 소유권)
- `specification/dev/api.md` — analyses_limit `number | null` 명시

## 주요 결정과 이유

- **process_analysis 분리**: 백그라운드 spawn 과 별개의 연관 async fn으로 분리 → 단위 테스트에서 직접 await 하여 spawn 타이밍 비결정성 회피. 통합 테스트는 GET:id 유한 폴링.
- **/users/me analyses_limit=null**: 플랜 제한 강제 미구현 정책 반영. 월 사용량은 표시용으로만 제공.
- **소유권 통일**: get_analysis 가 불일치/없음 모두 404.
- **main 어댑터 선택**: crawler=Http 고정, analyzer=키 유무로 Claude/Mock. 테스트는 결정론 위해 Mock.

## 검증 결과 (메인 직접 실행)

| 항목 | 명령 | 결과 |
|------|------|------|
| 포맷 | `cargo fmt --check` | ✅ |
| 린트 | `cargo clippy --all-targets -- -D warnings` | ✅ |
| 테스트 | `cargo test` | ✅ lib 32 + analysis 4 + auth 5 + health 1 |

## 커밋

- (5-2에서 기록)
