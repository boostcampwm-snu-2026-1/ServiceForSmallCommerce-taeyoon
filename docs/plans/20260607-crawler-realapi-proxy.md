# 쿠팡 크롤러: 실제 next-api/review + 프록시 + 에러전파 개선 계획

- **날짜**: 2026-06-07
- **목표**: 라이브 쿠팡 크롤링이 데이터센터 IP에서 Akamai 403 으로 막히는 문제에 대해, (1) 올바른 리뷰 API(`next-api/review`)와 완전한 브라우저 헤더로 크롤러를 제대로 구현하고, (2) 스크래핑 API 프록시 패스스루를 env 로 지원하며, (3) 실패 원인이 보이도록 에러 전파를 개선하고, (4) 로컬 데모용 `CRAWLER_MODE` 토글을 추가한다.

## 진단 (검증)

- `next-api/review`, 구 `/vp/product/reviews`, 상품 HTML 페이지 모두 데이터센터 IP에서 **403 Akamai**(Reference #18...). 헤더·Referer·sec-fetch·쿠키 선취득 모두 무효.
- 결론: 서버 IP 차단이라 헤더 트릭으로 우회 불가. 안전·현실적 경로는 (a) 가정용 IP/헤드리스 환경에서 실행 또는 (b) 스크래핑 API(잔여 anti-bot/IP 로테이션을 벤더가 처리) 사용.
- anti-bot sensor 스푸핑/프록시 로테이션 자체 구현은 하지 않음(취약 + ToS/법적 리스크 + 회피기술).

## 서브태스크 (단일 BE 단위, 원자적)

| # | 작업 | 대상 파일 |
|---|------|-----------|
| 1 | `AppError::detail()` 추가 — Internal/Database 의 내부 원인 문자열 노출 | `backend/src/error.rs` |
| 2 | Config 에 `crawler_mode`, `coupang_proxy_url` 추가 | `backend/src/config.rs`, `backend/tests/helpers/app.rs` |
| 3 | `HttpCoupangCrawler`: next-api/review 엔드포인트 + 풀 브라우저 헤더 + 프록시 패스스루 + 상태코드 검사(설명적 에러) + 파싱 보강(재귀 탐색) | `backend/src/adapters/coupang/crawler.rs` |
| 4 | 파이프라인 에러를 `e.detail()` 로 저장 + `tracing::error!` 로깅 | `backend/src/application/analysis_service.rs` |
| 5 | main.rs 와이어링(mode + proxy) | `backend/src/main.rs` |
| 6 | env/스펙 동기화 | `backend/.env`, `backend/.env.example`, `specification/dev/backend.md` |

## 설계

- **프록시 패스스루**: `COUPANG_PROXY_URL` 가 `{url}` 플레이스홀더를 포함하면, 타겟 URL 을 URL-encode 해 치환하여 그 주소로 GET. 벤더 무관(ScraperAPI/ZenRows 등). 미설정 시 직접 호출.
- **CRAWLER_MODE**: `http`(기본) | `mock`. mock 이면 MockCoupangCrawler 주입(데모: 실제 Gemini + 픽스처 리뷰).
- **에러 전파**: `AppError::Internal`/`Database` 는 Display 가 고정이라 원인이 가려짐 → `detail()` 로 내부 anyhow(`{:#}`)/sqlx 메시지를 추출해 DB `error` 컬럼에 저장 + 서버 로그.
- **파싱**: next-api/review 응답 스키마 비공개 → 알려진 키 우선 + 객체/배열 재귀 탐색으로 리뷰 배열 발견(방어적). 단위 테스트로 알려진 형태 고정.

## 테스트 전략

- crawler 단위 테스트: 기존 + 재귀 탐색/프록시 URL 빌드 케이스 추가.
- `cargo check` + `cargo test` 전체 통과.
- 한계 명시: 서버 IP 직접 호출은 여전히 403 예상(프록시 키 또는 가정용 IP 필요). mock 모드로 전체 플로우 즉시 검증 가능.
