use super::*;

/// struct to cancel all orders after x
#[derive(Debug, Deserialize, Serialize)]
pub struct CancelAllOrdersAfter {
    #[serde(rename = "currentTime")]
    pub current_time: String,
    #[serde(rename = "triggerTime")]
    pub trigger_time: String,
}

impl CancelAllOrdersAfter {
    /// Warning! unimplemented
    pub async fn get() {
        todo!()
    }
}
