use super::*;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct TickerInfo {
    pub a: Vec<String>,
    pub b: Vec<String>,
    pub c: Vec<String>,
    pub v: Vec<String>,
    pub p: Vec<String>,
    pub t: Vec<u32>,
    pub l: Vec<String>,
    pub h: Vec<String>,
    pub o: String,
}

impl TickerInfo {
    /// Creates a request for ticker information and returns a struct containing the data.
    ///
    /// # Parametres
    ///
    /// * `pair` - is an Option of the pair for which you want the ticker information or all of them given.
    pub async fn get(pair: Option<&str>) -> Result<HashMap<String, TickerInfo>, Error> {
        let url = build_url!(BASE_URL, TICKER_INFORMATION_URI; ; pair);
        let response = get_public_json::<HashMap<String, TickerInfo>>(&url).await?;
        Ok(response.result)
    }
}
