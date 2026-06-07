#!/bin/bash
PROJECT_ROOT="${CLAUDE_PROJECT_DIR:-$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)}"
FE_DIR="${PROJECT_ROOT}/frontend"
[ ! -d "$FE_DIR" ] && exit 0
[ ! -d "$FE_DIR/node_modules" ] && exit 0

cd "$FE_DIR" || exit 0
npm test -- --run --coverage --silent 2>&1 | grep -E "^(All files|src/|Coverage|Threshold)" >&2
exit ${PIPESTATUS[0]}
