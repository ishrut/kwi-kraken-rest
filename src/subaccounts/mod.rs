//! Module for subaccount operations

use super::*;
use serde::{Deserialize, Serialize};

// untested
mod account_transfer;
// untested
mod create_subaccount;

const ACCOUNT_TRANSFER_URI: &str = "/0/private/CreateSubaccount";
const CREATE_ACCOUNT_URI: &str = "/0/private/AccountTransfer";

pub use account_transfer::*;
pub use create_subaccount::*;
