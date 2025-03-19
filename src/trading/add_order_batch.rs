use super::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct AddOrderBatch {
    pub txid: String,
    pub descr: AddOrderBatchDescr,
    pub close: Option<String>,
}

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
