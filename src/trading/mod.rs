//! Module for user trading - also token.

use super::*;
use serde::{Deserialize, Serialize};

mod add_order;
mod token;
// Deprecated
mod amend_order;
mod edit_order;
// unimplemented
mod add_order_batch;
mod cancel_order;
// unimplemented
mod cancel_all_orders;
// unimplemented
mod cancel_order_batch;
// unimplemented
mod cancel_all_orders_after_x;

pub use add_order::*;
pub use add_order_batch::*;
pub use amend_order::*;
pub use cancel_all_orders::*;
pub use cancel_all_orders_after_x::*;
pub use cancel_order::*;
pub use cancel_order_batch::*;
pub use edit_order::*;
pub use token::*;

const ADD_ORDER_URI: &str = "/0/private/AddOrder";
const EDIT_ORDER_URI: &str = "/0/private/EditOrder";
const AMEND_ORDER_URI: &str = "/0/private/AmendOrder";
const WEBSOCKETS_TOKEN_URI: &str = "/0/private/GetWebSocketsToken";
const CANCEL_ORDER_URI: &str = "/0/private/CancelOrder";
// const CANCEL_ORDER_BATCH_URI: &str = "/0/private/CancelOrderBatch";
// const CANCEL_ALL_ORDERS_AFTER_URI: &str = "/0/private/CancelAllOrdersAfter";
// const CANCEL_ALL_URI: &str = "/0/private/CancelAll";
// const ADD_ORDER_BATCH_URI: &str = "/0/private/AddOrderBatch";
