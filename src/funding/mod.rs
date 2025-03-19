//! Module for funding functions

use super::*;
use serde::{Deserialize, Serialize};

mod deposit_addresses;
mod deposit_methods;
// untested
mod recent_deposits_status;
// untested
mod recent_withdrawals_status;
// untested
mod wallet_transfer;
// untested
mod withdraw_funds;
// untested
mod withdrawal_addresses;
// untested
mod withdrawal_cancellation;
// untested
mod withdrawal_information;
// untested
mod withdrawal_methods;

const DEPOSIT_METHODS_URI: &str = "/0/private/DepositMethods";
const DEPOSIT_ADDRESSES_URI: &str = "/0/private/DepositAddresses";
const DEPOSIT_STATUS_URI: &str = "/0/private/DepositStatus";
const WITHDRAW_METHODS_URI: &str = "/0/private/WithdrawMethods";
const WITHDRAW_ADDRESSES_URI: &str = "/0/private/WithdrawAddresses";
const WITHDRAW_INFO_URI: &str = "/0/private/WithdrawInfo";
const WITHDRAW_FUNDS_URI: &str = "/0/private/Withdraw";
const WITHDRAW_STATUS_URI: &str = "/0/private/WithdrawStatus";
const WITHDRAW_CANCEL_URI: &str = "/0/private/WithdrawCancel";
const WALLET_TRANSFER_URI: &str = "/0/private/WalletTransfer";

pub use deposit_addresses::*;
pub use deposit_methods::*;
pub use recent_deposits_status::*;
pub use recent_withdrawals_status::*;
pub use wallet_transfer::*;
pub use withdraw_funds::*;
pub use withdrawal_addresses::*;
pub use withdrawal_cancellation::*;
pub use withdrawal_information::*;
pub use withdrawal_methods::*;
