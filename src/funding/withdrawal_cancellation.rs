use super::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct WithdrawalCancellation;

impl WithdrawalCancellation {
    /// Warning! untested
    /// asset: Asset being withdrawn
    /// refid: Withdrawal reference ID
    pub async fn get(asset: &str, refid: &str) -> Result<bool, Error> {
        let body = build_queries!(asset, refid;);
        let response = get_private_json::<bool>(WITHDRAW_CANCEL_URI, Some(&body)).await?;
        Ok(response.result)
    }
}
