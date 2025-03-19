use super::*;

/// struct to withdraw funds
#[derive(Debug, Deserialize, Serialize)]
pub struct WithdrawFunds {
    pub refid: String,
}

impl WithdrawFunds {
    /// Warning! untested
    /// # Arguments
    ///
    /// * `asset` - Asset being withdrawn
    /// * `key` - Withdrawal key name, as set up on your account
    /// * `address` - Optional, crypto address that can be used to confirm address matches key (will return Invalid withdrawal address error if different)
    /// * `amount` - Amount to be withdrawn
    /// * `max_fee` - if the processed withdrawal fee is higher than max_fee, withdrawal will fail with EFunding:Max fee exceeded
    pub async fn get(
        asset: &str,
        key: &str,
        address: Option<&str>,
        amount: &str,
        max_fee: Option<&str>,
    ) -> Result<Self, Error> {
        let body = build_queries!(asset, key, amount; address, max_fee);
        let response = get_private_json::<Self>(WITHDRAW_FUNDS_URI, Some(&body)).await?;
        Ok(response.result)
    }
}
