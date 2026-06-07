# LLM 교체: Claude → Gemini 구현 계획

- **날짜**: 2026-06-07
- **목표**: AI 분석기를 Anthropic Claude Messages API 에서 Google Gemini `generateContent` API 로 교체한다. `AiAnalyzer` 포트 계약은 유지하고 구현체/설정/스펙만 교체한다.

## 설계 판단

- `AiAnalyzer` trait 는 이미 존재(교체 가능성 우선 원칙 적용 완료). 이번 작업은 **구현체 교체**이므로 trait 변경 없음.
- `ClaudeAiAnalyzer` → `GeminiAiAnalyzer` 로 교체, 모듈 `adapters/claude/` → `adapters/gemini/` 로 이름 변경.
- `MockAiAnalyzer` 는 provider 무관하므로 그대로 gemini 모듈에 유지(테스트/폴백용).
- 모듈 rename + 모든 import 사이트가 동시에 깨지는 **원자적 리팩터링**이므로 병렬 분해하지 않고 메인이 일괄 수정 후 harness 검증.

## 서브태스크

| # | 작업 | 대상 파일 | 병렬 가능 |
|---|------|-----------|-----------|
| 1 | analyzer 구현체 교체(Gemini generateContent), 모듈 rename | `backend/src/adapters/gemini/{mod,analyzer}.rs`, `adapters/mod.rs` | ❌ 원자적 |
| 2 | config 필드/env 교체 | `backend/src/config.rs`, `backend/.env.example`, `backend/.env` | ❌ (1과 동일 단위) |
| 3 | wiring 교체 | `backend/src/main.rs` | ❌ (1과 동일 단위) |
| 4 | 테스트 import 교체 | `backend/src/application/analysis_service.rs` | ❌ (1과 동일 단위) |
| 5 | 스펙 동기화 | `specification/dev/{tech-stack,backend,harness}.md` | ✅ 코드와 독립 |

## API 매핑 (Claude → Gemini)

| 항목 | Claude | Gemini |
|------|--------|--------|
| 엔드포인트 | `POST api.anthropic.com/v1/messages` | `POST generativelanguage.googleapis.com/v1beta/models/{model}:generateContent` |
| 인증 헤더 | `x-api-key` + `anthropic-version` | `x-goog-api-key` |
| 요청 body | `{model, max_tokens, messages}` | `{contents:[{parts:[{text}]}], generationConfig:{maxOutputTokens, responseMimeType}}` |
| 응답 경로 | `content[0].text` | `candidates[0].content.parts[0].text` |
| 기본 모델 | `claude-sonnet-4-6` | `gemini-2.5-flash` |
| env 키 | `CLAUDE_API_KEY` / `CLAUDE_MODEL` | `GEMINI_API_KEY` / `GEMINI_MODEL` |

- `generationConfig.responseMimeType = "application/json"` 로 JSON 강제 → 파싱 신뢰도 향상. 단 방어적 `parse_insights` 는 유지.

## 테스트 전략

- 기존 단위 테스트(프롬프트 빌드/JSON 파싱/기본 모델/Mock 집계·결정론)는 Gemini 기준으로 유지·보강.
- 네트워크 호출은 테스트하지 않음(기존 방침 유지).
- 검증: `cargo check` + `cargo test` 전체 통과.

## 보안

- 실제 API 키는 `backend/.env`(gitignored)에만 저장. `.env.example` 에는 플레이스홀더만. 커밋 금지.
