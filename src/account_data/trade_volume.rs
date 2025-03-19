use super::*;
use std::collections::HashMap;

/// struct to request trade volume
#[derive(Debug, Serialize, Deserialize)]
pub struct TradeVolume {
    pub currency: String,
    pub volume: String,
    pub fees: Option<HashMap<String, TradeVolumeFees>>,
    pub fees_maker: Option<HashMap<String, TradeVolumeFeesMaker>>,
}

/// TradeVolume inner field
#[derive(Debug, Serialize, Deserialize)]
pub struct TradeVolumeFees {
    pub fee: String,
    pub minfee: String,
    pub maxfee: String,
    pub nextfee: Option<String>,
    pub nextvolume: Option<String>,
    pub tiervolume: String,
}

/// TradeVolume inner field
#[derive(Debug, Serialize, Deserialize)]
pub struct TradeVolumeFeesMaker {
    pub fee: String,
    pub minfee: String,
    pub maxfee: String,
    pub nextfee: Option<String>,
    pub nextvolume: Option<String>,
    pub tiervolume: String,
}

impl TradeVolume {
    /// # Arguments
    ///
    /// * `pair` - string, Comma delimited list of asset pairs to get fee info on (optional, but required if any fee info is desired)
    pub async fn get(pair: Option<Vec<&str>>) -> Result<Self, Error> {
        let queries = match pair {
            Some(vals) => {
                let query = format!("pair={}", vals.join(","));
                Some(query)
            }
            None => None,
        };
        let response = match queries {
            Some(query) => get_private_json::<Self>(TRADE_VOLUME_URI, Some(&query)).await?,
            None => get_private_json(TRADE_VOLUME_URI, None).await?,
        };
        Ok(response.result)
    }
}
