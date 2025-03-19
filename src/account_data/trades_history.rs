use super::*;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct TradesHistory {
    pub count: u32,
    pub trades: HashMap<String, TradesHistoryFields>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TradesHistoryFields {
    pub cost: String,
    pub fee: String,
    pub leverage: String,
    pub maker: bool,
    pub margin: String,
    pub misc: String,
    pub ordertxid: String,
    pub ordertype: String,
    pub pair: String,
    pub postxid: String,
    pub price: String,
    pub time: f64,
    pub trade_id: u64,
    #[serde(rename = "type")]
    pub order_type: String,
    pub vol: String,
}

impl TradesHistory {
    /// Creates a request for trades history and returns a struct containing the data.
    ///
    /// # Arguments
    /// * `type` - string, Possible values: [all, any position, closed position, closing position, no position],Default value: all, Type of trade
    /// * `trades` - boolean, Whether or not to include trades related to position in output
    /// * `start` - integer, Starting unix timestamp or trade tx ID of results (exclusive)
    /// * `end` - integer, Ending unix timestamp or trade tx ID of results (inclusive)
    /// * `ofs` - integer, Result offset for pagination
    /// * `consolidate_taker` - boolean, Default value: true, Whether or not to consolidate trades by individual taker trades
    /// * `ledgers` - boolean, Whether or not to include related ledger ids for given trade, Note that setting this to true will slow request performance
    pub async fn get(
        position_type: Option<&str>,
        trades: Option<bool>,
        start: Option<usize>,
        end: Option<usize>,
        ofs: Option<usize>,
        consolidate_taker: Option<bool>,
        ledgers: Option<bool>,
    ) -> Result<Self, Error> {
        let queries = build_queries!( ; trades, start, end, ofs, consolidate_taker, ledgers);
        let body = match position_type {
            Some(val) => format!("{}&type={}", &queries, val),
            None => queries,
        };
        let response = get_private_json::<Self>(TRADES_HISTORY_URI, Some(&body)).await?;
        Ok(response.result)
    }
}
