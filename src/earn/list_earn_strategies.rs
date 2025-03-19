use super::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct ListEarnStrategies {
    pub next_cursor: String,
    pub items: Vec<ListEarnStrategiesItems>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ListEarnStrategiesItems {
    pub id: String,
    pub asset: String,
    pub lock_type: LockType,
    pub apr_estimate: AprEstimate,
    pub user_min_allocation: String,
    pub allocation_fee: String,
    pub deallocation_fee: String,
    pub auto_compound: ListEarnStrategiesType,
    pub yield_source: ListEarnStrategiesType,
    pub can_allocate: bool,
    pub can_deallocate: bool,
    pub allocation_restriction_info: Vec<String>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct LockType {
    pub r#type: String,
    pub payout_frequency: usize,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct AprEstimate {
    pub low: String,
    pub high: String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct ListEarnStrategiesType {
    pub r#type: String,
}

impl ListEarnStrategies {
    /// Warning! untested
    /// ascending: true to sort ascending, false (the default) for descending.
    /// asset: Filter strategies by asset name
    /// cursor: None to start at beginning/end, otherwise next page ID
    /// limit: How many items to return per page. Note that the limit may be cap'd to lower value in the application code.
    /// lock_type: Possible values: [flex, bonded, timed, instant], Filter strategies by lock type
    pub async fn get(
        ascending: Option<&str>,
        asset: Option<&str>,
        cursor: Option<&str>,
        limit: Option<&str>,
        lock_type: Option<&str>,
    ) -> Result<Self, Error> {
        let body = build_queries!(;ascending, asset, cursor, limit, lock_type);
        let response = get_private_json::<Self>(LIST_EARN_STRATEGIES_URI, Some(&body)).await?;
        Ok(response.result)
    }
}
