#!/bin/bash
INPUT=$(cat)
FILE_PATH=$(echo "$INPUT" | jq -r '.tool_input.file_path // .tool_input.path // ""')
PROJECT_ROOT="${CLAUDE_PROJECT_DIR:-$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)}"

# docs/plans/ 작성 자체는 허용
if echo "$FILE_PATH" | grep -qE 'docs/plans/'; then exit 0; fi

# 소스 파일이 아니면 통과
if ! echo "$FILE_PATH" | grep -qE '(backend/src/|frontend/src/|frontend/app/|backend/tests/|backend/migrations/)'; then
  exit 0
fi

PLAN_MARKER="${PROJECT_ROOT}/.claude/.plan-exists"
if [ -f "$PLAN_MARKER" ]; then exit 0; fi

echo '{"decision": "block", "reason": "소스 코드를 수정하기 전에 작업 플랜을 먼저 작성하세요.\n\ndocs/plans/YYYYMMDD-[작업명].md 파일을 작성한 뒤 다시 시도하세요."}'
