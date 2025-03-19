use super::*;

/// struct for withdrawal info
#[derive(Debug, Deserialize, Serialize)]
pub struct WithdrawalInfo {
    pub method: String,
    pub limit: String,
    pub amount: String,
    pub fee: String,
}

impl WithdrawalInfo {
    /// Warning! untested
    /// # Arguments
    ///
    /// * `asset` - Asset being withdrawn
    /// * `key` - Withdrawal key name, as set up on your account
    /// * `amount` - Amount to be withdrawn
    pub async fn get(asset: &str, key: &str, amount: &str) -> Result<Self, Error> {
        let body = build_queries!(asset, key, amount; );
        let response = get_private_json::<Self>(WITHDRAW_INFO_URI, Some(&body)).await?;
        Ok(response.result)
    }
}
