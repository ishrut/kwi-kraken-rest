use super::*;

#[derive(Debug, Deserialize)]
pub struct EditOrder {
    pub descr: EditOrderDescr,
    pub orders_cancelled: u32,
    pub originaltxid: String,
    pub price: String,
    pub price2: String,
    pub status: String,
    pub txid: String,
    pub volume: String,
}

#[derive(Debug, Deserialize)]
pub struct EditOrderDescr {
    pub order: String,
}

impl EditOrder {
    /// Creates a request to edit orders and returns a struct containing the data.
    /// Deprecated use amend_order instead.
    #[deprecated(note = "use amend_order instead")]
    pub async fn edit_order(
        txid: &str,
        volume: Option<&str>,
        displayvol: Option<&str>,
        pair: &str,
        price: Option<&str>,
        price2: Option<&str>,
        oflags: Option<&str>,
        deadline: Option<&str>,
        cancel_response: Option<&str>,
        validate: Option<&str>,
    ) -> Result<Self, Error> {
        let body = build_queries!(txid, pair; volume, displayvol, price2, price, oflags, deadline, cancel_response, validate);
        let response = get_private_json::<Self>(EDIT_ORDER_URI, Some(&body)).await?;
        Ok(response.result)
    }
}
