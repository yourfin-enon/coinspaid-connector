#[derive(Clone, Debug)]
pub struct CoinsPaidConfig {
    pub rest_api_host: String,
}

impl Default for CoinsPaidConfig {
    fn default() -> Self {
        Self {
            rest_api_host: "https://app.alphapo.net".into(),
        }
    }
}


impl CoinsPaidConfig {
    pub fn test_env() -> Self {
        Self {
            rest_api_host: "https://app.sandbox.cryptoprocessing.com".into(),
        }
    }
}