use super::*;

/// struct to cancel all orders
#[derive(Debug, Deserialize, Serialize)]
pub struct CancelAllOrders {
    pub count: usize,
    pub pending: Option<bool>,
}

impl CancelAllOrders {
    pub async fn get() {
        todo!()
    }
}
