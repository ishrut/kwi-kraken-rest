use super::*;

/// struct to deallocate earn funds
#[derive(Debug)]
pub struct DeallocateEarnFunds;

impl DeallocateEarnFunds {
    /// Warning! untested
    pub async fn get(amount: &str, strategy_id: &str) -> Result<bool, Error> {
        let body = build_queries!(amount, strategy_id;);
        let response = get_private_json::<bool>(DEALLOCATE_EARN_FUNDS_URI, Some(&body)).await?;
        Ok(response.result)
    }
}
