use super::*;

#[derive(Debug, Deserialize)]
pub struct DepositAddresses {
    pub address: String,
    pub expiretm: String,
    pub new: bool,
    pub tag: Option<String>,
}

impl DepositAddresses {
    /// asset: Asset being deposited
    /// method: Name of the deposit method
    /// new: Whether or not to generate a new address
    /// amount: Amount you wish to deposit (only required for method=Bitcoin Lightning)
    pub async fn get(
        asset: &str,
        method: &str,
        new: Option<bool>,
        amount: Option<&str>,
    ) -> Result<Vec<DepositAddresses>, Error> {
        let queries = build_queries!(asset, method; new, amount);
        let response = get_private_json(DEPOSIT_ADDRESSES_URI, Some(&queries)).await?;
        Ok(response.result)
    }
}
