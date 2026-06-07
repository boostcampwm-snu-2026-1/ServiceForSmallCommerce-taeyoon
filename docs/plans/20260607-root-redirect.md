# 루트(`/`) 인증 상태 기반 리다이렉트 구현 계획

- **날짜**: 2026-06-07
- **목표**: Next.js 기본 보일러플레이트로 남아있는 루트 `app/page.tsx` 를, 인증 상태에 따라 로그인 사용자는 `/dashboard`, 비로그인 사용자는 `/login` 으로 보내는 리다이렉트 페이지로 교체한다. (옵션 1: 별도 랜딩 메인화면은 만들지 않음)

## 설계 판단

- 인증 토큰은 zustand `persist`(localStorage)에 저장 → 클라이언트에서만 판별 가능 → `'use client'` 컴포넌트로 작성.
- 기존 `app/(dashboard)/layout.tsx` 의 인증 가드 패턴(토큰 보유 시/미보유 시 `router.replace`)과 동일한 컨벤션 사용.
- 리다이렉트 전 잠깐 보일 화면은 단순 "로딩 중" 표시(빈 화면 플래시 방지).
- 라우트 페이지(app/)는 `src/features/` 가 아니므로 code-rules 의 `.test.tsx` 의무 대상 아님. trait/hook 분리 불필요(단순 리다이렉트).

## 서브태스크

| # | 작업 | 대상 파일 | 병렬 가능 |
|---|------|-----------|-----------|
| 1 | 루트 page 를 인증 기반 리다이렉트로 교체 | `frontend/app/page.tsx` | ❌ 단일 파일 |
| 2 | 스펙 동기화(프론트 라우트/진입점) | `specification/dev/frontend.md` | ✅ 코드와 독립 |

## 영향 파일

- `frontend/app/page.tsx` (전면 교체)
- `specification/dev/frontend.md` (라우트 표/진입점 설명, 해당 항목 있으면 갱신)

## 테스트 전략

- FE: `npm run type-check` + `npm test -- --run` 전체 통과(기존 테스트 회귀 없음 확인).
- 수동: `/` 접속 시 비로그인 → `/login`, 로그인 상태 → `/dashboard` 로 이동 확인.
