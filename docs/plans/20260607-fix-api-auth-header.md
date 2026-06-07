# API 클라이언트 인증 헤더 누락 수정 계획

- **날짜**: 2026-06-07
- **목표**: 로그인 후에도 인증 필요 엔드포인트 호출이 401(Unauthorized) 되는 버그 수정. `apiClient` 가 zustand auth store 의 토큰을 읽어 `Authorization: Bearer <token>` 헤더로 첨부하도록 한다.

## 원인 (검증 완료)

- 백엔드 `AuthUser` 추출기는 `Authorization: Bearer <token>` 없으면 401.
- `frontend/src/shared/api/client.ts` 의 `request()` 가 해당 헤더를 전혀 붙이지 않음.
- curl 재현: 토큰 없이 `POST /api/v1/analyses` → 401, 토큰 포함 → 202. → BE 정상, FE 버그 확정.
- 영향 범위: 인증 필요한 모든 호출(analyses 생성/조회/목록, users/me).

## 서브태스크

| # | 작업 | 대상 파일 | 병렬 가능 |
|---|------|-----------|-----------|
| 1 | `request()` 가 `useAuthStore.getState().token` 으로 Authorization 헤더 첨부 | `frontend/src/shared/api/client.ts` | ❌ 단일 파일 |
| 2 | 회귀 방지 테스트 추가(토큰 있으면 헤더 첨부, 없으면 미첨부) | `frontend/src/shared/api/client.test.ts` | ❌ (1과 동일 단위) |

## 테스트 전략

- `global.fetch` mock + `useAuthStore.setState` 로 토큰 주입 → 호출된 fetch 의 headers 검증.
- `npm run type-check` + `npm test -- --run` 전체 통과.
- 수동: 로그인 후 분석 시작 → 202/정상 진행 확인.
