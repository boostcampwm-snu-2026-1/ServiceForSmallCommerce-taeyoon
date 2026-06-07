use anyhow::Result;

#[derive(Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_expires_in: u64,
    pub server_port: u16,
    /// Claude API 키. 없으면 Mock 분석기로 폴백한다.
    pub claude_api_key: Option<String>,
    /// Claude 모델명. 기본값 "claude-sonnet-4-6".
    pub claude_model: String,
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
            claude_api_key: std::env::var("CLAUDE_API_KEY").ok(),
            claude_model: std::env::var("CLAUDE_MODEL")
                .unwrap_or_else(|_| "claude-sonnet-4-6".to_string()),
        })
    }
}
