use super::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct AccountTransfer {
    pub transfer_id: String,
    pub status: String,
}

impl AccountTransfer {
    /// Warning! untested
    pub async fn get(asset: &str, amount: &str, from: &str, to: &str) -> Result<Self, Error> {
        let queries = build_queries!(asset, amount, from, to ;);
        let response = get_private_json::<Self>(ACCOUNT_TRANSFER_URI, Some(&queries)).await?;
        Ok(response.result)
    }
}
