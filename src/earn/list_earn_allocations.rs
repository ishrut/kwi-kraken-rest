use super::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct ListEarnAllocations {
    converted_asset: String,
    total_allocated: String,
    total_rewarded: String,
    next_cursor: String,
    items: Vec<EarnAllocationsItems>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EarnAllocationsItems {
    strategy_id: String,
    native_asset: String,
    amount_allocated: EarnAllocationsAmount,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EarnAllocationsAmount {
    bonding: EarnAllocationsBonding,
    total: EarnAllocationAllocated,
    total_rewarded: EarnAllocationAllocated,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EarnAllocationsBonding {
    native: String,
    converted: String,
    allocation_count: usize,
    allocations: Vec<AllocationsList>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AllocationsList {
    created_at: String,
    expires: String,
    native: String,
    converted: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EarnAllocationAllocated {
    native: String,
    converted: String,
}

impl ListEarnAllocations {
    /// Warning! Untested
    /// ascending: true to sort ascending, false (the default) for descending.
    /// converted_asset: A secondary currency to express the value of your allocations (the default is USD).
    /// hide_zero_allocations: Omit entries for strategies that were used in the past but now they don't hold any allocation (the default is false)
    pub async fn get(
        ascending: Option<&str>,
        converted_asset: Option<&str>,
        hide_zero_allocations: Option<&str>,
    ) -> Result<Self, Error> {
        let body = build_queries!(; ascending, converted_asset, hide_zero_allocations);
        let response = get_private_json::<Self>(LIST_EARN_ALLOCATIONS_URI, Some(&body)).await?;
        Ok(response.result)
    }
}
