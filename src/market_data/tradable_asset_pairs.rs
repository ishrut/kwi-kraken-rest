use super::*;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct TradableAssetPairs {
    pub aclass_base: Option<String>,
    pub aclass_quote: Option<String>,
    pub altname: Option<String>,
    pub base: Option<String>,
    pub cost_decimals: Option<u32>,
    pub costmin: Option<String>,
    pub fee_volume_currency: Option<String>,
    pub fees: Option<Vec<Vec<f64>>>,
    pub fees_maker: Option<Vec<Vec<f64>>>,
    pub leverage_buy: Option<Vec<u32>>,
    pub leverage_sell: Option<Vec<u32>>,
    pub long_position_limit: Option<u64>,
    pub short_position_limit: Option<u64>,
    pub lot: Option<String>,
    pub lot_decimals: Option<u32>,
    pub lot_multiplier: Option<u32>,
    pub margin_call: Option<u32>,
    pub margin_stop: Option<u32>,
    pub ordermin: Option<String>,
    pub pair_decimals: Option<u32>,
    pub quote: Option<String>,
    pub status: Option<String>,
    pub tick_size: Option<String>,
    pub wsname: Option<String>,
}

impl TradableAssetPairs {
    /// Creates a request for tradable asset pairs and returns a struct containing the data.
    ///
    /// # Parametres
    /// * `pair` - Asset pairs to get data for, e.g ETH/BTC
    /// * `info` - is an Option and can be: "info"(default), "leverage", "fees", "margin"
    pub async fn get(
        pair: Option<Vec<&str>>,
        info: Option<&str>,
        country_code: Option<&str>,
    ) -> Result<HashMap<String, TradableAssetPairs>, Error> {
        let url = match pair {
            Some(pairs) => {
                let pair_concat = pairs.join(",");
                format!(
                    "{}pair={}",
                    build_url!(BASE_URL, TRADADLE_ASSET_PAIRS_URI; ; info, country_code),
                    pair_concat,
                )
            }
            None => build_url!(BASE_URL, TRADADLE_ASSET_PAIRS_URI; ; info, country_code),
        };
        let response = get_public_json::<HashMap<String, TradableAssetPairs>>(&url).await?;
        Ok(response.result)
    }
}
