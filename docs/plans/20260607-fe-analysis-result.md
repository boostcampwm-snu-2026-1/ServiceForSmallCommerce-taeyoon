# FE 분석 결과 구현 계획

- **날짜**: 2026-06-07
- **로드맵**: docs/plans/20260607-feature-roadmap.md (단위 6/6, 최종)
- **목표**: 분석 결과 폴링 훅 + 결과 페이지(진행중/완료/실패) + InsightReport. PDF 등 수익 기능 제외.

## 서브태스크

| # | 작업 | 대상 | 병렬 |
|---|------|------|------|
| 1 | 전체 (훅→컴포넌트→페이지 결합) | 아래 영향 파일 | — |

## 영향 파일

- `frontend/src/features/analysis/hooks/useAnalysisPolling.ts` (신규) + `useAnalysisPolling.test.ts` — useQuery refetchInterval 2초, completed/failed 중단. 순수 `pollInterval(status)` 헬퍼 분리.
- `frontend/src/features/analysis/components/InsightReport.tsx` (신규) + `InsightReport.test.tsx` — 개선 TOP3/경쟁사 약점/구매 결정 요인/불만·긍정/평점 분포
- `frontend/app/(dashboard)/analyses/[id]/page.tsx` (신규) — 폴링 훅 사용, 상태별 UI
- `frontend/tests/unit/analysis-result-page.test.tsx` (신규, 선택) — completed 시 리포트 렌더, 진행중 로딩

## 설계 결정 (FE 분리 기준)

- 폴링(비동기 데이터 패턴) → `hooks/useAnalysisPolling.ts` 로 추출. refetchInterval 분기는 순수 `pollInterval(status): number | false` 로 분리해 결정론 테스트.
- InsightReport(독립 UI 블록, 100줄+) → `components/InsightReport.tsx`. props: `result: AnalysisResult`.
- 평점 분포는 InsightReport 내 간단한 바(외부 차트 라이브러리 없이 div 비율). 필요시 별도 컴포넌트로 추출 가능하나 1차는 내부.
- API는 기존 `getAnalysis` 재사용.

## 테스트 전략

- `useAnalysisPolling.test.ts`(필수): `pollInterval('pending')===2000`, `pollInterval('completed')===false`, `pollInterval('failed')===false`; renderHook(QueryClientProvider) 로 getAnalysis 모킹 후 데이터 반환.
- `InsightReport.test.tsx`(필수): 픽스처 AnalysisResult 로 개선 포인트 제목/경쟁사 약점/구매 요인/평점 분포 렌더 검증.
- `analysis-result-page.test.tsx`(선택): getAnalysis completed 모킹 시 리포트, pending 모킹 시 진행중 UI.
