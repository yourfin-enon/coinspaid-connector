pub enum CoinsPaidEndpoint {
    TakeAddress,
    CryptoWithdrawal,
    Ping,
    AccountBalances,
}

impl From<CoinsPaidEndpoint> for String {
    fn from(item: CoinsPaidEndpoint) -> Self {
        String::from(match item {
            CoinsPaidEndpoint::Ping => "/api/v2/ping",
            CoinsPaidEndpoint::TakeAddress => "/api/v2/addresses/take",
            CoinsPaidEndpoint::CryptoWithdrawal => "/api/v2/withdrawal/crypto",
            CoinsPaidEndpoint::AccountBalances => "/api/v2/accounts/list",
        })
    }
}
