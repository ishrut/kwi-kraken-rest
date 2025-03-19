use super::*;

/// struct to request trade balance
#[derive(Debug, Serialize, Deserialize)]
pub struct TradeBalance {
    pub eb: String,
    pub tb: String,
    pub m: String,
    pub uv: String,
    pub n: String,
    pub c: String,
    pub v: String,
    pub e: String,
    pub mf: String,
}

impl TradeBalance {
    /// Creates a request for trade balance and returns a struct containing the data.
    ///
    /// # Arguments
    /// * `asset` - Default value: ZUSD, Base asset used to determine balance
    pub async fn new(asset: Option<&str>) -> Result<Self, Error> {
        let queries = build_queries!(;asset);
        let response = get_private_json::<Self>(TRADE_BALANCE_URI, Some(&queries)).await?;
        Ok(response.result)
    }
}
