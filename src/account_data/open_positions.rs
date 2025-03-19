use super::*;
use std::collections::HashMap;

/// struct to get OpenPositions
#[derive(Debug, Deserialize, Serialize)]
pub struct OpenPositions {
    pub ordertxid: String,
    pub posstatus: String,
    pub pair: String,
    pub time: f64,
    #[serde(rename = "type")]
    pub position_type: String,
    pub ordertype: String,
    pub cost: String,
    pub fee: String,
    pub vol: String,
    pub vol_closed: String,
    pub margin: String,
    pub value: String,
    pub net: String,
    pub terms: String,
    pub rollovertm: String,
    pub misc: String,
    pub oflags: String,
}
impl OpenPositions {
    /// Creates a request for open positions and returns a struct containing the data.
    /// * `txid` - string, Comma delimited list of txids to limit output to
    /// * `docalcs` - boolean, Whether to include P&L calculations
    /// * `consolidation` - string, Possible values: [market], Consolidate positions by market/pair
    pub async fn get(
        txid: Option<&str>,
        docalcs: Option<&str>,
        consolidation: Option<&str>,
    ) -> Result<HashMap<String, OpenPositions>, Error> {
        let queries = build_queries!( ; txid, docalcs, consolidation);
        let response =
            get_private_json::<HashMap<String, OpenPositions>>(OPEN_POSITIONS_URI, Some(&queries))
                .await?;
        Ok(response.result)
    }
}
