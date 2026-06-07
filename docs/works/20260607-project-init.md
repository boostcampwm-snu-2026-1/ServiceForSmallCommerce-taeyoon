# 프로젝트 초기화

- **날짜**: 2026-06-07
- **작업자**: Claude (init-project)

## 작업 요약

Coupang Review AI 프로젝트를 단일 레포(monorepo)로 초기 세팅했다.
백엔드(Rust/Axum) + 프론트엔드(Next.js 14) 스캐폴드 + 문서 + 자동화 훅을 구성하고
컴파일/테스트 통과를 검증했다.

기존 `temp-workspace`의 스펙을 참고하되, 코드는 골격만 새로 생성했다 (기능 구현 제외).

## 주요 결정

- **monorepo**: 프론트/백을 별도 레포로 관리하던 기존 방식 → 하나의 레포로 통합.
  - 폴더명: `backend/`, `frontend/` (prefix 없음)
  - GitHub Actions 워크플로우는 레포 루트 `.github/workflows/`에 path 필터로 분리
- **toolchain 호환성**: 로컬 Node 18.18 기준으로 FE 테스트 스택 핀 고정
  - vitest 4 → `vitest@^2.1.9` (vitest 4는 Node 20+ 필요)
  - jsdom 29 → `jsdom@^24` (29는 ESM-only 전이 의존성으로 Node 18에서 실패)
  - `@vitejs/plugin-react@^4`
- **TestApp 자격증명**: testcontainers Postgres 기본값 `postgres:postgres`로 통일

## 생성된 구조

```
service-for-small-business-taeyoon/
├── CLAUDE.md
├── .github/workflows/      # backend-ci.yml, frontend-ci.yml (path 필터)
├── .claude/                # code-rules, dod, commit-rules, settings.json, hooks/
├── specification/
│   ├── service/overview.md
│   └── dev/{tech-stack,api,backend,frontend,harness}.md
├── docs/
│   ├── works/20260607-project-init.md
│   └── decisions/20260607-initial-tech-stack.md
├── backend/                # Rust + Axum (Hexagonal)
│   ├── src/{domain,application,adapters,http}/
│   ├── tests/helpers/ + health_test.rs
│   ├── migrations/00001_init.sql
│   ├── docker-compose.yml, Makefile, .env.example
└── frontend/               # Next.js 14 + TS + Tailwind
    ├── app/
    ├── src/{features,shared,components}/
    ├── tests/, vitest.config.ts, playwright.config.ts
    └── .env.example
```

## 검증 결과

| 항목 | 명령 | 결과 |
|------|------|------|
| BE 컴파일 | `cargo check --all-targets` | ✅ |
| BE 포맷 | `cargo fmt --check` | ✅ |
| BE 린트 | `cargo clippy --all-targets -- -D warnings` | ✅ |
| BE 통합테스트 | `cargo test --test health_test` (testcontainers) | ✅ 1 passed |
| FE 타입 | `npm run type-check` | ✅ |
| FE 린트 | `npm run lint` | ✅ |
| FE 단위테스트 | `npx vitest run` | ✅ 2 passed |
| FE 커버리지 | `npx vitest run --coverage` | ✅ |

## 다음 단계

1. [ ] `.env` 파일 생성: `cp backend/.env.example backend/.env`, `cp frontend/.env.example frontend/.env.local`
2. [ ] 도메인 모델링: users / analyses 테이블 마이그레이션 작성
3. [ ] 인증 (JWT + argon2) 구현: register/login 엔드포인트
4. [ ] 쿠팡 크롤러 Port + 어댑터 구현
5. [ ] Claude AI 분석 어댑터 구현
6. [ ] FE 인증/대시보드/분석결과 페이지 구현
7. [ ] Playwright 브라우저 설치 후 E2E 작성: `npx playwright install --with-deps chromium`
8. [ ] shadcn/ui 초기화 (필요 시): `npx shadcn@latest init`
