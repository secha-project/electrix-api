#[derive(Clone, Debug)]
pub struct Host {
    url: String,
    token_header: String,
    allow_invalid_certs: bool,
}


impl Host {
    pub fn get_from_env() -> Result<Self, String> {
        const HOST_URL_STR: &str = "HOST_URL";
        const ACCESS_TOKEN_STR: &str = "ACCESS_TOKEN";
        const ALLOW_INVALID_CERTIFICATES: &str = "ALLOW_INVALID_CERTIFICATES";
        const ERROR_MESSAGE: &str = "environment variable is not set";

        let url: String = std::env::var(HOST_URL_STR)
            .map_or_else(
                |_| Err(format!("{HOST_URL_STR} {ERROR_MESSAGE}")),
                Ok
                // |url| Ok(url)
            )?;

        let token_header: String = std::env::var(ACCESS_TOKEN_STR)
            .map_or_else(
                |_| Err(format!("{ACCESS_TOKEN_STR} {ERROR_MESSAGE}")),
                |token| Ok(format!("Api-Key {token}"))
            )?;

        let allow_invalid_certs: bool = std::env::var(ALLOW_INVALID_CERTIFICATES)
            .map_or_else(
                |_| false,
                |value| value == "true"
            );

        Ok(
            Self {
                url,
                token_header,
                allow_invalid_certs,
            }
        )
    }

    pub fn get_url(&self) -> String {
        self.url.clone()
    }

    pub fn get_headers(&self) -> Vec<(String, String)> {
        vec![
            ("Content-Type".to_string(), "application/json".to_string()),
            ("Authorization".to_string(), self.token_header.clone()),
        ]
    }

    pub const fn allow_invalid_certs(&self) -> bool {
        self.allow_invalid_certs
    }
}
