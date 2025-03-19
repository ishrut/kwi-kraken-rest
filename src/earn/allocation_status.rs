use super::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct AllocationStatus {
    pub pending: bool,
}

impl AllocationStatus {
    /// Warning! untested
    pub async fn get(strategy_id: &str) -> Result<Self, Error> {
        let body = build_queries!(strategy_id;);
        let response = get_private_json::<Self>(ALLOCATION_STATUS_URI, Some(&body)).await?;
        Ok(response.result)
    }
}
