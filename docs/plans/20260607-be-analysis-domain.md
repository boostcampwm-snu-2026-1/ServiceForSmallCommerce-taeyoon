# BE 분석 도메인·어댑터 구현 계획

- **날짜**: 2026-06-07
- **로드맵**: docs/plans/20260607-feature-roadmap.md (단위 2/6)
- **목표**: 분석 기능의 도메인 모델 + 포트(저장소/크롤러/AI) + PgAnalysisRepository + Mock 가능한 크롤러/AI 어댑터. 서비스·HTTP는 단위 3.

## 서브태스크

| # | 작업 | 대상 | 병렬 |
|---|------|------|------|
| 1 | 전체 (모델→포트→어댑터 강결합) | 아래 영향 파일 | — |

## 영향 파일

- `backend/migrations/00003_analyses.sql` (신규) — analyses 테이블
- `backend/src/domain/models/analysis.rs` (신규), `models/mod.rs` (수정) — Analysis, AnalysisStatus, Review, AnalysisResult/ProductSummary/Insights 등
- `backend/src/domain/ports/analysis_repository.rs` (신규) — `AnalysisRepository` trait
- `backend/src/domain/ports/crawler.rs` (신규) — `CoupangCrawler` trait + `ProductReviews`
- `backend/src/domain/ports/ai_analyzer.rs` (신규) — `AiAnalyzer` trait
- `backend/src/domain/ports/mod.rs` (수정)
- `backend/src/adapters/postgres/analysis_repo.rs` (신규) — `PgAnalysisRepository` + testcontainer 테스트
- `backend/src/adapters/coupang/mod.rs` + `crawler.rs` (신규) — `MockCoupangCrawler`(결정론적), `HttpCoupangCrawler`(real) + 파싱 헬퍼 테스트
- `backend/src/adapters/claude/mod.rs` + `analyzer.rs` (신규) — `MockAiAnalyzer`(결정론적), `ClaudeAiAnalyzer`(real) + 파싱 헬퍼 테스트
- `backend/src/adapters/mod.rs` (수정) — `pub mod coupang; pub mod claude;`
- `backend/Cargo.toml` (수정) — reqwest 를 일반 의존성으로 추가 (현재 dev-only)
- `specification/dev/api.md` — 변경 없음 (명세 일치)

## 설계 결정 (교체 가능성)

- `AnalysisRepository`, `CoupangCrawler`, `AiAnalyzer` 모두 trait. 크롤러/AI 는 외부 의존(쿠팡/Claude)이라 테스트 시 Mock 주입이 필수 → trait 필수.
- 각 어댑터 모듈에 **공개(pub) 결정론적 Mock** 과 **real 구현** 을 함께 제공:
  - 크롤러: `MockCoupangCrawler`(고정 픽스처), `HttpCoupangCrawler`(reqwest, `parse_reviews` 순수 헬퍼 분리)
  - AI: `MockAiAnalyzer`(키워드 집계 기반 결정론적), `ClaudeAiAnalyzer`(Claude API, `parse_insights` 순수 헬퍼 분리)
- ProductSummary(통계: 개수/평균/분포)는 AI가 아니라 리뷰에서 결정론적으로 집계 → 단위 3 서비스에서 계산. AiAnalyzer 는 `Insights` 만 생성.
- `result`(AnalysisResult)는 JSONB 직렬화. `urls`는 TEXT[].

## 테스트 전략

- `analysis_repo.rs`: testcontainer Postgres. FK 때문에 user 행 선삽입 후 create→find_by_id→update_status→set_result→list_by_user→count_this_month 검증.
- `crawler.rs`: `MockCoupangCrawler` 결정론(동일 입력 동일 출력), `parse_reviews` 픽스처 JSON 파싱.
- `analyzer.rs`: `MockAiAnalyzer` 결정론(불만/긍정 집계), `parse_insights` 픽스처 JSON 파싱.
