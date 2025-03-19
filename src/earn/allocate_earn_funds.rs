use super::*;

/// struct to allocate earn funds
#[derive(Debug, Deserialize, Serialize)]
pub struct AllocateEarnFunds;

impl AllocateEarnFunds {
    /// Warning! untested
    pub async fn get(amount: &str, strategy_id: &str) -> Result<bool, Error> {
        let body = build_queries!(amount, strategy_id;);
        let response = get_private_json::<bool>(ALLOCATE_EARN_FUNDS_URI, Some(&body)).await?;
        Ok(response.result)
    }
}
