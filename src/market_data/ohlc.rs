use super::*;
use std::collections::HashMap;

/// struct to get ohlc
#[derive(Debug, Serialize, Deserialize)]
pub struct Ohlc {
    pub last: u32,
    #[serde(flatten)]
    pub ohlcvt: HashMap<String, Vec<Ohlcvt>>,
}

/// Ohlc inner field
#[derive(Debug, Serialize, Deserialize)]
pub struct Ohlcvt {
    #[serde(rename = "0")]
    pub timestamp: u32,
    #[serde(rename = "1")]
    pub open: String,
    #[serde(rename = "2")]
    pub high: String,
    #[serde(rename = "3")]
    pub low: String,
    #[serde(rename = "4")]
    pub close: String,
    #[serde(rename = "5")]
    pub vwap: String,
    #[serde(rename = "6")]
    pub volume: String,
    #[serde(rename = "7")]
    pub count: u32,
}

impl Ohlc {
    /// # Examples
    ///
    /// ```
    /// use kraken_rest::market_data::Ohlc;
    /// let ohlc_data = Ohlc::get("SOLEUR", Some(1), None).await.unwrap();
    /// ```
    ///
    /// # Arguments
    ///
    /// * `pair` - &str is the asset pair you want the data for e.g SOLEUR, BTCUSD ...
    /// * `interval` - `Option<u32>` is the interval for which you want the candles e.g 1, 5, 15, 30, 60, 240, 1440, 10080, 21600.
    /// * `since` -`Option<u64>` is the unixtimestamp for since when you want the data. Returns last 720 candles by default.
    ///
    /// # Returns
    ///
    /// Response in deserialized form.  
    pub async fn get(
        pair: &str,
        interval: Option<&str>,
        since: Option<&str>,
    ) -> Result<Self, Error> {
        let url = build_url!(BASE_URL, OHLC_URI; pair; interval, since);
        let response = get_public_json::<Ohlc>(&url).await?;
        Ok(response.result)
    }
}
