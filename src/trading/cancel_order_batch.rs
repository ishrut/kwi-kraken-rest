use super::*;

/// struct to cancel order batch
#[derive(Debug, Deserialize, Serialize)]
pub struct CancelOrderBatch {
    pub count: usize,
}

impl CancelOrderBatch {
    /// Warning! unimplemented
    pub async fn get() {
        todo!()
    }
}
