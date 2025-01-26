#[derive(Debug, serde::Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MarketDataKlineReqBody {
    pub symbol: String,
    pub interval: String,
    pub start_time: Option<u64>,
    pub end_time: Option<u64>,
    pub time_zone: Option<String>,
    pub limit: Option<u64>,
}

impl MarketDataKlineReqBody {
    pub fn default() -> Self {
        Self {
            symbol: "TRUMPUSDC".to_string(),
            interval: "1m".to_string(),
            start_time: None,
            end_time: None,
            time_zone: None,
            limit: None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Endpoint {
    Ping,
    MarketDataKline(MarketDataKlineReqBody, usize),
}

impl Endpoint {
    pub fn as_str(&self) -> &'static str {
        match self {
            Endpoint::Ping => "/api/v3/ping",
            Endpoint::MarketDataKline(_, _) => "/api/v3/klines",
        }
    }
}
