use std::collections::HashMap;

use super::*;

/// struct to get trades info
#[derive(Debug, Deserialize, Serialize)]
pub struct TradesInfo {
    pub ordertxid: String,
    pub postxid: String,
    pub pair: String,
    pub time: f64,
    #[serde(rename = "type")]
    pub trade_type: String,
    pub ordertype: String,
    pub price: String,
    pub cost: String,
    pub fee: String,
    pub vol: String,
    pub margin: String,
    pub misc: String,
    pub trade_id: usize,
    pub maker: bool,
}

impl TradesInfo {
    /// Creates a request for trades info and returns a struct containing the data.
    ///
    /// # Arguments
    /// * `txid` - string, Comma delimited list of transaction IDs to query info about (20 maximum)
    /// * `trades` - boolean, Whether or not to include trades related to position in output
    pub async fn get(
        txid: &str,
        trades: Option<&str>,
    ) -> Result<HashMap<String, TradesInfo>, Error> {
        let queries = build_queries!(txid ; trades);
        let response =
            get_private_json::<HashMap<String, TradesInfo>>(TRADES_INFO_URI, Some(&queries))
                .await?;
        Ok(response.result)
    }
}
