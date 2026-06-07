# Coupang Review AI - 프로젝트 가이드

## 프로젝트 개요

쿠팡 셀러가 경쟁 상품 URL을 입력하면, AI가 리뷰를 분석해 "이걸 고치면 경쟁사를 이긴다"는 인사이트를 제공하는 SaaS 도구.

백엔드(Rust)와 프론트엔드(Next.js)를 **하나의 레포(monorepo)** 로 관리한다.

---

## 작업 시작 전 반드시 읽을 것

새 세션 또는 서브에이전트 시작 시 순서대로 읽어라:

1. 이 파일 (`CLAUDE.md`) — 전체 맥락
2. `docs/works/` 최신 파일 — 최근 작업 내용 및 레퍼런스
3. `specification/service/` — 현재 서비스 스펙
4. `specification/dev/` 해당 영역 — 기술 구현 스펙
5. 필요 시 `docs/decisions/` 세부 내용 참고

---

## 문서 구조

```
service-for-small-business-taeyoon/
│
├── specification/              # 살아있는 스펙 (항상 최신 상태)
│   ├── service/                # 서비스 스펙 (사용자/기능 관점)
│   │   └── overview.md
│   └── dev/                    # 개발 스펙 (기술 구현)
│       ├── tech-stack.md
│       ├── api.md
│       ├── backend.md
│       ├── frontend.md
│       └── harness.md
│
├── docs/                       # 작업 기록 (축적형, 삭제 금지)
│   ├── works/                  # 작업 완료 로그 (YYYYMMDD-slug.md)
│   ├── plans/                  # 작업 시작 전 플랜 (YYYYMMDD-slug.md)
│   ├── decisions/              # 의사결정 기록 (ADR)
│   └── sessions/               # 세션 종료 시 자동 생성 로그 (YYYYMMDD-HHMMSS.md)
│
├── backend/                    # Rust 백엔드 코드
├── frontend/                   # Next.js 프론트엔드 코드
└── CLAUDE.md                   # 이 파일
```

### 문서 작성 규칙

| 문서 종류 | 위치 | 언제 작성 |
|-----------|------|-----------|
| 스펙 변경 | `specification/` | 기능/설계가 바뀔 때마다 업데이트 |
| 작업 플랜 | `docs/plans/YYYYMMDD-slug.md` | 작업 시작 전 (훅이 강제함) |
| 작업 로그 | `docs/works/YYYYMMDD-slug.md` | 작업 단위 완료 시 |
| 의사결정 | `docs/decisions/YYYYMMDD-slug.md` | 중요한 기술 결정 시 |
| 세션 로그 | `docs/sessions/YYYYMMDD-HHMMSS.md` | 세션 종료 시 자동 생성 |

**모든 문서는 삭제하지 않는다.** 스펙 파일은 업데이트 가능.

**`docs/` 하위 문서는 WAL(Write-Ahead Log) 방식으로 append-only.**

---

## 기술 스택

| 영역 | 스택 |
|------|------|
| 백엔드 | Rust + Axum + Tokio + sqlx + PostgreSQL |
| 프론트엔드 | Next.js 14 (TypeScript) + Tailwind + Zustand + TanStack Query |
| 테스트 | cargo test + testcontainers + Vitest + Playwright + GitHub Actions CI |

→ 결정 이유: `docs/decisions/20260607-initial-tech-stack.md`

---

## 워크플로우 규칙

상세 내용은 아래 파일 참조:

| 규칙 | 파일 |
|------|------|
| 코드 수정 규칙 | `.claude/code-rules.md` |
| 작업 완료 기준 (DoD) | `.claude/dod.md` |
| 커밋 규칙 | `.claude/commit-rules.md` |
| 테스트 안전망 | `specification/dev/harness.md` |

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
- 변경된 스펙 문서 업데이트
- `docs/works/YYYYMMDD-summary.md` 작성
- 필요 시 ADR 작성 (`docs/decisions/`)
- CLAUDE.md 업데이트
