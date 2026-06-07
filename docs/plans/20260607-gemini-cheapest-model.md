# 기본 Gemini 모델을 최저가 모델로 변경 구현 계획

- **날짜**: 2026-06-07
- **목표**: 기본 Gemini 모델을 `gemini-2.5-flash` → `gemini-2.0-flash-lite`(현재 지원 모델 중 최저가, ~$0.075/$0.30 per 1M tok)로 변경한다. API 연동 로직은 변경 없음, 기본 모델 문자열만 교체.

## 설계 판단

- 코드 구조 변경 없음(엔드포인트는 `models/{model}:generateContent` 로 model 을 동적 삽입하므로 모델명만 바꾸면 됨).
- `GEMINI_MODEL` env 로 언제든 override 가능 → 이번 변경은 기본값(fallback) 교체.

## 서브태스크

| # | 작업 | 대상 파일 | 병렬 가능 |
|---|------|-----------|-----------|
| 1 | `DEFAULT_MODEL` 상수 + config fallback + 테스트 헬퍼 기본값 교체 | `backend/src/adapters/gemini/analyzer.rs`, `backend/src/config.rs`, `backend/tests/helpers/app.rs` | ❌ 단일 단위 |
| 2 | env 기본값 교체 | `backend/.env`, `backend/.env.example` | ❌ (1과 동일 단위) |
| 3 | 스펙 동기화 | `specification/dev/tech-stack.md`, `specification/dev/backend.md` | ✅ 코드와 독립 |

## 테스트 전략

- 기존 단위 테스트 유지. `new_uses_default_model_when_empty` 는 상수 비교라 자동 통과.
- 검증: `cargo check` + `cargo test` 전체 통과.
