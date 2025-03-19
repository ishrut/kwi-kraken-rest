use super::*;

/// struct to get server time
#[derive(Debug, Serialize, Deserialize)]
pub struct ServerTime {
    pub unixtime: i32,
    pub rfc1123: String,
}

impl ServerTime {
    /// Creates a request for server time.
    pub async fn get() -> Result<Self, Error> {
        let url = build_url!(BASE_URL, SERVER_TIME_URI);
        let response = get_public_json::<ServerTime>(&url).await?;
        Ok(response.result)
    }
}
