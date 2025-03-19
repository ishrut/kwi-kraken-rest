use super::*;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct OrdersInfo {
    pub closetm: Option<f64>,
    pub cost: String,
    pub descr: OrdersInfoDescr,
    pub expiretm: f64,
    pub fee: String,
    pub limitprice: String,
    pub misc: String,
    pub oflags: String,
    pub opentm: Option<f64>,
    pub price: String,
    pub reason: Option<String>,
    pub refid: Option<String>,
    pub starttm: f64,
    pub status: String,
    pub stopprice: String,
    pub userref: Option<usize>,
    pub vol: String,
    pub vol_exec: String,
    pub cl_ord_id: Option<String>,
    pub trigger: Option<String>,
    pub margin: Option<bool>,
    pub sender_sub_id: Option<String>,
    pub trades: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct OrdersInfoDescr {
    pub close: String,
    pub leverage: String,
    pub order: String,
    pub ordertype: String,
    pub pair: String,
    pub price: String,
    pub price2: String,
    pub r#type: String,
}

impl OrdersInfo {
    /// Creates a request for orders information and returns a struct containing the data.
    ///
    /// # Arguments
    ///
    /// * `trades` - boolean, Whether or not to include trades related to position in output
    /// * `userref` - int32, Restrict results to given user reference id
    /// * `txid` - String, Comma delimited list of transaction IDs to query info about (50 maximum)
    /// * `consolidate_taker` - boolean Default value: true, Whether or not to consolidate trades by individual taker trades
    pub async fn get(
        txid: &str,
        userref: Option<&str>,
        trades: Option<&str>,
        consolidate_taker: Option<&str>,
    ) -> Result<HashMap<String, OrdersInfo>, Error> {
        let body = build_queries!(txid; userref, trades, consolidate_taker);
        let response =
            get_private_json::<HashMap<String, OrdersInfo>>(ORDERS_INFO_URI, Some(&body)).await?;
        Ok(response.result)
    }
}
