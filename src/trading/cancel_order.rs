use super::*;

/// struct to cancel order
#[derive(Debug, Deserialize)]
pub struct CancelOrder {
    pub count: usize,
    pub pending: Option<bool>,
}

impl CancelOrder {
    /// # Arguments
    ///
    /// * `txid` - order txid to cancel
    pub async fn cancel_txid(txid: &str) -> Result<CancelOrder, Error> {
        let body = build_queries!(txid; );
        let resp = get_private_json::<Self>(CANCEL_ORDER_URI, Some(&body)).await?;
        Ok(resp.result)
    }
}
