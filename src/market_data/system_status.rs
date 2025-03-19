use super::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemStatus {
    pub status: String,
    pub timestamp: String,
}

impl SystemStatus {
    /// Creates a request for system status.
    pub async fn get() -> Result<Self, Error> {
        let url = build_url!(BASE_URL, SYSTEM_STATUS_URI);
        let response = get_public_json::<SystemStatus>(&url).await?;
        Ok(response.result)
    }
}
