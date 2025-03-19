use super::*;

/// struct to get withdrawal addresses
#[derive(Debug, Deserialize, Serialize)]
pub struct WithdrawalAddresses {
    pub address: String,
    pub asset: String,
    pub method: String,
    pub key: String,
    pub tag: String,
    pub verified: bool,
}

impl WithdrawalAddresses {
    /// Warning! untested
    /// asset: Filter addresses for specific asset
    /// aclass: Default value: currency, Filter addresses for specific asset class
    /// method: Filter addresses for specific method
    /// key: Find address for by withdrawal key name, as set up on your account
    /// verified: Filter by verification status of the withdrawal address. Withdrawal addresses successfully completing email confirmation will have a verification status of true.
    pub async fn get(
        asset: Option<&str>,
        aclass: Option<&str>,
        method: Option<&str>,
        key: Option<&str>,
        verified: Option<&str>,
    ) -> Result<Vec<Self>, Error> {
        let body = build_queries!(;asset, aclass, method, key, verified);
        let response = get_private_json::<Vec<Self>>(WITHDRAW_ADDRESSES_URI, Some(&body)).await?;
        Ok(response.result)
    }
}
