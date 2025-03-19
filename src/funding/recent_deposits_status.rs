use super::*;

/// struct to get recent deposit status
#[derive(Debug, Deserialize, Serialize)]
pub struct RecentDepositsStatus {
    pub deposit: Deposit,
    pub next_cursor: String,
}

/// RecentDepositsStatus inner field
#[derive(Debug, Deserialize, Serialize)]
pub struct Deposit {
    pub method: String,
    pub aclass: String,
    pub asset: String,
    pub refid: String,
    pub txid: String,
    pub info: String,
    pub amount: String,
    pub fee: String,
    pub time: i32,
    pub status: String,
    #[serde(rename = "status-prop")]
    pub status_prop: String,
    pub originators: Vec<String>,
}

impl RecentDepositsStatus {
    /// Warning: untested
    /// Creates a request for account balance and returns a struct containing the data.
    /// # Params
    /// asset: Filter for specific asset being deposited
    /// aclass: Default value: currency, Filter for specific asset class being deposited
    /// method: Filter for specific name of deposit method
    /// start: Start timestamp, deposits created strictly before will not be included in the response
    /// end: End timestamp, deposits created strictly after will be not be included in the response
    /// cursor: true/false to enable/disable paginated response (boolean) or cursor for next page of results (string)
    /// limit: Default value: 25, Number of results to include per page
    pub async fn get(
        asset: Option<&str>,
        aclass: Option<&str>,
        method: Option<&str>,
        start: Option<&str>,
        end: Option<&str>,
        cursor: Option<&str>,
        limit: Option<&str>,
    ) -> Result<Self, Error> {
        let queries = build_queries!(; asset, aclass, method, start, end, cursor, limit);
        let response = get_private_json::<Self>(DEPOSIT_STATUS_URI, Some(&queries)).await?;
        Ok(response.result)
    }
}
