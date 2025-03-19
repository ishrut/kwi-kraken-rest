//! Module for earn functions

use super::*;
use serde::{Deserialize, Serialize};

// untested
mod allocate_earn_funds;
// untested
mod deallocate_earn_funds;
// untested
mod allocation_status;
// untested
mod deallocation_status;
// untested
mod list_earn_allocations;
// untested
mod list_earn_strategies;

const ALLOCATE_EARN_FUNDS_URI: &str = "/0/private/Earn/Allocate";
const DEALLOCATE_EARN_FUNDS_URI: &str = "/0/private/Earn/Deallocate";
const ALLOCATION_STATUS_URI: &str = "/0/private/Earn/AllocateStatus";
const DEALLOCATION_STATUS_URI: &str = "/0/private/Earn/DeallocateStatus";
const LIST_EARN_ALLOCATIONS_URI: &str = "/0/private/Earn/Allocations";
const LIST_EARN_STRATEGIES_URI: &str = "/0/private/Earn/Strategies";

pub use allocate_earn_funds::*;
pub use allocation_status::*;
pub use deallocate_earn_funds::*;
pub use deallocation_status::*;
pub use list_earn_allocations::*;
pub use list_earn_strategies::*;
