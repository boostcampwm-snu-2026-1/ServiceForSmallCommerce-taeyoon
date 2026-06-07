# ADR: 초기 기술 스택 선택

- **날짜**: 2026-06-07
- **상태**: Accepted

## 컨텍스트

Coupang Review AI 프로젝트의 초기 기술 스택과 리포지토리 구조를 결정한다.

## 결정

### 리포지토리: 단일 레포 (monorepo)

- 백엔드/프론트엔드를 `backend/`, `frontend/` 디렉토리로 하나의 git 레포에서 관리.
- 이전에는 별도 레포로 관리했으나, 단일 과제 레포(`boostcampwm-snu-2026-1`)에 맞춰 통합.
- CI는 레포 루트 `.github/workflows/`에 두고 `paths` 필터로 BE/FE 잡을 분리.

### 백엔드: Rust + Axum + sqlx + PostgreSQL

- **Rust**: 컴파일 타임 안전성. `Result<T, E>` 강제로 에러 처리 누락 불가. AI Agent 코드 수정 시 컴파일 단계에서 버그 차단.
- **Axum**: Tokio 기반 비동기. 타입 안전 라우터. 크롤러 + AI API 호출 같은 I/O 중심 워크로드에 최적.
- **sqlx**: 컴파일 타임 SQL 검증. 런타임 ORM 오버헤드 없음.
- **PostgreSQL**: 트랜잭션, JSONB(분석 결과 저장), uuid 네이티브 지원.

### 아키텍처: Hexagonal (Ports & Adapters)

- 외부 의존성(크롤러, AI, DB)을 Port trait으로 격리.
- 쿠팡 API 변경 / 네이버 추가 시 어댑터만 교체.
- 테스트 시 Mock 어댑터 주입 가능.

### 프론트엔드: Next.js 14 + TypeScript + Tailwind + Zustand + TanStack Query

- **Next.js App Router**: 서버 컴포넌트로 초기 로딩 최적화, 랜딩 SEO.
- **TypeScript**: 프론트-백 타입 계약 명시적 관리.
- **Zustand**: 클라이언트 UI 상태(토큰, 사용자) 전용, 보일러플레이트 최소.
- **TanStack Query**: 서버 상태 + 분석 완료 폴링(`refetchInterval`) 전용.

### 테스트: testcontainers + TestApp + Vitest + Playwright

- **testcontainers**: `cargo test`만으로 격리 PostgreSQL 자동 관리. 로컬 DB 설치 불필요. 프로세스당 컨테이너 1개 공유.
- **TestApp**: 격리 test_<uuid> DB + 랜덤 포트로 병렬 테스트 가능.
- **Vitest / Playwright**: 단위 + E2E 커버.

## 로컬 환경 제약 (Node 18.18) 으로 인한 버전 핀

- `vitest@^2.1.9` — vitest 4는 Node 20+ 필요(`node:util` styleText).
- `jsdom@^24` — jsdom 29는 ESM-only 전이 의존성으로 Node 18에서 require 실패.
- `@vitejs/plugin-react@^4`.
- CI는 Node 20을 사용하므로, 추후 로컬 Node 업그레이드 시 최신 버전으로 재핀 가능.

## 결과

- AI Agent가 코드를 수정할 때 컴파일 + 테스트 통과를 강제 검증 가능.
- 외부 API(쿠팡, Claude) 의존성을 어댑터로 격리해 리스크 통제.
- 단일 레포로 BE/FE 변경을 한 PR/커밋 단위로 묶어 관리.
