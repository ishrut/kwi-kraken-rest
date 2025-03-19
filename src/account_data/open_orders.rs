use super::*;
use std::collections::HashMap;

/// struct to get open orders
#[derive(Debug, Deserialize)]
pub struct OpenOrders {
    pub open: HashMap<String, OpenOrdersData>,
}

/// OpenOrders inner field
#[derive(Debug, Deserialize, Clone)]
pub struct OpenOrdersData {
    pub refid: Option<String>,
    pub userref: Option<i32>,
    pub cl_ord_id: Option<String>,
    pub status: String,
    pub opentm: f64,
    pub starttm: i32,
    pub expiretm: i32,
    pub descr: OpenOrdersDescription,
    pub vol: String,
    pub vol_exec: String,
    pub cost: String,
    pub fee: String,
    pub price: String,
    pub stopprice: String,
    pub limitprice: String,
    pub misc: String,
    pub oflags: String,
}

/// OpenOrdersData inner field
#[derive(Debug, Deserialize, Clone)]
pub struct OpenOrdersDescription {
    pub pair: String,
    #[serde(rename = "type")]
    pub order_type: String,
    pub ordertype: String,
    pub price: String,
    pub price2: String,
    pub leverage: String,
    pub order: String,
    pub close: String,
}

impl OpenOrders {
    /// Fetches open orders data from kraken server and returns a struct.
    ///
    /// # Arguments
    ///
    /// * `trades` - boolean, Whether or not to include trades related to position in output
    /// * `userref` - int32, Restrict results to given user reference
    /// * `cl_ord_id` - string, Restrict results to given client order id
    pub async fn get(
        trades: Option<&str>,
        userref: Option<&str>,
        cl_ord_id: Option<&str>,
    ) -> Result<Self, Error> {
        let queries = build_queries!( ; trades, userref, cl_ord_id);
        let resp = get_private_json::<Self>(OPEN_ORDERS_URI, Some(&queries)).await?;
        Ok(resp.result)
    }
}
