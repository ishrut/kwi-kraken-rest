use super::*;

/// struct to create subaccount
pub struct CreateSubaccount;

impl CreateSubaccount {
    /// Warning! untested
    pub async fn get(username: &str, email: &str) -> Result<bool, Error> {
        let queries = build_queries!(username, email ;);
        let response = get_private_json(CREATE_ACCOUNT_URI, Some(&queries)).await?;
        Ok(response.result)
    }
}
