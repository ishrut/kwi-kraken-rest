use super::*;

/// struct to batch add order
#[derive(Debug, Deserialize, Serialize)]
pub struct AddOrderBatch {
    pub txid: String,
    pub descr: AddOrderBatchDescr,
    pub close: Option<String>,
}

/// AddOrderBatch inner field
#[derive(Debug, Deserialize, Serialize)]
pub struct AddOrderBatchDescr {
    pub order: String,
}

impl AddOrderBatch {
    /// Warning! not yet implemented
    pub async fn get() {
        todo!()
    }
}
