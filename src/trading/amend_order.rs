use super::*;

#[derive(Debug, Deserialize)]
pub struct AmendOrder {
    pub amend_id: String,
}

impl AmendOrder {
    /// Creates a request to amend orders and returns a struct containing the data.
    /// # Arguments
    ///
    /// * `txid` - The Kraken identifier for the order to be amended. Either txid or cl_ord_id is required.
    /// * `cl_ord_id` - The client identifier for the order to be amended. Either txid or cl_ord_id is required.
    /// * `order_qty` - The new order quantity in terms of the base asset.
    /// * `display_qty` - For iceberg orders only, it defines the new quantity to show in the book while the rest of order quantity remains hidden. Minimum value is 1 / 15 of remaining order quantity.
    /// * `limit_price` - The new limit price restriction on the order (for order types that support limit price only).
    ///    The relative pricing can be set by using the +, - prefixes and/or % suffix.
    ///    • + adds the amount from the reference price, i.e. market rises 50 USD "+50".
    ///    • - subtracts the amount from the reference price, i.e. market drops 100 USD "-100".
    /// * `trigger_price` - The new trigger price to activate the order (for triggered order types only).
    ///    The relative pricing can be set by using the +, - prefixes and/or % suffix.
    ///    • + adds the amount from the reference price, i.e. market rises 50 USD "+50".
    ///    • - subtracts the amount from the reference price, i.e. market drops 100 USD "-100".
    /// * `post_only` - An optional flag for limit_price amends. If true, the limit price change will be rejected if the order cannot be posted passively in the book.
    /// * `deadline` - RFC3339 timestamp (e.g. 2021-04-01T00:18:45Z) after which the matching engine should reject the new order request, in presence of latency or order queueing. min now() + 2 seconds, max now() + 60 seconds.
    pub async fn get(
        txid: Option<&str>,
        cl_ord_id: Option<&str>,
        order_qty: Option<&str>,
        display_qty: Option<&str>,
        limit_price: Option<&str>,
        trigger_price: Option<&str>,
        post_only: Option<&str>,
    ) -> Result<Self, Error> {
        let body = build_queries!( ; txid, cl_ord_id, order_qty, display_qty, limit_price, trigger_price, post_only);
        let response = get_private_json::<Self>(AMEND_ORDER_URI, Some(&body)).await?;
        Ok(response.result)
    }
}
