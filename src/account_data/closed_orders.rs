use super::*;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClosedOrders {
    pub closed: HashMap<String, ClosedOrdersData>,
    pub count: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClosedOrdersData {
    pub closetm: Option<f64>,
    pub cost: Option<String>,
    pub descr: ClosedOrdersDescr,
    pub expiretm: Option<f64>,
    pub fee: String,
    pub limitprice: String,
    pub misc: String,
    pub oflags: String,
    pub opentm: Option<f64>,
    pub price: String,
    pub reason: Option<String>,
    pub refid: Option<String>,
    pub starttm: Option<f64>,
    pub status: String,
    pub stopprice: Option<String>,
    pub trigger: Option<String>,
    pub userref: Option<f64>,
    pub vol: String,
    pub vol_exec: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClosedOrdersDescr {
    pub close: String,
    pub leverage: String,
    pub order: String,
    pub ordertype: String,
    pub pair: String,
    pub price: String,
    pub price2: String,
    #[serde(rename = "type")]
    pub closed_order_type: String,
}

impl ClosedOrders {
    /// Creates a request for closed orders and returns a struct containing the data.
    ///
    /// # Arguments
    ///
    /// * `trades` - boolean, Whether or not to include trades related to position in output
    /// * `userref` - int32, Restrict results to given user reference
    /// * `cl_ord_id` - string, Restrict results to given client order id
    /// * `start` - integer, Starting unix timestamp or order tx ID of results (exclusive)
    /// * `end` - integer, Ending unix timestamp or order tx ID of results (inclusive)
    /// * `ofs` - integer, Result offset for pagination
    /// * `closetime` - string, Possible values: [open, close, both], Default value: both, Which time to use to search
    /// * `consolidate_taker` - boolean, Default value: true, Whether or not to consolidate trades by individual taker trades
    pub async fn get(
        trades: Option<&str>,
        userref: Option<&str>,
        cl_ord_id: Option<&str>,
        start: Option<&str>,
        end: Option<&str>,
        ofs: Option<&str>,
        closetime: Option<&str>,
        consolidate_taker: Option<&str>,
    ) -> Result<Self, Error> {
        let queries = build_queries!( ; trades, userref, cl_ord_id, start, end, ofs, closetime, consolidate_taker);
        let response = get_private_json::<Self>(CLOSED_ORDERS_URI, Some(&queries)).await?;
        Ok(response.result)
    }
}
