# 기본 Gemini 모델을 free-tier 가능한 모델로 변경

- **날짜**: 2026-06-07
- **플랜**: docs/plans/20260607-gemini-free-tier-model.md

## 작업 요약

분석 실패 진단 중, 제공된 Gemini API 키가 `gemini-2.0-flash-lite` 에 대해 free tier 쿼터 0(429)임을 확인했다. 같은 키로 `gemini-2.5-flash` 는 200 정상 응답 → 기본 모델을 `gemini-2.5-flash` 로 변경했다.

## 진단 (검증)

- `gemini-2.0-flash-lite:generateContent` → 429 `free_tier_requests limit: 0`.
- `gemini-2.5-flash:generateContent` → 200 OK(정상 응답 본문 확인).
- 일부 모델 404 는 프로젝트 미등록 모델 접근 시 발생(전역 models 목록과 무관).

## 변경 내용

### 1. 코드 기본값
- `backend/src/adapters/gemini/analyzer.rs`: `DEFAULT_MODEL` = `gemini-2.5-flash` + 사유 주석. endpoint 테스트의 명시 모델명도 정렬.
- `backend/src/config.rs`: `gemini_model` fallback + doc 주석.
- `backend/tests/helpers/app.rs`: 테스트 Config 기본값.

### 2. env
- `backend/.env`(gitignored), `backend/.env.example`: `GEMINI_MODEL=gemini-2.5-flash`.

### 3. 스펙
- `specification/dev/tech-stack.md`, `backend.md`: 모델명/사유 갱신.

## 주요 결정과 이유

- **free tier 우선(사용자 결정)**: 비용 최저가(flash-lite)보다 "이 키 free tier 에서 실제 호출되는 모델"이 우선 → 2.5-flash.
- 비용 최저가를 다시 쓰려면 빌링 활성화 또는 flash-lite 쿼터가 있는 키 필요.

## 미해결 (별도 이슈)

- 쿠팡 크롤러는 라이브 쿠팡이 403(anti-bot)을 반환 → 분석 파이프라인 첫 단계에서 실패. access token 으로 해결 불가(인증 게이트가 아닌 WAF 차단, 공개 리뷰 API 없음). 로컬 전체 플로우 검증은 Mock 크롤러 + 실제 Gemini 조합이 현실적.
- 실패 시 DB 에 `"Internal server error"` 만 저장돼 원인 파악이 어려움(`error.rs` Display 고정 + 파이프라인 `e.to_string()`). 에러 전파 개선은 후속 작업 후보.

## 검증

- `cargo test`: lib 33 + 통합 10 전부 통과.
- 키로 `gemini-2.5-flash` 200 확인.

## 커밋

- <commit-id> (루트 단일 커밋, `backend/.env` 제외)
