use super::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct WalletTransfer {
    refid: String,
}

impl WalletTransfer {
    /// Warning! untested
    /// # Params
    /// asset: Asset to transfer (asset ID or altname)
    /// from: Possible values: [Spot Wallet], Source wallet
    /// to: Possible values: [Futures Wallet], Destination wallet
    /// amount: Amount to transfer
    pub async fn get(asset: &str, from: &str, to: &str, amount: &str) -> Result<Self, Error> {
        let body = build_queries!(asset, from, to, amount;);
        let response = get_private_json::<Self>(WALLET_TRANSFER_URI, Some(&body)).await?;
        Ok(response.result)
    }
}
