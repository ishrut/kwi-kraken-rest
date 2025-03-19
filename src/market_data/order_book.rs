use super::*;
use std::collections::HashMap;

/// struct to get order book
#[derive(Debug, Serialize, Deserialize)]
pub struct OrderBook {
    pub asks: Vec<OrderBookData>,
    pub bids: Vec<OrderBookData>,
}

/// OrderBook inner field
#[derive(Debug, Serialize, Deserialize)]
pub struct OrderBookData {
    #[serde(rename = "0")]
    pub price: String,
    #[serde(rename = "1")]
    pub quantity: String,
    #[serde(rename = "2")]
    pub timestamps: u32,
}

impl OrderBook {
    /// Creates a request for order book and returns a struct containing the data.
    ///
    /// # Parametres
    /// * `pair` - Pair for which you want the order book information.
    /// * `count` - is the depth for which you want the order book, default 100, values 1..500, maximum bids and asks.
    pub async fn get(pair: &str, count: Option<&str>) -> Result<HashMap<String, OrderBook>, Error> {
        let url = build_url!(BASE_URL, ORDER_BOOK_URI; pair; count);
        let response = get_public_json::<HashMap<String, OrderBook>>(&url).await?;
        Ok(response.result)
    }
}
