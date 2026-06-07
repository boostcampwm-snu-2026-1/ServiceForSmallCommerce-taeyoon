#!/bin/bash
INPUT=$(cat)
STOP_HOOK_ACTIVE=$(echo "$INPUT" | jq -r '.stop_hook_active // false')
if [ "$STOP_HOOK_ACTIVE" = "true" ]; then exit 0; fi
PROJECT_ROOT="${CLAUDE_PROJECT_DIR:-$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)}"

# 코드 작업이 있었던 턴에서만 세션 문서를 생성 (마커 게이트).
# 대화만 한 턴은 무거운 claude --print 호출 없이 즉시 통과.
[ ! -f "${PROJECT_ROOT}/.claude/.pending-doc-check" ] && exit 0

TRANSCRIPT=$(echo "$INPUT" | jq -r '.transcript_path // ""')
[ -z "$TRANSCRIPT" ] || [ ! -f "$TRANSCRIPT" ] && exit 0

SESSION_DIR="${PROJECT_ROOT}/docs/sessions"
TODAY=$(date +%Y%m%d)
NOW=$(date +%H%M%S)
SESSION_FILE="$SESSION_DIR/${TODAY}-${NOW}.md"
mkdir -p "$SESSION_DIR"

CONVERSATION=$(
  jq -r '
    select(.type == "user" or .type == "assistant") |
    if .type == "user" then
      "[사용자] " + (.message.content | if type == "string" then . elif type == "array" then map(select(.type == "text") | .text) | join("") else "" end)
    else
      "[Claude] " + (.message.content | if type == "array" then map(select(.type == "text") | .text) | join("") | .[0:500] elif type == "string" then .[0:500] else "" end)
    end
  ' "$TRANSCRIPT" 2>/dev/null | grep -v '^$' | tail -c 30000
)

[ -z "$CONVERSATION" ] && exit 0

PROMPT_FILE=$(mktemp)
cat > "$PROMPT_FILE" << 'PROMPT_EOF'
아래는 Claude Code 세션의 대화 내용이야. 이 세션에서 있었던 일을 다음 형식의 마크다운 문서로 정리해줘.

# 세션 정리 - DATE_PLACEHOLDER

## 작업 목표
## 진행한 작업
## 주요 결정과 이유
## 발생한 문제와 해결
## 다음 세션에서 이어할 것

---
PROMPT_EOF
echo "DATE_PLACEHOLDER = ${TODAY}" >> "$PROMPT_FILE"
echo "$CONVERSATION" >> "$PROMPT_FILE"

RESULT=$(claude --print "$(cat "$PROMPT_FILE")" 2>/dev/null)
rm -f "$PROMPT_FILE"
[ -n "$RESULT" ] && echo "${RESULT/DATE_PLACEHOLDER/$TODAY}" > "$SESSION_FILE"
