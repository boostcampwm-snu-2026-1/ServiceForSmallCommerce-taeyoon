#!/bin/bash
INPUT=$(cat)
FILE_PATH=$(echo "$INPUT" | jq -r '.tool_input.file_path // .tool_input.path // ""')
PROJECT_ROOT="${CLAUDE_PROJECT_DIR:-$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)}"

MARKER="${PROJECT_ROOT}/.claude/.pending-doc-check"
RUST_MARKER="${PROJECT_ROOT}/.claude/.pending-rust-check"
SPEC_MARKER="${PROJECT_ROOT}/.claude/.pending-spec-check"
FE_MARKER="${PROJECT_ROOT}/.claude/.pending-fe-check"
PLAN_MARKER="${PROJECT_ROOT}/.claude/.plan-exists"

# docs/plans/ 작성 시 플랜 마커 생성
if echo "$FILE_PATH" | grep -qE 'docs/plans/'; then
  touch "$PLAN_MARKER"
  exit 0
fi

# docs/works/ 또는 specification/ 작성 시 마커 제거
if echo "$FILE_PATH" | grep -qE '(docs/works/|specification/)'; then
  rm -f "$MARKER" "$RUST_MARKER" "$SPEC_MARKER" "$FE_MARKER"
  exit 0
fi

# .claude/code-rules.md 작성 시 spec 마커만 제거
if echo "$FILE_PATH" | grep -qE '\.claude/code-rules\.md$'; then
  rm -f "$SPEC_MARKER"
  exit 0
fi

# 소스 코드 변경 시 doc 마커 생성
if echo "$FILE_PATH" | grep -qE '(backend/src/|frontend/src/|frontend/app/|backend/tests/|backend/migrations/)'; then
  touch "$MARKER"
fi

# 프론트 소스 변경 시 fe-check 마커 생성 (Stop 훅의 커버리지 게이트용)
if echo "$FILE_PATH" | grep -qE '(frontend/src/|frontend/app/)'; then
  touch "$FE_MARKER"
fi

# .rs 변경 시 rust-check 마커 생성
if echo "$FILE_PATH" | grep -qE 'backend/.*\.rs$'; then
  touch "$RUST_MARKER"
fi

# 구조적 BE 파일 변경 시 spec-check 마커 생성
if echo "$FILE_PATH" | grep -qE 'backend/src/(http/(mod|macros|router|state)\.rs|domain/ports/)'; then
  touch "$SPEC_MARKER"
fi
