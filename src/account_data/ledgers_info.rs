use super::*;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct LedgersInfo {
    pub count: u32,
    pub ledger: HashMap<String, LedgersInfoData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LedgersInfoData {
    pub aclass: String,
    pub amount: String,
    pub asset: String,
    pub balance: String,
    pub fee: String,
    pub refid: String,
    pub subtype: String,
    pub time: f64,
    #[serde(rename = "type")]
    pub ledger_type: String,
}

impl LedgersInfo {
    /// Creates a request for ledgers information and returns a struct containing the data.
    /// # Arguments
    ///
    /// * `asset` - string, Default value: all, Filter output by asset or comma delimited list of assets
    /// * `aclass` - string, Default value: currency, Filter output by asset class
    /// * `ledger_type` - string, Possible values: [all, trade, deposit, withdrawal, transfer, margin, adjustment, rollover, credit, settled, staking, dividend, sale, nft_rebate], Default value: all, Type of ledger to retrieve
    /// * `start` - integer, Starting unix timestamp or ledger ID of results (exclusive)
    /// * `end` - integer, Ending unix timestamp or ledger ID of results (inclusive)
    /// * `ofs` - integer, Result offset for pagination
    /// * `without_count` - boolean, If true, does not retrieve count of ledger entries. Request can be noticeably faster for users with many ledger entries as this avoids an extra database query.
    pub async fn get(
        asset: Option<&str>,
        aclass: Option<&str>,
        ledger_type: Option<&str>,
        start: Option<&str>,
        end: Option<&str>,
        ofs: Option<&str>,
        without_count: Option<&str>,
    ) -> Result<Self, Error> {
        let queries = build_queries!(; asset, aclass, start, end, ofs, without_count);
        let body = match ledger_type {
            Some(val) => format!("{}&type={}", &queries, val),
            None => queries,
        };
        let response = get_private_json::<Self>(LEDGERS_INFO_URI, Some(&body)).await?;
        Ok(response.result)
    }
}
