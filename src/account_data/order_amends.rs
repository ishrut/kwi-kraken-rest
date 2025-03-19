use super::*;

/// struct to get order amends
#[derive(Debug, Deserialize, Serialize)]
pub struct OrderAmends {
    pub amends: Vec<OrderAmendsData>,
    pub count: usize,
}

/// OrderAmends inner field
#[derive(Debug, Deserialize, Serialize)]
pub struct OrderAmendsData {
    pub amend_id: String,
    pub amend_type: String,
    pub order_qty: String,
    pub remaining_qty: String,
    pub limit_price: String,
    pub timestamp: usize,
}

impl OrderAmends {
    /// * `order_id` - string, The Kraken order identifier for the amended order.
    pub async fn get(order_id: Option<&str>) -> Result<Self, Error> {
        let queries = build_queries!( ; order_id);
        let response = get_private_json::<Self>(ORDER_AMENDS_URI, Some(&queries)).await?;
        Ok(response.result)
    }
}
