use super::*;

/// struct to get deallocation status
#[derive(Debug, Deserialize, Serialize)]
pub struct DeallocationStatus {
    pending: bool,
}

impl DeallocationStatus {
    /// Warning! untested
    pub async fn get(strategy_id: &str) -> Result<Self, Error> {
        let body = build_queries!(strategy_id;);
        let response = get_private_json::<Self>(DEALLOCATION_STATUS_URI, Some(&body)).await?;
        Ok(response.result)
    }
}
