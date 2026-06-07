# 쿠팡 크롤러: 실제 next-api/review + 프록시 + 에러전파 개선

- **날짜**: 2026-06-07
- **플랜**: docs/plans/20260607-crawler-realapi-proxy.md

## 작업 요약

분석 실패("Internal server error")의 원인이 라이브 쿠팡의 Akamai 403(데이터센터 IP 차단)임을 검증하고, 크롤러를 실제 리뷰 API(`next-api/review`) + 완전한 브라우저 헤더로 재구현했다. 추가로 (1) 스크래핑 API 프록시 패스스루(env), (2) 실패 원인 노출(에러 전파), (3) 로컬 데모용 `CRAWLER_MODE` 토글을 도입했다.

자체 anti-bot sensor 스푸핑/프록시 로테이션은 구현하지 않음(취약 + ToS/법적 리스크). 안정적 수집은 스크래핑 API 프록시 또는 가정용 IP/헤드리스 환경으로 위임한다.

## 진단 (검증)

- `next-api/review`, 구 `/vp/product/reviews`, 상품 HTML 페이지 모두 데이터센터 IP에서 **403 Akamai**(Reference #18...). UA/Referer/sec-fetch/쿠키 선취득 모두 무효 → IP 단 차단 확정.

## 변경 내용

### 1. `backend/src/error.rs`
- `AppError::detail()` 추가: `Internal`/`Database` 의 내부 원인(anyhow `{:#}`/sqlx)을 노출. 사용자 응답(`Display`)은 그대로 generic 유지.

### 2. `backend/src/adapters/coupang/crawler.rs`
- `HttpCoupangCrawler::new(client, proxy_url: Option<String>)` 로 시그니처 변경.
- `build_review_api_url`: `next-api/review?...&sortBy=ORDER_SCORE_ASC&ratingSummary=true` 엔드포인트.
- `resolve_request_url`: 프록시 템플릿 `{url}` 치환(없으면 `&url=` 덧붙임), 의존성 없는 `percent_encode`.
- `fetch_reviews`: 풀 브라우저 헤더(UA/Referer/Accept/Accept-Language/sec-fetch-*), **status 검사 → 실패 시 HTTP 코드+본문 스니펫을 에러로**, JSON 파싱 실패도 스니펫 포함.
- `parse_reviews`: 알려진 키 확장 + `find_reviews_deep` 재귀 탐색. `find_product_name` 깊이 탐색.
- 단위 테스트 6종 추가(next-api URL, 프록시 치환/덧붙임/패스스루, 재귀 파싱, 상품명).

### 3. `backend/src/config.rs` (+ `tests/helpers/app.rs`)
- `crawler_mode`(env `CRAWLER_MODE`, 기본 "http"), `coupang_proxy_url`(env `COUPANG_PROXY_URL`).

### 4. `backend/src/application/analysis_service.rs`
- 파이프라인 실패 시 `e.detail()` 저장 + 크롤링/AI 분석 실패에 `tracing::error!` 로깅.

### 5. `backend/src/main.rs`
- `CRAWLER_MODE` 에 따라 Mock/Http 크롤러 선택, Http 에 프록시 URL 주입.

### 6. env / 스펙
- `.env`/`.env.example`: `CRAWLER_MODE`, `COUPANG_PROXY_URL`(예시 주석).
- `specification/dev/backend.md`: 크롤러 전략 + anti-bot 한계/운영 방식 섹션 갱신.

## 검증

- `cargo test`: lib 단위 **39**(+6 크롤러) + 통합(analysis 4 / auth 5 / health 1) 전부 통과.
- **E2E 라이브 검증**: `CRAWLER_MODE=mock` + 실제 Gemini 2.5-flash 로 분석 생성 → `analyzing → completed`. `improvement_points` 가 실제 Gemini 생성 한국어 인사이트(템플릿 아님) 확인.

## 한계 / 운영 메모

- 데이터센터(서버) IP 직접 호출은 여전히 403 예상 → 실제 수집엔 `COUPANG_PROXY_URL`(스크래핑 API 키) 또는 가정용 IP 필요.
- 로컬 전체 플로우 데모는 `CRAWLER_MODE=mock` + 실제 Gemini 권장.

## 커밋

- <commit-id> (루트 단일 커밋, `backend/.env` 제외)
