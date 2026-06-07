# 기능 구현 로드맵 (수익 영역 제외)

- **날짜**: 2026-06-07
- **목표**: MVP 기능을 적절한 단위로 분해하여 `/task-with-harness`로 순차 구현. 각 단위 = 1 커밋.

## 제외 범위 (수익 관련)

- 플랜별 사용량 제한 강제 (월 N회, 리뷰 개수 제한, 403 Forbidden 플랜 초과)
- 요금제/결제/빌링
- PDF 내보내기 (Pro 플랜 전용 기능)

> `users.plan` 컬럼은 스키마에 유지하되 기본값 `free`로만 둔다. 제한 로직은 구현하지 않는다.

## 구현 단위 (순차)

| # | 단위 | 영역 | 주요 산출물 | 커밋 |
|---|------|------|-------------|------|
| 1 | BE 인증 | Backend | users 마이그레이션, User 모델, UserRepository(port+pg), AuthService(JWT+argon2), register/login 핸들러 | `feat: 백엔드 인증(회원가입/로그인)` |
| 2 | BE 분석 도메인·어댑터 | Backend | analyses 마이그레이션, Analysis 모델, AnalysisRepository/CoupangCrawler/AiAnalyzer port, pg repo, Mock 크롤러·분석 어댑터 | `feat: 분석 도메인 모델 및 크롤러/AI 어댑터` |
| 3 | BE 분석 서비스·API | Backend | AnalysisService(백그라운드 태스크), 분석 핸들러(POST/GET/목록), /users/me 사용량, 인증 미들웨어, 라우터 연결 | `feat: 분석 요청/조회 API 및 사용량 조회` |
| 4 | FE 인증 | Frontend | auth Zustand store, QueryProvider, 로그인/회원가입 페이지 | `feat: 프론트 인증 페이지 및 상태관리` |
| 5 | FE 대시보드 | Frontend | 인증 가드 레이아웃, 대시보드(URL 입력 + 히스토리), UrlInput/AnalysisCard | `feat: 프론트 대시보드 및 분석 요청 UI` |
| 6 | FE 분석 결과 | Frontend | useAnalysisPolling 훅, 분석 결과 페이지, InsightReport | `feat: 프론트 분석 결과 리포트 및 폴링` |

## 의존 관계

- 3 ← 1, 2 (인증·도메인 선행)
- 4 ← 1 (BE 인증 API)
- 5 ← 3, 4
- 6 ← 3, 4

## 교체 가능성 설계 (trait)

| 대상 | trait | 기본 구현 |
|------|-------|-----------|
| 사용자 저장소 | `UserRepository` | `PgUserRepository` |
| 분석 저장소 | `AnalysisRepository` | `PgAnalysisRepository` |
| 리뷰 크롤러 | `CoupangCrawler` | `HttpCoupangCrawler` (+ 테스트용 Mock) |
| AI 분석 | `AiAnalyzer` | `ClaudeAiAnalyzer` (+ 테스트용 Mock) |
| 인증 서비스 | `AuthService` | `StandardAuthService` |
| 분석 서비스 | `AnalysisService` | `StandardAnalysisService` |
