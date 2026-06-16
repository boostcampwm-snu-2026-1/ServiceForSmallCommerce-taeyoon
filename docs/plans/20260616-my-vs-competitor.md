# 내 제품 vs 경쟁사 비교 분석 구현 계획

- **날짜**: 2026-06-16
- **이슈**: #34
- **목표**: 링크 입력을 "내 제품 URL(필수) + 경쟁 상품 URL(1~3개)" 로 구분해 받고, 두 제품군 리뷰를 비교하여 **내 제품에서 개선할 점**을 도출한다.

## 설계 결정

- **입력 모델**: `my_url: String`(필수) + `competitor_urls: Vec<String>`(1~3). 기존 평면 `urls` 요청 필드는 대체.
- **응답/DB는 가급적 additive**:
  - DB: `analyses.my_url TEXT`(nullable) 컬럼 추가. `urls` 는 경쟁사 URL 보관(기존 행 호환).
  - 도메인 `Analysis.my_url: Option<String>`.
  - 응답 뷰: 기존 `urls` 유지 + `my_url` 추가.
  - `Insights.comparison_summary: Option<String>` 추가(`#[serde(default)]`).
  - `ProductSummary.is_mine: bool` / `ProductReviews.is_mine: bool` 추가.
- **역할 구분**: 크롤러는 URL만 알므로 `is_mine=false` 로 생성. **서비스**가 my_url 결과에 `is_mine=true` 설정.
- **분석 관점 전환**: top_complaints/top_positives = 내 제품 기준, improvement_points = 내 제품 개선 우선순위, competitor_weaknesses = 경쟁사 약점(공략 기회), comparison_summary = 비교 총평.

## 서브태스크

| # | 작업 | 대상 | 병렬 |
|---|------|------|------|
| 1 | 백엔드 전체 (도메인/포트/마이그레이션/repo/service/crawler/analyzer/handler/통합테스트) | `backend/**` | ✅ (계약 사전 확정) |
| 2 | 프론트 전체 (types/UrlInput/InsightReport/AnalysisCard/상세헤더/api/dashboard/테스트) | `frontend/**` | ✅ |

> BE/FE는 API 계약을 사전 확정해 병렬로 진행. 메인이 Phase 4에서 양쪽 모두 직접 검증.

## API 계약 (확정)

`POST /api/v1/analyses` 요청:
```json
{ "my_url": "https://...", "competitor_urls": ["https://...", "..."], "review_limit": 100 }
```
응답(202): 기존과 동일 (`analysis_id`, `status`, `created_at`).

`GET /api/v1/analyses/:id` 응답 추가 필드:
- `my_url: string | null`
- `result.products[].is_mine: boolean`
- `result.insights.comparison_summary: string | null`

목록 요약 뷰: `my_url` 추가.

## 테스트 전략
- BE: 단위(mock 분석기 결정론, 집계), testcontainers 통합(my_url 라운드트립), 검증 로직, 통합 플로우(my_url/competitor_urls).
- FE: vitest 컴포넌트/훅/api + type-check.
