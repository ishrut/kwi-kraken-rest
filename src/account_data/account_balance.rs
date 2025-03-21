use super::*;
use std::collections::HashMap;

/// Struct to get account balance
#[derive(Debug)]
pub struct AccountBalance {}

impl AccountBalance {
    /// Creates a request for account balance
    pub async fn get() -> Result<HashMap<String, String>, Error> {
        let response =
            get_private_json::<HashMap<String, String>>(ACCOUNT_BALANCE_URI, None).await?;
        Ok(response.result)
    }

    /// Warning! unimplemented
    pub async fn filter_assets() -> HashMap<String, f64> {
        todo!()
    }

    /// Warning! unimplemented
    pub async fn filter_currency() -> HashMap<String, f64> {
        todo!()
    }
}
