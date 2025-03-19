use super::*;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtendedBalance {
    pub balance: String,
    pub hold_trade: String,
}

impl ExtendedBalance {
    /// Creates a request for extended balance and returns a struct containing the data.
    pub async fn get() -> Result<HashMap<String, Self>, Error> {
        let response =
            get_private_json::<HashMap<String, Self>>(EXTENDED_BALANCE_URI, None).await?;
        Ok(response.result)
    }
}
