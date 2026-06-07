# 루트(`/`) 인증 상태 기반 리다이렉트

- **날짜**: 2026-06-07
- **플랜**: docs/plans/20260607-root-redirect.md

## 작업 요약

Next.js 기본 보일러플레이트로 남아있던 루트 `app/page.tsx` 를, 인증 상태에 따라 리다이렉트하는 페이지로 교체했다(옵션 1: 별도 랜딩 메인화면 미구현). 로그인 사용자는 `/dashboard`, 비로그인 사용자는 `/login` 으로 이동한다.

## 변경 내용

### 1. `frontend/app/page.tsx` (전면 교체)
- `'use client'` 컴포넌트로 변경. `useAuthStore` 의 `token` 으로 분기:
  - 토큰 보유 → `router.replace('/dashboard')`
  - 미보유 → `router.replace('/login')`
- `app/(dashboard)/layout.tsx` 의 인증 가드와 동일하게 `mounted` 상태로 하이드레이션 미스매치 방지(mount 이후에만 `replace`).
- 리다이렉트 직전 "로딩 중..." 표시로 빈 화면 플래시 방지.

### 2. `frontend/tests/unit/root-page.test.tsx` (신규)
- 기존 페이지 테스트(tests/unit/*) 컨벤션에 맞춰 추가:
  - 토큰 없음 → `/login` 리다이렉트
  - 토큰 있음 → `/dashboard` 리다이렉트
- `next/navigation` mock + `useAuthStore.setState` 로 상태 주입.

### 3. `specification/dev/frontend.md`
- 디렉토리 트리의 `page.tsx` 설명을 "랜딩" → "인증 상태 기반 리다이렉트" 로 갱신.
- "페이지 구성"의 `랜딩 (/)` 섹션을 `루트 (/)` 리다이렉트 동작 설명으로 교체.

## 주요 결정과 이유

- **옵션 1 채택(사용자 선택)**: 서비스 소개 랜딩 화면을 새로 만들지 않고, 루트를 인증 분기 리다이렉트로만 처리해 "메인화면 부재"로 인한 기본 Next 페이지 노출 문제를 최소 비용으로 해결.
- **기존 인증 가드 패턴 재사용**: `mounted + token + router.replace` 컨벤션을 그대로 따라 일관성 유지, persist(localStorage) 복원 타이밍/SSR 미스매치 회피.
- **라우트 페이지지만 테스트 추가**: code-rules 의 `.test.tsx` 의무 대상(`src/features/`)은 아니지만, tests/unit/ 에 페이지 테스트 관례가 있어 리다이렉트 분기 회귀 방지용으로 동반.

## 검증

- `npm run type-check` 통과.
- `npm test -- --run`: 10 파일 / **34** 테스트 전부 통과(루트 리다이렉트 2건 추가).
- 실행 중 dev 서버에서 `/` 응답이 기본 보일러플레이트 → 리다이렉트 페이지로 교체됨 확인, `/login` 200.

## 커밋

- <commit-id> (루트 단일 커밋)
