pub const CURRENCIES: &[Currency] = &[
    Currency {
        id: "BTC",
        symbol: "BTC",
        blockchain: "BTC",
    },
    Currency {
        id: "LTC",
        symbol: "LTC",
        blockchain: "LTC",
    },
    Currency {
        id: "BCH",
        symbol: "BCH",
        blockchain: "BCH",
    },
    Currency {
        id: "ADA",
        symbol: "ADA",
        blockchain: "ADA",
    },
    Currency {
        id: "DOGE",
        symbol: "DOGE",
        blockchain: "DOGE",
    },
    Currency {
        id: "XRP",
        symbol: "XRP",
        blockchain: "XRP",
    }, Currency {
        id: "USDTE",
        symbol: "USDT",
        blockchain: "ERC20",
    }, Currency {
        id: "USDTT",
        symbol: "USDT",
        blockchain: "TRC20",
    }, Currency {
        id: "BNB",
        symbol: "BNB",
        blockchain: "BEP2",
    }, Currency {
        id: "EURS",
        symbol: "EURS",
        blockchain: "EURS",
    }, Currency {
        id: "USDC",
        symbol: "USDC",
        blockchain: "ERC20",
    }, Currency {
        id: "BNB-BSC",
        symbol: "BNB",
        blockchain: "BEP20",
    },
];

pub struct Currency {
    pub id: &'static str,
    pub symbol: &'static str,
    pub blockchain: &'static str,
}