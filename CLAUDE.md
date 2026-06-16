# Coupang Review AI - 프로젝트 가이드

## 프로젝트 개요

쿠팡 셀러가 경쟁 상품 URL을 입력하면, AI가 리뷰를 분석해 "이걸 고치면 경쟁사를 이긴다"는 인사이트를 제공하는 SaaS 도구.

백엔드(Rust)와 프론트엔드(Next.js)를 **하나의 레포(monorepo)** 로 관리한다.

---

## 작업 시작 전 반드시 읽을 것

새 세션 또는 서브에이전트 시작 시 순서대로 읽어라:

1. 이 파일 (`CLAUDE.md`) — 전체 맥락
2. `.claude/github-workflow.md` — **이슈 → 작업 → PR → Wiki** 작업 흐름 (필수)
3. 관련 **GitHub Issue / PR** — 최근 작업 맥락 (`gh issue list`, `gh pr list`)
4. **GitHub Wiki** — 현재 서비스/기술 스펙의 최종 현황 (canonical)
5. 필요 시 `specification/` 스냅샷 참고 (최신본은 Wiki)

---

## 작업 흐름 (GitHub Issue → 작업 → PR → Wiki)

문서화는 **GitHub Issue / PR / Wiki를 적극 활용**한다. 모든 작업은 아래 4단계를 따른다.

```
1. 이슈 등록     gh issue create — 작업 단위를 GitHub Issue로 (= 기존 docs/plans)
2. 작업          /task-with-harness — 이슈 기반 브랜치에서 구현 + harness 검증
3. PR            gh pr create — "Closes #N", PR 본문이 작업 로그 (= 기존 docs/works)
4. Wiki 업데이트  스펙 변경분을 GitHub Wiki(최종 현황, canonical)에 반영
```

상세 절차: **`.claude/github-workflow.md`** (gh 명령어, 브랜치 규칙, PR/Wiki 템플릿)

### 문서가 사는 곳

| 문서 종류 | 위치 | 비고 |
|-----------|------|------|
| **스펙 (살아있는 현황)** | **GitHub Wiki** | canonical. 서비스/기술 스펙의 최종 현황 |
| **작업 플랜** | **GitHub Issue** | 작업 시작 전 등록 (기존 `docs/plans` 대체) |
| **작업 로그** | **GitHub PR** | `Closes #N` + PR 본문 (기존 `docs/works` 대체) |
| 의사결정 (ADR) | **GitHub Wiki / PR** | 설계 판단은 PR 본문 + 해당 Wiki 페이지에 기록 |
| 세션 로그 | **GitHub PR / Issue** | 작업 맥락은 PR/이슈가 담당 |

> `specification/` 디렉터리는 Wiki 이관 이전의 스냅샷이다. **스펙의 최신본은 항상 Wiki를 본다.**
> **로컬 `docs/` 는 더 이상 쌓지 않는다.** 모든 문서는 GitHub Issue/PR/Wiki가 canonical이며,
> `docs/` 는 `.gitignore` 처리되어 커밋되지 않는다.

### 레포 디렉터리

```
service-for-small-business-taeyoon/
├── specification/   # Wiki 이관 전 스냅샷 (최신본은 Wiki)
├── backend/         # Rust 백엔드 코드
├── frontend/        # Next.js 프론트엔드 코드
├── .claude/         # 워크플로우 규칙 + 훅
└── CLAUDE.md        # 이 파일
```

---

## 기술 스택

| 영역 | 스택 |
|------|------|
| 백엔드 | Rust + Axum + Tokio + sqlx + PostgreSQL |
| 프론트엔드 | Next.js 14 (TypeScript) + Tailwind + Zustand + TanStack Query |
| 테스트 | cargo test + testcontainers + Vitest + Playwright + GitHub Actions CI |

→ 상세 및 결정 이유: Wiki `Tech-Stack` 페이지

---

## 워크플로우 규칙

상세 내용은 아래 파일 참조:

| 규칙 | 파일 |
|------|------|
| **GitHub 작업 흐름 (이슈/PR/Wiki)** | `.claude/github-workflow.md` |
| 코드 수정 규칙 | `.claude/code-rules.md` |
| 작업 완료 기준 (DoD) | `.claude/dod.md` |
| 커밋 규칙 | `.claude/commit-rules.md` |
| 테스트 안전망 | Wiki: `Harness` 페이지 (`specification/dev/harness.md` 스냅샷) |

---

## 빠른 명령어

```bash
# 백엔드
cd backend && cargo check && cargo test

# 프론트엔드
cd frontend && npm run type-check && npm test -- --run
```

---

## 세션 마무리

세션 마무리 시 `/wrap` 스킬 실행:
- 진행한 작업의 **PR 본문/이슈 코멘트** 정리 + 머지된 변경분 **Wiki 반영**
- 변경된 스펙은 GitHub **Wiki** 해당 페이지 업데이트
- 의사결정(ADR)은 PR 본문 + 해당 **Wiki** 페이지에 기록 (로컬 `docs/` 미사용)
- CLAUDE.md 업데이트
