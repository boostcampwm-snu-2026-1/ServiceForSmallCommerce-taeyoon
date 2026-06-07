# 기본 Gemini 모델을 최저가 모델로 변경

- **날짜**: 2026-06-07
- **플랜**: docs/plans/20260607-gemini-cheapest-model.md

## 작업 요약

기본 Gemini 모델을 `gemini-2.5-flash` → `gemini-2.0-flash-lite`(현재 지원 모델 중 최저가, ~$0.075 input / $0.30 output per 1M tok)로 변경했다. API 연동 로직은 무변경, 기본 모델 문자열만 교체. `GEMINI_MODEL` env 로 언제든 override 가능.

## 모델 선택 근거

사용자에게 "가장 싼 모델" 후보를 제시하고 확인받음:
- `gemini-2.0-flash-lite` (~$0.075/$0.30) — **선택**. 현재 지원 모델 중 최저가, 안정적.
- `gemini-2.5-flash-lite` (~$0.10/$0.40) — 더 비쌈.
- `gemini-1.5-flash-8b` (~$0.0375/$0.15) — 역대 최저가지만 1.5 세대 API 지원 종료(deprecated) 진행 중 → 호출 실패 위험으로 제외.

## 변경 내용

### 1. 코드 기본값
- `backend/src/adapters/gemini/analyzer.rs`: `DEFAULT_MODEL` = `gemini-2.0-flash-lite`. `endpoint_includes_model_name` 테스트의 명시 모델 인자도 동일하게 정렬.
- `backend/src/config.rs`: `gemini_model` fallback 문자열 교체 + doc 주석.
- `backend/tests/helpers/app.rs`: 테스트 Config 기본값 교체.

### 2. env
- `backend/.env`(gitignored), `backend/.env.example`: `GEMINI_MODEL=gemini-2.0-flash-lite`.

### 3. 스펙
- `specification/dev/tech-stack.md`, `specification/dev/backend.md`: 모델명 및 선정 사유 갱신.

## 검증

- `cargo test`: lib 단위 33 + 통합(analysis 4 / auth 5 / health 1) 전부 통과, 실패 0.
- 코드/테스트에 잔존 `2.5-flash` 참조 없음(grep 확인).

## 커밋

- <commit-id> (루트 단일 커밋, `backend/.env` 제외)
