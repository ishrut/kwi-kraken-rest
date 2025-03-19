use super::*;

/// struct to list earn allocations
#[derive(Debug, Deserialize, Serialize)]
pub struct ListEarnAllocations {
    pub converted_asset: String,
    pub total_allocated: String,
    pub total_rewarded: String,
    pub next_cursor: String,
    pub items: Vec<EarnAllocationsItems>,
}

/// ListEarnAllocation inner field
#[derive(Debug, Deserialize, Serialize)]
pub struct EarnAllocationsItems {
    pub strategy_id: String,
    pub native_asset: String,
    pub amount_allocated: EarnAllocationsAmount,
}

/// EarnAllocationsItems inner field
#[derive(Debug, Deserialize, Serialize)]
pub struct EarnAllocationsAmount {
    pub bonding: EarnAllocationsBonding,
    pub total: EarnAllocationAllocated,
    pub total_rewarded: EarnAllocationAllocated,
}

/// EarnAllocationsAmount inner field
#[derive(Debug, Deserialize, Serialize)]
pub struct EarnAllocationsBonding {
    pub native: String,
    pub converted: String,
    pub allocation_count: usize,
    pub allocations: Vec<AllocationsList>,
}

/// EarnAllocationsBonding inner field
#[derive(Debug, Deserialize, Serialize)]
pub struct AllocationsList {
    created_at: String,
    expires: String,
    native: String,
    converted: String,
}

/// EarnAllocationsAmount inner field
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
