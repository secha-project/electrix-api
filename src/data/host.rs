use crate::utils::environ::{get_env, get_env_or_default};

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

        let url: String = get_env(HOST_URL_STR)?;
        let token: String = get_env::<String>(ACCESS_TOKEN_STR)?;
        let allow_invalid_certs: bool = get_env_or_default(ALLOW_INVALID_CERTIFICATES, &false);

        Ok(
            Self {
                url,
                token_header: format!("Api-Key {token}"),
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
