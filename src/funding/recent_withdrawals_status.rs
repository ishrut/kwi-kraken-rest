use super::*;

/// struct to get recent withdrawal status
#[derive(Debug, Deserialize, Serialize)]
pub struct RecentWithdrawalStatus {
    pub method: String,
    pub aclass: String,
    pub asset: String,
    pub refid: String,
    pub txid: String,
    pub info: String,
    pub amount: String,
    pub fee: String,
    pub time: f64,
    pub status: String,
    pub key: String,
}

impl RecentWithdrawalStatus {
    /// Warning! untested
    /// # Params
    ///
    /// * `asset` - Filter for specific asset being withdrawn
    /// * `aclass` - Default value: currency, Filter for specific asset class being withdrawn
    /// * `method` - Filter for specific name of withdrawal method
    /// * `start` - Start timestamp, withdrawals created strictly before will not be included in the response
    /// * `end` - End timestamp, withdrawals created strictly after will be not be included in the response
    /// * `cursor` - true/false to enable/disable paginated response (boolean) or cursor for next page of results (string), default false
    /// * `limit` - Default value: 500, Number of results to include per page
    pub async fn get(
        asset: Option<&str>,
        aclass: Option<&str>,
        method: Option<&str>,
        start: Option<&str>,
        end: Option<&str>,
        cursor: Option<&str>,
        limit: Option<&str>,
    ) -> Result<Vec<Self>, Error> {
        let body = build_queries!(; asset, aclass, method, start, end, cursor, limit);
        let response = get_private_json::<Vec<Self>>(WITHDRAW_STATUS_URI, Some(&body)).await?;
        Ok(response.result)
    }
}
