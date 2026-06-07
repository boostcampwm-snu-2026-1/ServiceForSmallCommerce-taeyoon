# FE 대시보드 구현 계획

- **날짜**: 2026-06-07
- **로드맵**: docs/plans/20260607-feature-roadmap.md (단위 5/6)
- **목표**: 인증 가드 레이아웃 + 대시보드(URL 입력 → 분석 생성, 히스토리 목록) + UrlInput/AnalysisCard.

## 서브태스크

| # | 작업 | 대상 | 병렬 |
|---|------|------|------|
| 1 | 전체 (컴포넌트→레이아웃→페이지 결합) | 아래 영향 파일 | — |

## 영향 파일

- `frontend/src/features/analysis/components/UrlInput.tsx` (신규) + `UrlInput.test.tsx` — URL 입력 폼(최대 3개 + 리뷰수), props 기반
- `frontend/src/features/analysis/components/AnalysisCard.tsx` (신규) + `AnalysisCard.test.tsx` — 히스토리 항목 카드(상태 배지, 링크)
- `frontend/app/(dashboard)/layout.tsx` (신규) — 인증 가드 + 헤더/사이드바
- `frontend/app/(dashboard)/dashboard/page.tsx` (신규) — UrlInput + 히스토리(useQuery(listAnalyses))
- `frontend/tests/unit/dashboard-page.test.tsx` (신규, 선택) — 제출 시 createAnalysis + 라우팅

> API: `createAnalysis`, `listAnalyses` 이미 존재(src/features/analysis/api). 변경 불필요.

## 설계 결정 (FE 분리 기준)

- UrlInput(폼 로직 100줄 가능)·AnalysisCard(재사용 카드) → `src/features/analysis/components/` 로 추출(테스트 커버리지 대상, .test.tsx 필수).
- 컴포넌트는 props 기반 순수 UI: UrlInput(value/onChange/onSubmit, loading), AnalysisCard(analysis: 목록 항목).
- 인증 가드는 레이아웃(client)에서 useAuthStore 토큰 확인 → 없으면 /login. persist 하이드레이션 플리커는 mounted 가드로 처리.
- 서버 상태(히스토리)는 TanStack Query useQuery. 클라이언트 상태(토큰)는 Zustand.

## 테스트 전략

- `UrlInput.test.tsx`(필수): URL 칸 추가/삭제(최대 3, 최소 1), 리뷰수 변경, 제출 시 onSubmit 이 입력값으로 호출.
- `AnalysisCard.test.tsx`(필수): 주어진 항목의 상태/URL 렌더, /analyses/[id] 링크 href.
- `dashboard-page.test.tsx`(선택): UrlInput 제출 → createAnalysis 모킹 호출 + router.push('/analyses/:id').
