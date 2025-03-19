use super::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct CancelAllOrders {
    count: usize,
    pending: Option<bool>,
}

impl CancelAllOrders {
    pub async fn get() {
        todo!()
    }
}
