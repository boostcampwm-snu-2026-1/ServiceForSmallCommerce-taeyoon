use anyhow::Result;

#[derive(Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_expires_in: u64,
    pub server_port: u16,
    /// Gemini API 키. 없으면 Mock 분석기로 폴백한다.
    pub gemini_api_key: Option<String>,
    /// Gemini 모델명. 기본값 "gemini-2.5-flash"(키 free tier 에서 호출 가능).
    pub gemini_model: String,
    /// 크롤러 모드. "http"(기본, 실제 쿠팡 호출) | "mock"(픽스처 리뷰).
    pub crawler_mode: String,
    /// 스크래핑 API 프록시 URL 템플릿. `{url}` 플레이스홀더에 타겟 URL 이 치환된다.
    /// 설정 시 모든 크롤링 요청이 이 프록시를 경유한다(anti-bot/IP 로테이션은 벤더가 처리).
    pub coupang_proxy_url: Option<String>,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            database_url: std::env::var("DATABASE_URL")?,
            jwt_secret: std::env::var("JWT_SECRET")?,
            jwt_expires_in: std::env::var("JWT_EXPIRES_IN")
                .unwrap_or_else(|_| "86400".to_string())
                .parse()?,
            server_port: std::env::var("SERVER_PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()?,
            gemini_api_key: std::env::var("GEMINI_API_KEY").ok(),
            gemini_model: std::env::var("GEMINI_MODEL")
                .unwrap_or_else(|_| "gemini-2.5-flash".to_string()),
            crawler_mode: std::env::var("CRAWLER_MODE").unwrap_or_else(|_| "http".to_string()),
            coupang_proxy_url: std::env::var("COUPANG_PROXY_URL")
                .ok()
                .filter(|s| !s.trim().is_empty()),
        })
    }
}
