# 프론트엔드 구조

> 위치: `frontend/` (monorepo). Next.js 14 App Router.

## 디렉토리 구조

> 아래는 v1 목표 구조다. 초기 스캐폴드는 기본 레이아웃/랜딩 + API 클라이언트로 시작한다.

```
frontend/
├── app/                        # Next.js App Router
│   ├── layout.tsx              # 루트 레이아웃 (QueryProvider, 토스트)
│   ├── page.tsx                # 랜딩 페이지 (로그인하지 않은 사용자)
│   ├── (auth)/
│   │   ├── login/page.tsx
│   │   └── register/page.tsx
│   └── (dashboard)/
│       ├── layout.tsx          # 인증 가드 + 사이드바
│       ├── dashboard/page.tsx  # 메인: URL 입력 + 분석 히스토리
│       └── analyses/
│           └── [id]/page.tsx   # 분석 결과 상세 (폴링)
│
├── src/
│   ├── features/               # 도메인별 모듈
│   │   ├── auth/
│   │   │   ├── types.ts
│   │   │   ├── api/index.ts
│   │   │   └── store.ts        # Zustand: 토큰, 사용자 정보
│   │   └── analysis/
│   │       ├── types.ts
│   │       ├── api/index.ts
│   │       └── hooks/
│   │           └── useAnalysisPolling.ts  # TanStack Query refetchInterval
│   │
│   ├── shared/
│   │   └── api/
│   │       └── client.ts       # apiClient (fetch 래퍼, 토큰 자동 주입)
│   │
│   └── components/
│       ├── ui/                 # shadcn/ui 컴포넌트
│       └── analysis/
│           ├── UrlInput.tsx    # URL 입력 폼 (최대 3개)
│           ├── AnalysisCard.tsx
│           └── InsightReport.tsx
│
└── tests/
    ├── unit/
    └── e2e/
```

---

## 페이지 구성

### 랜딩 (`/`)
- 서비스 소개
- 로그인/회원가입 CTA

### 대시보드 (`/dashboard`)
- URL 입력 폼 (최대 3개 URL, 리뷰 수 설정)
- 분석 시작 버튼 → POST /analyses → /analyses/[id]로 이동
- 최근 분석 히스토리 목록

### 분석 결과 (`/analyses/[id]`)
- 상태 폴링 (2초 간격, TanStack Query `refetchInterval`)
- Pending/Crawling/Analyzing 상태: 진행 중 UI (로딩 바)
- Completed: 인사이트 리포트 표시
  - 고쳐야 할 것 TOP 3
  - 경쟁사 약점
  - 구매 결정 요인
  - 평점 분포 차트
- PDF 내보내기 버튼 (Pro 플랜)

---

## 상태 관리 전략

| 상태 종류 | 도구 | 예시 |
|-----------|------|------|
| 서버 상태 (API 데이터) | TanStack Query | 분석 목록, 분석 결과 |
| 클라이언트 UI 상태 | Zustand | 로그인 토큰, 사용자 정보 |
| 폼 상태 | React 로컬 state | URL 입력 폼 |

### auth store (Zustand)
```typescript
interface AuthStore {
  token: string | null;
  user: User | null;
  setAuth: (token: string, user: User) => void;
  logout: () => void;
}
```

### 분석 폴링 (TanStack Query v5)
`src/features/analysis/hooks/useAnalysisPolling.ts` 로 추출. v5 에서 `refetchInterval`
콜백 인자는 `query` 객체이며 `query.state.data` 로 접근한다. 간격 분기는 순수 함수
`pollInterval(status)` 로 분리해 단위 테스트한다.
```typescript
// completed/failed면 폴링 중단(false), 그 외 2초
export function pollInterval(status?: AnalysisStatus): number | false {
  return status === 'completed' || status === 'failed' ? false : 2000;
}

export function useAnalysisPolling(analysisId: string) {
  return useQuery({
    queryKey: ['analysis', analysisId],
    queryFn: () => getAnalysis(analysisId),
    refetchInterval: (query) => pollInterval(query.state.data?.status),
    enabled: !!analysisId,
  });
}
```

---

## API 통신 패턴

```typescript
// src/shared/api/client.ts
// 토큰 자동 주입 + 에러 처리

// src/features/{domain}/types.ts
// {Action}Request / {Action}Response 인터페이스

// src/features/{domain}/api/index.ts
// export async function actionName(req: ActionRequest): Promise<ActionResponse>
```

**규칙**: 인라인 타입(`Promise<{ field: Type }>`) 금지. 모든 API 함수는 명명된 타입 사용.

---

## 환경 변수

```
NEXT_PUBLIC_API_URL=http://localhost:8080
```
