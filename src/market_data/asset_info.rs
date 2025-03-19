use super::*;
use std::collections::HashMap;

/// AssetInfo inner field
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AssetInfoDescr {
    pub aclass: String,
    pub altname: String,
    pub decimals: u32,
    pub display_decimals: u32,
    pub collateral_value: Option<f32>,
    pub status: String,
}

/// struct to get asset info
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetInfo {
    #[serde(flatten)]
    pub assets: HashMap<String, AssetInfoDescr>,
}

impl AssetInfo {
    /// Creates a request for asset info and returns a struct.
    ///
    /// # Parametres
    ///
    /// * `asset` - is an Option of the asset, e.g Some("ETH")
    /// * `aclass` - is an Option, e.g Some("currency")
    pub async fn get(asset: Option<&str>, aclass: Option<&str>) -> Result<Self, Error> {
        let url = build_url!(BASE_URL, ASSET_INFO_URI; ; asset, aclass);
        let response = get_public_json::<Self>(&url).await?;
        Ok(response.result)
    }
}
