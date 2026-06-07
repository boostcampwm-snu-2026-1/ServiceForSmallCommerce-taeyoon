# 기본 Gemini 모델을 free-tier 가능한 모델로 변경 계획

- **날짜**: 2026-06-07
- **목표**: 제공된 API 키의 free tier 에서 실제 호출 가능한 모델로 기본값을 변경한다. `gemini-2.0-flash-lite`(이 키 free tier 쿼터 0, 429) → `gemini-2.5-flash`(이 키 free tier 200 확인).

## 진단 근거 (검증 완료)

- `gemini-2.0-flash-lite` 호출 → 429 `free_tier_requests limit: 0` (이 키엔 쿼터 없음).
- `gemini-2.5-flash` 깨끗한 단일 호출 → **200 OK** (정상 응답 확인).
- 일부 모델 404 는 프로젝트에 등록되지 않은 모델 접근 시 발생(전역 models 목록과 별개).

## 서브태스크

| # | 작업 | 대상 파일 | 병렬 가능 |
|---|------|-----------|-----------|
| 1 | `DEFAULT_MODEL` + config fallback + endpoint 테스트 + 테스트 헬퍼 기본값 교체 | `backend/src/adapters/gemini/analyzer.rs`, `backend/src/config.rs`, `backend/tests/helpers/app.rs` | ❌ 단일 단위 |
| 2 | env 기본값 교체 | `backend/.env`, `backend/.env.example` | ❌ (1과 동일 단위) |
| 3 | 스펙 동기화 | `specification/dev/tech-stack.md`, `specification/dev/backend.md` | ✅ 코드와 독립 |

## 테스트 전략

- 기존 단위 테스트 유지. endpoint 테스트의 명시 모델명을 2.5-flash 로 정렬.
- `cargo check` + `cargo test` 전체 통과.
- 수동: 키로 `gemini-2.5-flash:generateContent` 200 확인 완료.
