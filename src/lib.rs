//! kraken-rest library
//!
//! This library contains convenient functions to send requests to kraken REST API and deserialises the response.
mod error;

pub mod account_data;
pub mod earn;
pub mod funding;
pub mod market_data;
pub mod request;
pub mod subaccounts;
pub mod trading;

pub use crate::error::Error;
pub use request::*;

const BASE_URL: &str = "https://api.kraken.com";
