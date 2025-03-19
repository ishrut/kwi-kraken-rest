use super::*;

/// struct to get deposit methods
#[derive(Debug, Deserialize, Serialize)]
pub struct DepositMethods {
    pub fee: String,
    #[serde(rename = "gen-address")]
    pub gen_address: bool,
    pub limit: bool,
    pub method: String,
    pub minimum: String,
}

impl DepositMethods {
    /// # Arguments
    ///
    /// * `asset` - Asset being deposited
    /// * `aclass` - Default value: currency, Asset class being deposited (optional)
    pub async fn get(asset: &str, aclass: Option<&str>) -> Result<Vec<DepositMethods>, Error> {
        let body = build_queries!(asset; aclass);
        let response = get_private_json::<Vec<Self>>(DEPOSIT_METHODS_URI, Some(&body)).await?;
        Ok(response.result)
    }
}
