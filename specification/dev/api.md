# API 엔드포인트 설계

Base URL: `/api/v1`

---

## 인증

### POST /auth/register
회원가입

**Request**
```json
{
  "email": "string",
  "password": "string"
}
```

**Response 201**
```json
{
  "token": "string",
  "user": {
    "id": "uuid",
    "email": "string",
    "plan": "free | starter | pro",
    "created_at": "datetime"
  }
}
```

---

### POST /auth/login
로그인

**Request**
```json
{
  "email": "string",
  "password": "string"
}
```

**Response 200**
```json
{
  "token": "string",
  "user": {
    "id": "uuid",
    "email": "string",
    "plan": "free | starter | pro"
  }
}
```

---

## 분석

### POST /analyses
분석 요청 생성 (비동기 처리)

**Request**
```json
{
  "urls": ["string"],   // 쿠팡 상품 URL, 최대 3개
  "review_limit": 100   // 50 ~ 500, 플랜별 제한
}
```

**Response 202**
```json
{
  "analysis_id": "uuid",
  "status": "pending",
  "created_at": "datetime"
}
```

---

### GET /analyses/:id
분석 상태 및 결과 조회 (프론트엔드 폴링용)

**Response 200**
```json
{
  "id": "uuid",
  "status": "pending | crawling | analyzing | completed | failed",
  "urls": ["string"],
  "result": null,           // status가 completed일 때만 존재
  "error": null,            // status가 failed일 때만 존재
  "created_at": "datetime",
  "completed_at": "datetime | null"
}
```

**result 구조** (status = completed)
```json
{
  "products": [
    {
      "url": "string",
      "product_name": "string",
      "total_reviews": 100,
      "avg_rating": 4.2,
      "rating_distribution": {
        "1": 3, "2": 5, "3": 10, "4": 30, "5": 52
      }
    }
  ],
  "insights": {
    "top_complaints": [
      { "text": "배송 포장이 허술하다", "count": 37, "severity": "high" }
    ],
    "top_positives": [
      { "text": "가성비가 좋다", "count": 52 }
    ],
    "improvement_points": [
      { "rank": 1, "title": "포장 강화", "detail": "..." }
    ],
    "competitor_weaknesses": [
      { "title": "AS 응답 느림", "opportunity": "..." }
    ],
    "purchase_drivers": ["string"]
  }
}
```

---

### GET /analyses
분석 히스토리 목록

**Query**
- `page`: number (기본값 1)
- `per_page`: number (기본값 20)

**Response 200**
```json
{
  "analyses": [
    {
      "id": "uuid",
      "status": "string",
      "urls": ["string"],
      "created_at": "datetime"
    }
  ],
  "total": 100,
  "page": 1,
  "per_page": 20
}
```

---

### GET /analyses/:id/export/pdf
PDF 내보내기 (Pro 플랜 전용)

**Response 200** `Content-Type: application/pdf`

---

## 사용자

### GET /users/me
현재 사용자 정보 (플랜, 이번 달 사용량)

**Response 200**
```json
{
  "id": "uuid",
  "email": "string",
  "plan": "free | starter | pro",
  "usage": {
    "analyses_this_month": 3,
    "analyses_limit": null
  }
}
```

> `usage.analyses_limit` 은 `number | null`. 현재는 항상 `null` = 제한 없음(플랜별 제한 강제는 수익 기능으로 보류).

---

## 공통

### GET /health
헬스체크

**Response 200**
```json
{ "status": "ok" }
```

---

## 에러 형식

```json
{
  "error": "string"
}
```

| HTTP 코드 | 의미 |
|-----------|------|
| 400 | Bad Request (잘못된 입력) |
| 401 | Unauthorized (토큰 없음/만료) |
| 403 | Forbidden (플랜 초과) |
| 404 | Not Found |
| 500 | Internal Server Error |
