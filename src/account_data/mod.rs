//! Module for account data

use super::*;
use serde::{Deserialize, Serialize};

const ACCOUNT_BALANCE_URI: &str = "/0/private/Balance";
const EXTENDED_BALANCE_URI: &str = "/0/private/BalanceEx";
const TRADE_BALANCE_URI: &str = "/0/private/TradeBalance";
const OPEN_ORDERS_URI: &str = "/0/private/OpenOrders";
const CLOSED_ORDERS_URI: &str = "/0/private/ClosedOrders";
const ORDERS_INFO_URI: &str = "/0/private/QueryOrders";
const TRADES_HISTORY_URI: &str = "/0/private/TradesHistory";
const TRADES_INFO_URI: &str = "/0/private/QueryTrades";
const OPEN_POSITIONS_URI: &str = "/0/private/OpenPositions";
const LEDGERS_INFO_URI: &str = "/0/private/Ledgers";
const QUERY_LEDGERS_URI: &str = "/0/private/QueryLedgers";
const TRADE_VOLUME_URI: &str = "/0/private/TradeVolume";
const ORDER_AMENDS_URI: &str = "/0/private/OrderAmends";
const ADD_EXPORT_URI: &str = "/0/private/AddExport";
const EXPORT_STATUS_URI: &str = "/0/private/ExportStatus";
const RETRIEVE_EXPORT_URI: &str = "/0/private/RetrieveExport";
const REMOVE_EXPORT_URI: &str = "/0/private/RemoveExport";

mod account_balance;
mod closed_orders;
mod extended_balance;
mod ledgers_info;
mod open_orders;
mod order_amends;
// untested
mod open_positions;
mod orders_info;
mod query_ledgers;
mod trade_balance;
mod trade_volume;
mod trades_history;
mod trades_info;
// untested
mod request_export_report;
// untested
mod get_export_report_status;
// untested
mod delete_export_report;
// untested
mod retrieve_data_export;

pub use account_balance::*;
pub use closed_orders::*;
pub use delete_export_report::*;
pub use extended_balance::*;
pub use get_export_report_status::*;
pub use ledgers_info::*;
pub use open_orders::*;
pub use open_positions::*;
pub use order_amends::*;
pub use orders_info::*;
pub use query_ledgers::*;
pub use request_export_report::*;
pub use retrieve_data_export::*;
pub use trade_balance::*;
pub use trade_volume::*;
pub use trades_history::*;
pub use trades_info::*;
