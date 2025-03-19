use kwi_kraken_rest::market_data::*;

#[tokio::main]
async fn main() {
    // server_time().await;
    // system_status().await;
    // asset_info().await;
    // ohlc().await;
    // order_book().await;
    // recent_spreads().await;
    // recent_trades().await;
    tradable_assets().await;
}

#[allow(dead_code)]
async fn server_time() {
    let response = ServerTime::get().await.unwrap();
    println!("response: {:?}", response);
}

#[allow(dead_code)]
async fn system_status() {
    let response = SystemStatus::get().await.unwrap();
    println!("response: {:?}", response);
}

#[allow(dead_code)]
async fn asset_info() {
    let response = AssetInfo::get(Some("SOL"), Some("currency")).await.unwrap();
    println!("response: {:?}", response);
}

#[allow(dead_code)]
async fn ohlc() {
    let response = Ohlc::get("SOL/EUR", Some("15"), None).await.unwrap();
    println!("response: {:?}", response);
}

#[allow(dead_code)]
async fn order_book() {
    let response = OrderBook::get("SOL/EUR", Some("1")).await.unwrap();
    println!("response: {:?}", response);
}

#[allow(dead_code)]
async fn recent_spreads() {
    let response = RecentSpreads::get("SOL/EUR", None).await.unwrap();
    println!("response: {:?}", response);
}

#[allow(dead_code)]
async fn recent_trades() {
    let response = RecentTrades::get("SOL/EUR", None, None).await.unwrap();
    println!("response: {:?}", response);
}

#[allow(dead_code)]
async fn tradable_assets() {
    let response = TradableAssetPairs::get(Some(vec!["SOL/EUR"]), None, None)
        .await
        .unwrap();
    println!("response: {:?}", response);
}
