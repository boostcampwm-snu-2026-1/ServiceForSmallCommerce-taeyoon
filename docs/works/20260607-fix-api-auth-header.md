# API 클라이언트 인증 헤더 누락 수정

- **날짜**: 2026-06-07
- **플랜**: docs/plans/20260607-fix-api-auth-header.md

## 작업 요약

로그인 후 "분석 시작"을 누르면 `Unauthorized`(401)가 뜨던 버그를 수정했다. 프론트 `apiClient` 가 인증 토큰을 요청 헤더에 싣지 않아, 인증 필요 엔드포인트가 전부 401이었다. `request()` 가 zustand auth store 의 토큰을 읽어 `Authorization: Bearer <token>` 를 첨부하도록 고쳤다.

## 원인 (검증 완료)

- 백엔드 `AuthUser` 추출기(`backend/src/http/extractors.rs`)는 `Authorization: Bearer <token>` 가 없으면 무조건 `Unauthorized`.
- `frontend/src/shared/api/client.ts` 의 `request()` 가 해당 헤더를 전혀 붙이지 않음 → 토큰은 store(localStorage)에 있는데 요청엔 안 실림.
- curl 재현으로 확정: 토큰 없이 `POST /api/v1/analyses` → 401, 토큰 포함 → 202. (BE 정상, FE 버그)
- 영향: 인증 필요한 모든 호출(analyses 생성/조회/목록, users/me). 대시보드 히스토리도 사실 401이었으나 빈 목록처럼 보였음.

## 변경 내용

### 1. `frontend/src/shared/api/client.ts`
- `useAuthStore.getState().token` 으로 토큰을 읽어, 존재하면 `Authorization: Bearer <token>` 헤더 첨부.
- 헤더 병합 순서: `Content-Type` → `authHeader` → 호출자 `init.headers`(오버라이드 허용).
- 비로그인(로그인/회원가입) 호출은 토큰이 없으므로 헤더 미첨부 → 기존 동작 유지.

### 2. `frontend/src/shared/api/client.test.ts` (신규)
- `global.fetch` mock + `useAuthStore.setState` 로 검증:
  - 토큰 있으면 `Authorization: Bearer <token>` 첨부
  - 토큰 없으면 미첨부 + `Content-Type` 유지
  - non-ok 응답 시 백엔드 `error` 메시지로 throw

## 주요 결정과 이유

- **store 직접 접근(`getState`)**: API 클라이언트는 React 컴포넌트 밖이라 hook 사용 불가 → zustand 의 `getState()` 로 최신 토큰을 동기 조회. 순환 import 없음(store 는 client 를 import 하지 않음).
- **인터셉터/리프레시는 범위 외**: 401 시 자동 로그아웃·토큰 갱신은 별도 작업으로 분리(이번엔 헤더 첨부만).

## 검증

- `npm run type-check` 통과.
- `npm test -- --run`: 11 파일 / **37** 테스트 전부 통과(client 인증 헤더 3건 추가).
- curl: 토큰 포함 `POST /api/v1/analyses` → 202 확인.

## 커밋

- <commit-id> (루트 단일 커밋)
