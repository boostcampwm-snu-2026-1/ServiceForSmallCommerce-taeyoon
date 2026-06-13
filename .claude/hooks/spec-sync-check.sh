#!/bin/bash
INPUT=$(cat)
STOP_HOOK_ACTIVE=$(echo "$INPUT" | jq -r '.stop_hook_active // false')
if [ "$STOP_HOOK_ACTIVE" = "true" ]; then exit 0; fi
PROJECT_ROOT="${CLAUDE_PROJECT_DIR:-$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)}"

cd "${PROJECT_ROOT}" || exit 0
git rev-parse --git-dir &>/dev/null || exit 0

# 스펙의 최종 현황은 GitHub Wiki(canonical). 구조적 코드 변경 시,
# 갱신이 필요할 수 있는 Wiki 페이지를 비차단으로 리마인드한다. (.claude/github-workflow.md)
CHANGED=$(git diff --name-only 2>/dev/null; git diff --name-only --cached 2>/dev/null)
CHANGED=$(echo "$CHANGED" | sort -u)
[ -z "$CHANGED" ] && exit 0

PAGES=()
echo "$CHANGED" | grep -qE "^backend/src/(application|domain|adapters)/" && PAGES+=("Backend")
echo "$CHANGED" | grep -qE "^backend/src/http/(handlers/|router\.rs)" && PAGES+=("API")
echo "$CHANGED" | grep -qE "^frontend/src/features/" && PAGES+=("Frontend")

[ ${#PAGES[@]} -eq 0 ] && exit 0

echo "⚠️  코드 변경 감지 — GitHub Wiki 페이지 갱신이 필요한지 확인하세요:" >&2
printf "  - Wiki: %s\n" "${PAGES[@]}" >&2
