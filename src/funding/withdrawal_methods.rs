use super::*;

/// struct to get withdrawal methods
#[derive(Debug, Deserialize, Serialize)]
pub struct WithdrawalMethods {
    pub asset: String,
    pub method: String,
    pub network: String,
    pub minimum: String,
}

impl WithdrawalMethods {
    /// Warning! untested
    /// # Arguments
    ///
    /// * `asset` - Filter methods for specific asset
    /// * `aclass` - Default value: currency, Filter methods for specific asset class
    /// * `network` - Filter methods for specific network
    pub async fn get(
        asset: Option<&str>,
        aclass: Option<&str>,
        network: Option<&str>,
    ) -> Result<Self, Error> {
        let body = build_queries!(; asset, aclass, network);
        let response = get_private_json::<Self>(WITHDRAW_METHODS_URI, Some(&body)).await?;
        Ok(response.result)
    }
}
