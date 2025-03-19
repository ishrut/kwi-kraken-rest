use super::*;
use std::collections::HashMap;

/// RecentSpreads inner field
#[derive(Debug, Serialize, Deserialize)]
pub struct RecentSpreadsData {
    #[serde(rename = "0")]
    pub timestamp: u32,
    #[serde(rename = "1")]
    pub bid: String,
    #[serde(rename = "2")]
    pub ask: String,
}

/// struct to get recent spreads
#[derive(Debug, Serialize, Deserialize)]
pub struct RecentSpreads {
    #[serde(rename = "last")]
    pub last: u32,
    #[serde(flatten)]
    pub recent_trades: HashMap<String, Vec<RecentSpreadsData>>,
}

impl RecentSpreads {
    /// Creates a request for recent spreads and returns a struct containing the data.
    ///
    /// # Parametres
    ///
    /// * `pair` - Asset pair to get data for, e.g SOLEUR
    /// * `since` - Option, Returns spread data since given timestamp. Optional, intended for incremental updates within available dataset (does not contain all historical spreads).
    pub async fn get(pair: &str, since: Option<&str>) -> Result<Self, Error> {
        let url = build_url!(BASE_URL, RECENT_SPREADS_URI; pair; since);
        let response = get_public_json::<RecentSpreads>(&url).await?;
        Ok(response.result)
    }
}
