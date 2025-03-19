use super::*;

#[derive(Debug, Deserialize)]
pub struct AddOrder {
    pub txid: Vec<String>,
    pub descr: AddOrderDescr,
}

#[derive(Debug, Deserialize)]
pub struct AddOrderDescr {
    pub order: String,
}

impl AddOrder {
    /// Warning! untested
    /// Places a buy limit order and returns a struct containing the response from the server
    pub async fn buy_limit(
        pair: &str,
        volume: &str,
        price: &str,
        cl_ord_id: Option<&str>,
    ) -> Result<Self, Error> {
        let param_queries = build_queries!(pair, volume, price; cl_ord_id);
        let body = format!("{}&ordertype=limit&type=buy", param_queries);
        let response = get_private_json(ADD_ORDER_URI, Some(&body)).await?;
        Ok(response.result)
    }

    /// Warning! untested
    pub async fn sell_limit(
        pair: &str,
        volume: &str,
        price: &str,
        cl_ord_id: Option<&str>,
    ) -> Result<Self, Error> {
        let body = format!(
            "{}&ordertype=limit&type=sell",
            build_queries!(pair, volume, price; cl_ord_id)
        );
        let response = get_private_json(ADD_ORDER_URI, Some(&body)).await?;
        Ok(response.result)
    }

    /// Warning! untested
    pub async fn buy_market(
        pair: &str,
        volume: &str,
        cl_ord_id: Option<&str>,
    ) -> Result<Self, Error> {
        let body = format!(
            "{}&ordertype=market&type=buy",
            build_queries!(pair, volume; cl_ord_id)
        );
        let response = get_private_json(ADD_ORDER_URI, Some(&body)).await?;
        Ok(response.result)
    }

    /// Warning! untested
    pub async fn sell_market(
        pair: &str,
        volume: &str,
        cl_ord_id: Option<&str>,
    ) -> Result<Self, Error> {
        let body = format!(
            "{}&ordertype=market&type=sell",
            build_queries!(pair, volume; cl_ord_id)
        );
        let response = get_private_json(ADD_ORDER_URI, Some(&body)).await?;
        Ok(response.result)
    }
}
