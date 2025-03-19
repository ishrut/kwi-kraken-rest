use super::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct DepositMethods {
    fee: String,
    #[serde(rename = "gen-address")]
    gen_address: bool,
    limit: bool,
    method: String,
    minimum: String,
}

impl DepositMethods {
    /// asset: Asset being deposited
    /// aclass: Default value: currency, Asset class being deposited (optional)
    pub async fn get(asset: &str, aclass: Option<&str>) -> Result<Vec<DepositMethods>, Error> {
        let body = build_queries!(asset; aclass);
        let response = get_private_json::<Vec<Self>>(DEPOSIT_METHODS_URI, Some(&body)).await?;
        Ok(response.result)
    }
}
