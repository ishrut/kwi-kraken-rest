use super::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct CancelOrderBatch {
    count: usize,
}

impl CancelOrderBatch {
    /// Warning! unimplemented
    pub async fn get() {
        todo!()
    }
}
