# FE 대시보드

- **날짜**: 2026-06-07
- **플랜**: docs/plans/20260607-fe-dashboard.md
- **로드맵**: docs/plans/20260607-feature-roadmap.md (단위 5/6)

## 작업 요약

인증 가드 레이아웃과 대시보드(URL 입력 → 분석 생성, 히스토리 목록)를 구현하고, 재사용 컴포넌트 UrlInput/AnalysisCard를 추출했다.

## 변경 내용

- `src/features/analysis/components/UrlInput.tsx` (+test 6) — props 기반 폼(URL 1~3개 동적, 리뷰수 50/100/200/500), 빈 url 제외 후 onSubmit, loading 처리
- `src/features/analysis/components/AnalysisCard.tsx` (+test 4) — 상태 배지(진행중/완료/실패 색상), url 요약, /analyses/[id] 링크
- `app/(dashboard)/layout.tsx` — 인증 가드(mounted+token, 미인증 시 /login replace) + 헤더(로그아웃)
- `app/(dashboard)/dashboard/page.tsx` — UrlInput 제출 → createAnalysis → 결과 페이지 이동, useQuery(listAnalyses) 히스토리
- `tests/unit/dashboard-page.test.tsx` (test 2) — 제출 흐름 + 빈 히스토리

## 주요 결정과 이유

- **하이드레이션 가드**: zustand persist 플리커 방지를 위해 mounted state 사용, mounted 후 token 없을 때만 리다이렉트.
- **컴포넌트 분리**: 재사용·테스트 커버리지 위해 src/features/analysis/components/ 로 추출(skill FE 분리 기준).
- API는 기존 createAnalysis/listAnalyses 재사용(신규 없음). 수익 UI(요금제/PDF/플랜) 미구현.

## 검증 결과 (메인 직접 실행)

| 항목 | 명령 | 결과 |
|------|------|------|
| 타입 | `npm run type-check` | ✅ |
| 린트 | `npm run lint` | ✅ |
| 테스트 | `npm test -- --run` | ✅ 18 passed (신규 12) |

## 커밋

- (5-2에서 기록)
