//! kraken-rest library
//!
//! This library contains convenient functions to send requests to kraken REST API and deserialises the response.
//! The submodules are organised in a similar fashion as the official documentation at: https://docs.kraken.com/api/
//!
//! # Example
//!
//! ```
//!    use kwi_kraken_rest::market_data::Ohlc;
//!
//!    #[tokio::main]
//!    async fn main() {
//!        let ohlc_data = Ohlc::get("SOLEUR", Some(1), None).await.unwrap();
//!        println!("ohlc data: {:#?}", ohlc);
//!    }
//! ```
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
