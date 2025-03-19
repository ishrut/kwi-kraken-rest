use super::*;
use std::collections::HashMap;

/// struct to query ledgers
#[derive(Debug, Serialize, Deserialize)]
pub struct QueryLedgers {
    pub refid: String,
    pub time: f64,
    #[serde(rename = "type")]
    pub ledger_type: String,
    pub subtype: String,
    pub aclass: String,
    pub asset: String,
    pub amount: String,
    pub fee: String,
    pub balance: String,
}

impl QueryLedgers {
    /// Creates a request for ledgers using the id and returns a struct containing the data.
    ///
    /// # Arguments
    ///
    /// * `id` - string, required, Comma delimited list of ledger IDs to query info about (20 maximum)
    /// * `trades` - boolean, Whether or not to include trades related to position in output
    pub async fn get(
        id: Vec<&str>,
        trades: Option<&str>,
    ) -> Result<HashMap<String, QueryLedgers>, Error> {
        let id = id.join(",");
        let queries = build_queries!(id; trades);
        let resp =
            get_private_json::<HashMap<String, QueryLedgers>>(QUERY_LEDGERS_URI, Some(&queries))
                .await?;
        Ok(resp.result)
    }
}
