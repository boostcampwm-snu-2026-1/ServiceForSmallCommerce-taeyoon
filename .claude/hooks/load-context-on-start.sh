#!/bin/bash
PROJECT_ROOT="${CLAUDE_PROJECT_DIR:-$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)}"
SESSIONS_DIR="${PROJECT_ROOT}/docs/sessions"
WORKS_DIR="${PROJECT_ROOT}/docs/works"

LAST_SESSION=$(ls -t "$SESSIONS_DIR"/*.md 2>/dev/null | head -1)
if [ -n "$LAST_SESSION" ]; then
  echo "## 직전 세션 요약 ($(basename "$LAST_SESSION" .md))"
  cat "$LAST_SESSION"
  echo ""
fi

RECENT_WORKS=$(ls -t "$WORKS_DIR"/*.md 2>/dev/null | head -3)
if [ -n "$RECENT_WORKS" ]; then
  echo "## 최근 작업 로그"
  for f in $RECENT_WORKS; do
    echo "### $(basename "$f" .md)"
    head -20 "$f"
    echo ""
  done
fi
