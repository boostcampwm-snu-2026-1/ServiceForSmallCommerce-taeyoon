# BE 분석 도메인·어댑터

- **날짜**: 2026-06-07
- **플랜**: docs/plans/20260607-be-analysis-domain.md
- **로드맵**: docs/plans/20260607-feature-roadmap.md (단위 2/6)

## 작업 요약

분석 기능의 도메인 모델 + 포트(저장소/크롤러/AI) + PgAnalysisRepository + Mock·real 크롤러/AI 어댑터를 구현했다. 서비스·HTTP 연결은 단위 3에서 진행한다.

## 변경 내용

### 1. 마이그레이션
- `backend/migrations/00003_analyses.sql` — analyses 테이블 + `idx_analyses_user_created` 인덱스

### 2. 도메인 모델 (`domain/models/analysis.rs`)
- `AnalysisStatus`(Pending/Crawling/Analyzing/Completed/Failed, serde lowercase, `as_str`/`from_db_str`, Default=Pending)
- `Review`, 결과 JSONB 구조(`Complaint`/`Positive`/`ImprovementPoint`/`CompetitorWeakness`/`Insights`/`ProductSummary`/`AnalysisResult`), `Analysis` — 모두 api.md snake_case 일치

### 3. 포트
- `AnalysisRepository`(create/find_by_id/list_by_user/update_status/set_result/set_error/count_this_month)
- `CoupangCrawler` + `ProductReviews`, `AiAnalyzer`

### 4. 어댑터
- `PgAnalysisRepository`(런타임 sqlx, JSONB↔AnalysisResult, TEXT[]↔urls, AnalysisRow 중간 매핑) + testcontainer 테스트 3개
- `coupang`: `MockCoupangCrawler`(결정론), `HttpCoupangCrawler`(reqwest + `parse_reviews`/`extract_product_id` 순수 헬퍼)
- `claude`: `MockAiAnalyzer`(결정론 집계), `ClaudeAiAnalyzer`(Claude API + `build_prompt`/`parse_insights` 순수 헬퍼)
- `Cargo.toml`: reqwest 를 일반 의존성으로 이동

## 주요 결정과 이유

- **sqlx 런타임 쿼리 일관 사용**: 단위 1과 동일하게 CI(빈 Postgres) 호환.
- **Mock + real 어댑터 공존**: 크롤러/AI 는 외부 의존이라 trait 필수. 결정론적 Mock 은 단위 3 서비스 테스트와 로컬 dev 에서 재사용. real 구현의 파싱은 순수 헬퍼로 분리해 네트워크 없이 단위 테스트.
- **real 어댑터 방어적 파싱**: 쿠팡 리뷰 JSON 스키마가 비공개라 여러 키 후보를 시도하고 실패 항목 skip. doc 주석에 통합 시 보정 필요 명시. Claude 응답은 첫 `{`~마지막 `}` 추출 후 파싱.
- **ProductSummary 통계 집계는 단위 3(서비스)**: AI가 아닌 결정론적 집계이므로 AiAnalyzer 책임에서 분리.

## 검증 결과 (메인 직접 실행)

| 항목 | 명령 | 결과 |
|------|------|------|
| 포맷 | `cargo fmt --check` | ✅ |
| 린트 | `cargo clippy --all-targets -- -D warnings` | ✅ |
| 테스트 | `cargo test` | ✅ lib 24(신규 13) + auth 5 + health 1 |

## 커밋

- (5-2에서 기록)
