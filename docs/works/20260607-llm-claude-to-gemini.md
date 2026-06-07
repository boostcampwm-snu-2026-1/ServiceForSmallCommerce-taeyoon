# LLM 교체: Claude → Gemini

- **날짜**: 2026-06-07
- **플랜**: docs/plans/20260607-llm-claude-to-gemini.md

## 작업 요약

AI 리뷰 분석기를 Anthropic Claude Messages API 에서 Google Gemini `generateContent` API 로 교체했다. `AiAnalyzer` 포트 계약과 응답 도메인 모델(`Insights`)은 그대로 유지하고, 구현체·설정·환경변수·스펙만 교체했다. FE 는 BE API 만 호출하므로 변경 없음.

## 변경 내용

### 1. adapters 모듈 rename: `claude/` → `gemini/`
- `backend/src/adapters/claude/` 삭제, `backend/src/adapters/gemini/` 신설.
- `ClaudeAiAnalyzer` → `GeminiAiAnalyzer` 로 교체.
  - 엔드포인트: `POST https://generativelanguage.googleapis.com/v1beta/models/{model}:generateContent`
  - 인증: `x-goog-api-key` 헤더 (기존 `x-api-key` + `anthropic-version` 제거)
  - 요청 body: `{ contents:[{parts:[{text}]}], generationConfig:{ maxOutputTokens:4096, responseMimeType:"application/json" } }`
  - 응답 파싱 경로: `candidates[0].content.parts[0].text`
  - `endpoint()` 헬퍼 추가, 기본 모델 `DEFAULT_MODEL = "gemini-2.5-flash"`.
- `MockAiAnalyzer` 는 provider 무관 → 그대로 gemini 모듈로 이동(폴백/테스트용).
- `build_prompt` / `parse_insights` 는 동작 동일하게 유지(방어적 JSON 추출 그대로).
- `adapters/mod.rs`: `pub mod claude` → `pub mod gemini`.

### 2. 설정/환경변수
- `backend/src/config.rs`: `claude_api_key`/`claude_model` → `gemini_api_key`/`gemini_model`,
  env `CLAUDE_API_KEY`/`CLAUDE_MODEL` → `GEMINI_API_KEY`/`GEMINI_MODEL`, 기본 모델 `gemini-2.5-flash`.
- `backend/.env.example`: `GEMINI_API_KEY` / `GEMINI_MODEL` 플레이스홀더 추가.
- `backend/.env`(gitignored): 실제 `GEMINI_API_KEY` 저장. **커밋 제외**.

### 3. wiring
- `backend/src/main.rs`: import 및 분석기 주입 분기를 Gemini 기준으로 교체.
  `GEMINI_API_KEY` 미설정 시 `MockAiAnalyzer` 폴백 유지.

### 4. 테스트
- `backend/src/adapters/gemini/analyzer.rs`: 기존 단위 테스트 모두 유지(프롬프트 빌드/JSON 파싱/기본 모델/Mock 집계·결정론) + `endpoint_includes_model_name` 추가.
- `backend/src/application/analysis_service.rs`, `backend/tests/helpers/app.rs`: `claude` import 및 Config 필드를 gemini 로 교체.

### 5. 스펙 동기화
- `specification/dev/tech-stack.md`: AI 분석 스택 → Gemini API (gemini-2.5-flash).
- `specification/dev/backend.md`: 디렉토리 트리(`gemini/`), adapters 의존성, "Gemini API 분석 프롬프트 전략"(엔드포인트/인증/responseMimeType/폴백 명시), Mock 주입 문구.
- `specification/dev/harness.md`: Mock Adapter 문구 Gemini 로 갱신.

## 주요 결정과 이유

- **trait 변경 없음**: `AiAnalyzer` 포트가 이미 존재 → provider 교체는 구현체 교체만으로 끝남(교체 가능성 우선 설계의 효과). 호출부(`StandardAnalysisService`)는 무수정.
- **`responseMimeType: application/json` 사용**: Gemini 는 구조화 JSON 출력을 네이티브 지원 → 파싱 신뢰도 향상. 단 안전을 위해 방어적 `parse_insights`(코드펜스/산문 제거)는 그대로 유지.
- **원자적 리팩터링**: 모듈 rename 으로 모든 import 사이트가 동시에 깨지므로 병렬 서브에이전트로 분해하지 않고 메인이 일괄 수정 후 harness 검증.
- **키 보안**: 실제 API 키는 gitignored `backend/.env` 에만 저장, `.env.example` 에는 플레이스홀더만.

## 검증

- `cargo check` 통과.
- `cargo test`: lib 단위 33 + 통합(analysis 4 / auth 5 / health 1) 전부 통과, 실패 0.
- 코드/테스트에 잔존 `claude`/`anthropic` 참조 없음(grep 확인).

## 커밋

- <commit-id> (루트 단일 커밋, `backend/.env` 제외)
