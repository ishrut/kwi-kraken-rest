use super::*;
use std::collections::HashMap;

/// RecentTrades inner field
#[derive(Debug, Serialize, Deserialize)]
pub struct RecentTradesFields {
    #[serde(rename = "0")]
    pub price: String,
    #[serde(rename = "1")]
    pub volume: String,
    #[serde(rename = "2")]
    pub timestamp: f64,
    #[serde(rename = "3")]
    pub transaction_type: String,
    #[serde(rename = "4")]
    pub order_type: String,
    #[serde(rename = "5")]
    pub misc: String,
    #[serde(rename = "6")]
    pub trade_id: u32,
}

/// struct to get recent trades
#[derive(Debug, Serialize, Deserialize)]
pub struct RecentTrades {
    #[serde(rename = "last")]
    pub last: String,
    #[serde(flatten)]
    pub recent_trades: HashMap<String, Vec<RecentTradesFields>>,
}

impl RecentTrades {
    /// Creates a request for recent trades and returns a struct containing the data.
    ///
    /// # Parametres
    ///
    /// * `pair` - asset pair to get data for
    /// * `since` - Option, Return trade data since given timestamp.
    /// * `count` - Option, default 1000, Return specific number of trades, up to 1000
    pub async fn get(pair: &str, since: Option<&str>, count: Option<&str>) -> Result<Self, Error> {
        let url = build_url!(BASE_URL, RECENT_TRADES_URI; pair; since, count);
        let response = get_public_json::<Self>(&url).await?;
        Ok(response.result)
    }
}
