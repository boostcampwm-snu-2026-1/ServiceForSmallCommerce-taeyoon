#!/bin/bash
INPUT=$(cat)
STOP_HOOK_ACTIVE=$(echo "$INPUT" | jq -r '.stop_hook_active // false')
if [ "$STOP_HOOK_ACTIVE" = "true" ]; then exit 0; fi
PROJECT_ROOT="${CLAUDE_PROJECT_DIR:-$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)}"

cd "${PROJECT_ROOT}" || exit 0
git rev-parse --git-dir &>/dev/null || exit 0

CHANGED=$(git diff --name-only 2>/dev/null; git diff --name-only --cached 2>/dev/null)
CHANGED=$(echo "$CHANGED" | sort -u)
[ -z "$CHANGED" ] && exit 0

MISSING_SPECS=()
check_spec() {
  local spec_file="$1"
  echo "$CHANGED" | grep -q "^${spec_file}$" && return
  MISSING_SPECS+=("$spec_file")
}

echo "$CHANGED" | grep -qE "^backend/src/(application|domain|adapters)/" && check_spec "specification/dev/backend.md"
echo "$CHANGED" | grep -qE "^backend/src/http/(handlers/|router\.rs)" && check_spec "specification/dev/api.md"
echo "$CHANGED" | grep -qE "^frontend/src/features/" && check_spec "specification/dev/frontend.md"

[ ${#MISSING_SPECS[@]} -eq 0 ] && exit 0

echo "⚠️  스펙 동기화 확인 필요:" >&2
printf "  - %s\n" "${MISSING_SPECS[@]}" >&2
