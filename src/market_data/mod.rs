//! Module for market data

use super::*;
use serde::{Deserialize, Serialize};

mod asset_info;
mod ohlc;
mod order_book;
mod recent_spreads;
mod recent_trades;
mod server_time;
mod system_status;
mod ticker;
mod tradable_asset_pairs;

pub use asset_info::*;
pub use ohlc::*;
pub use order_book::*;
pub use recent_spreads::*;
pub use recent_trades::*;
pub use server_time::*;
pub use system_status::*;
pub use ticker::*;
pub use tradable_asset_pairs::*;

const OHLC_URI: &str = "/0/public/OHLC";
const SERVER_TIME_URI: &str = "/0/public/Time";
const SYSTEM_STATUS_URI: &str = "/0/public/SystemStatus";
const ASSET_INFO_URI: &str = "/0/public/Assets";
const TRADADLE_ASSET_PAIRS_URI: &str = "/0/public/AssetPairs";
const TICKER_INFORMATION_URI: &str = "/0/public/Ticker";
const ORDER_BOOK_URI: &str = "/0/public/Depth";
const RECENT_TRADES_URI: &str = "/0/public/Trades";
const RECENT_SPREADS_URI: &str = "/0/public/Spread";
