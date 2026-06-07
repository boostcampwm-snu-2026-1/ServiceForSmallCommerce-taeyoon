#!/bin/bash
INPUT=$(cat)
STOP_HOOK_ACTIVE=$(echo "$INPUT" | jq -r '.stop_hook_active // false')
if [ "$STOP_HOOK_ACTIVE" = "true" ]; then exit 0; fi
PROJECT_ROOT="${CLAUDE_PROJECT_DIR:-$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)}"

RUST_MARKER="${PROJECT_ROOT}/.claude/.pending-rust-check"
SPEC_MARKER="${PROJECT_ROOT}/.claude/.pending-spec-check"
MARKER="${PROJECT_ROOT}/.claude/.pending-doc-check"

if [ -f "$RUST_MARKER" ]; then
  BACKEND_DIR="${PROJECT_ROOT}/backend"
  CHECK_OUTPUT=$(cd "$BACKEND_DIR" && cargo check 2>&1)
  if [ $? -ne 0 ]; then
    echo "{\"decision\": \"block\", \"reason\": \"cargo check 실패. 컴파일 오류를 수정한 후 완료하세요.\"}"
    exit 0
  fi
fi

if [ -f "$SPEC_MARKER" ]; then
  echo "{\"decision\": \"block\", \"reason\": \"구조적 BE 파일이 변경됐는데 specification/ 업데이트가 없습니다.\"}"
  exit 0
fi

if [ -f "$MARKER" ]; then
  TODAY=$(date +%Y%m%d)
  echo "{\"decision\": \"block\", \"reason\": \"소스 코드가 변경됐는데 작업 로그가 없습니다. docs/works/${TODAY}-[작업명].md 를 작성한 후 완료하세요.\"}"
  exit 0
fi
