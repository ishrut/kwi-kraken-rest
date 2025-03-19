//! This module contains functions for generating requests and helper macros.

use super::*;
use base64::Engine as _;
use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::Digest;
use std::time::{SystemTime, UNIX_EPOCH};

/// Basic structure of responses
#[derive(Debug, Serialize, Deserialize)]
pub struct KrakenResponse<T> {
    pub error: Option<Vec<String>>,
    pub result: T,
}

/// Gets a response at given url
pub async fn get_public_response(url: &str) -> Result<reqwest::Response, Error> {
    let response = match reqwest::get(url).await {
        Ok(response_data) => response_data,
        Err(e) => return Err(Error::Reqwest(e)),
    };
    Ok(response)
}

/// Request to public endpoints of kraken.com
/// Generic T is the json in which the data is deserialized to
pub async fn get_public_json<T>(url: &str) -> Result<KrakenResponse<T>, Error>
where
    T: for<'de> serde::Deserialize<'de>,
{
    let response = get_public_response(url).await?;
    let text_resp = match response.text().await {
        Ok(data) => data,
        Err(e) => return Err(Error::Reqwest(e)),
    };
    match serde_json::from_str(&text_resp) {
        Ok(data) => Ok(data),
        Err(e) => Err(Error::Decode(e, text_resp.to_string())),
    }
}

/// Makes a private request to kraken.com
/// This requires private keys to be set as environment variable
/// Returns a reqwest Response. see reqwest crate docs
pub async fn get_private_response(
    uri: &str,
    queries: Option<&str>,
) -> Result<reqwest::Response, Error> {
    let api_key = match std::env::var("API_KEY") {
        Ok(key) => key,
        Err(e) => return Err(Error::ApiKey(e)),
    };
    let api_sec = match std::env::var("API_SEC") {
        Ok(sec) => sec,
        Err(e) => return Err(Error::ApiSec(e)),
    };
    let nonce = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(res) => res.as_millis().to_string(),
        Err(e) => return Err(Error::Nonce(e)),
    };
    let body = match queries {
        Some(val) => format!("nonce={}&{}", &nonce, val),
        None => format!("nonce={}", &nonce),
    };
    let sha2_result = {
        let mut hasher = sha2::Sha256::default();
        hasher.update(&nonce);
        hasher.update(&body);
        hasher.finalize()
    };
    let hmac_sha_key = match base64::engine::general_purpose::STANDARD.decode(api_sec) {
        Ok(data) => data,
        Err(e) => return Err(Error::Base64DecodeError(e)),
    };
    let mut mac = match Hmac::<sha2::Sha512>::new_from_slice(hmac_sha_key.as_slice()) {
        Ok(data) => data,
        Err(e) => return Err(Error::MacError(e)),
    };
    mac.update(uri.as_bytes());
    mac.update(&sha2_result);
    let mac = mac.finalize().into_bytes();
    let signature = base64::engine::general_purpose::STANDARD.encode(mac);

    let url = format!("{}{}", BASE_URL, uri);

    let response = match reqwest::Client::new()
        .post(&url)
        .header("API-Key", &api_key)
        .header("API-Sign", &signature)
        .body(body)
        .send()
        .await
    {
        Ok(response) => response,
        Err(e) => return Err(Error::Reqwest(e)),
    };

    Ok(response)
}

/// Makes a private request to kraken.com
/// This requires private keys to be set as environment variable
/// Generic T is the struct in which the json will be deserialized to
pub async fn get_private_json<T>(
    uri: &str,
    queries: Option<&str>,
) -> Result<KrakenResponse<T>, Error>
where
    T: for<'de> serde::Deserialize<'de>,
{
    let response = get_private_response(uri, queries).await?;
    let text_resp = match response.text().await {
        Ok(data) => data,
        Err(e) => return Err(Error::Reqwest(e)),
    };
    match serde_json::from_str(&text_resp) {
        Ok(data) => Ok(data),
        Err(e) => Err(Error::Decode(e, text_resp.to_string())),
    }
}

/// This macro generates urls.
/// It helps in handling function parametres with mandatory and optional arguments
/// Usage: generate_url!(BASE_URL, URI; mandatory_params, ...; optional_params, ...);
#[macro_export]
macro_rules! build_url {
    ($base_url:expr, $uri:expr) => {{
        format!("{}{}?", $base_url, $uri)
    }};
    ($base_url:expr, $uri:expr; $($mand_params:expr),*;) => {{
        let mut mand_query_params = Vec::<String>::new();
        $(
            mand_query_params.push(format!("{}={}", stringify!($mand_params), $mand_params));
        )*
        let mand_queries = mand_query_params.join("&").to_string();
        format!("{}{}?{}", $base_url, $uri, mand_queries)
    }};
    ($base_url:expr, $uri:expr; ; $($opt_params:expr),*) => {{
        let mut opt_query_params = Vec::<String>::new();
        $(
            if let Some(val) = $opt_params {
                opt_query_params.push(format!("{}={}", stringify!($opt_params), val));
            }
        )*
        let opt_queries = opt_query_params.join("&").to_string();
        format!("{}{}?{}", $base_url, $uri, opt_queries)
    }};
    ($base_url:expr, $uri:expr; $($mand_params:expr),*; $($opt_params:expr),*) => {{
        let mut mand_query_params = Vec::<String>::new();
        $(
            mand_query_params.push(format!("{}={}", stringify!($mand_params), $mand_params));
        )*
        let mand_queries = mand_query_params.join("&").to_string();
        let mut opt_query_params = Vec::<String>::new();
        $(
            if let Some(val) = $opt_params {
                opt_query_params.push(format!("{}={}", stringify!($opt_params), val));
            }
        )*
        let opt_queries = opt_query_params.join("&").to_string();
        format!("{}{}?{}&{}", $base_url, $uri, mand_queries, opt_queries)
    }}
}

/// This macro generates queries for private requests
/// # Example
/// `build_queries! (mandatory_arg, ...; optional_arg, ...)`
#[macro_export]
macro_rules! build_queries {
    ($($mand_params:expr),*; ) => {{
        let mut mand_query_params = Vec::<String>::new();
        $(
            mand_query_params.push(format!("{}={}", stringify!($mand_params), $mand_params));
        )*
        mand_query_params.join("&").to_string()
    }};
    ( ; $($opt_params:expr),*) => {{
        let mut opt_query_params = Vec::<String>::new();
        $(
            if let Some(val) = $opt_params {
                opt_query_params.push(format!("{}={}", stringify!($opt_params), val));
            }
        )*
        opt_query_params.join("&").to_string()
    }};
    ($($mand_params:expr),*; $($opt_params:expr),*) => {{
        let mut mand_query_params = Vec::<String>::new();
        $(
            mand_query_params.push(format!("{}={}", stringify!($mand_params), $mand_params));
        )*
        let mand_queries = mand_query_params.join("&").to_string();
        let mut opt_query_params = Vec::<String>::new();
        $(
            if let Some(val) = $opt_params {
                opt_query_params.push(format!("{}={}", stringify!($opt_params), val));
            }
        )*
        let opt_queries = opt_query_params.join("&").to_string();
        format!("{}&{}", mand_queries, opt_queries)
    }}
}
