# BE 분석 서비스·API 구현 계획

- **날짜**: 2026-06-07
- **로드맵**: docs/plans/20260607-feature-roadmap.md (단위 3/6)
- **목표**: 분석 서비스(백그라운드 파이프라인) + 인증 미들웨어 + 분석/사용량 HTTP 엔드포인트. 플랜 제한 강제는 제외.

## 서브태스크

| # | 작업 | 대상 | 병렬 |
|---|------|------|------|
| 1 | 전체 (서비스→미들웨어→핸들러→와이어링 강결합) | 아래 영향 파일 | — |

> state.rs/router.rs/main.rs/config.rs 공유 파일을 동시에 만져야 하므로 단일 서브에이전트.

## 영향 파일

- `backend/src/application/analysis_service.rs` (신규) — `AnalysisService` trait + `StandardAnalysisService` + 백그라운드 파이프라인 + `aggregate_summary` 순수 헬퍼 + #[cfg(test)]
- `backend/src/application/user_service.rs` (신규) — `UserService` trait + `StandardUserService`(user_repo + analysis_repo) + #[cfg(test)]
- `backend/src/application/mod.rs` (수정)
- `backend/src/http/extractors.rs` (신규) — `AuthUser(Uuid)` FromRequestParts 추출기 (Bearer 토큰 검증)
- `backend/src/http/handlers/analysis.rs` (신규) — POST/GET:id/GET 목록 (수동 핸들러)
- `backend/src/http/handlers/user.rs` (신규) — GET /users/me
- `backend/src/http/handlers/mod.rs`, `backend/src/http/mod.rs` (수정)
- `backend/src/http/state.rs` (수정) — analysis_service, user_service 필드 추가
- `backend/src/http/router.rs` (수정) — 라우트 4개 추가
- `backend/src/config.rs` (수정) — `claude_api_key: Option<String>`, `claude_model` 추가
- `backend/src/main.rs` (수정) — 서비스 조립(real 어댑터, 키 없으면 Mock 폴백)
- `backend/tests/helpers/app.rs` (수정) — AppState 조립(**Mock 크롤러/분석 주입** → 결정론·무네트워크)
- `backend/tests/analysis_test.rs` (신규) — 통합 테스트
- `specification/dev/api.md` (수정) — /users/me 의 analyses_limit 를 nullable(null=제한 없음) 로 명시

## 설계 결정 (교체 가능성)

- `AnalysisService`, `UserService` 모두 trait + Standard 구현 (AppState 에 Arc<dyn _>). 통합 테스트는 real 서비스 + Mock 어댑터.
- **백그라운드 파이프라인은 별도 async fn `process_analysis(repo, crawler, analyzer, id, urls, limit)` 로 분리** 후 create_analysis 가 `tokio::spawn` 으로 호출. 테스트는 process_analysis 를 직접 await → spawn 타이밍 비결정성 회피. (BackgroundTask trait 까지는 과설계 → 주입된 crawler/analyzer trait 로 충분)
- `aggregate_summary(&ProductReviews) -> ProductSummary` 순수 함수(개수/평균/평점분포) — AI 아닌 결정론 집계, 단위 테스트.
- 소유권 검증: get_analysis/list 는 user_id 로 스코프. 타 유저 분석 조회는 NotFound(404).
- **플랜 제한 강제 없음**: 입력 검증(urls 1~3개, review_limit 50~500)만 수행. /users/me 의 analyses_limit 는 `null`(제한 없음). 월 사용량 카운트는 표시용으로 제공.

## 테스트 전략

- 단위(analysis_service): Mock repo/crawler/analyzer 로 ① process_analysis 후 status=completed + result.products/insights 검증, ② 크롤러 실패 시 status=failed, ③ aggregate_summary 평균/분포 정확성, ④ create 입력 검증(0개/4개 urls, 범위 밖 limit → BadRequest).
- 단위(user_service): Mock repo 로 get_me 가 (user, 이번달 카운트) 반환.
- 통합(analysis_test): register→POST 202→GET:id 폴링(유한 루프)→completed+result 검증, 목록/사용량, 무토큰 401, 잘못된 입력 400, 타유저 404.
