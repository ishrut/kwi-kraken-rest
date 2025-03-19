use kwi_kraken_rest::account_data::*;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().unwrap();

    // account_balance().await;
    // extended_balance().await;
    // closed_orders().await;
    // open_orders().await;
    // open_positions().await;
    // ledgers_info().await;
    // query_ledgers().await;
    // order_amends().await;
    // trade_volume().await;
    // trades_history().await;
    // trades_info().await;
    // request_export_report().await;
    export_report_status().await;
}

#[allow(dead_code)]
async fn account_balance() {
    let response = AccountBalance::get().await.unwrap();
    println!("response: {:?}", response);
}

#[allow(dead_code)]
async fn closed_orders() {
    let response = ClosedOrders::get(None, None, None, None, None, None, None, None)
        .await
        .unwrap();
    println!("response: {:?}", response);
}

//get some open orders to test
#[allow(dead_code)]
async fn open_orders() {
    let response = OpenOrders::get(None, None, None).await.unwrap();
    println!("response: {:?}", response);
}

//get some open positions to test
#[allow(dead_code)]
async fn open_positions() {
    let response = OpenPositions::get(None, None, None).await.unwrap();
    println!("response: {:?}", response);
}

#[allow(dead_code)]
async fn extended_balance() {
    let response = ExtendedBalance::get().await.unwrap();
    println!("response: {:?}", response);
}

#[allow(dead_code)]
async fn ledgers_info() {
    let response = LedgersInfo::get(None, None, None, None, None, None, None)
        .await
        .unwrap();
    println!("response: {:?}", response);
}

#[allow(dead_code)]
async fn query_ledgers() {
    let response = QueryLedgers::get(vec!["LCDHQD-7BVXR-CJOCXG"], None)
        .await
        .unwrap();
    println!("response: {:?}", response);
}

#[allow(dead_code)]
async fn order_amends() {
    let response = OrderAmends::get(Some("OEWPLD-VQLXP-44EJII")).await.unwrap();
    println!("response: {:?}", response);
}

#[allow(dead_code)]
async fn trade_volume() {
    let response = TradeVolume::get(Some(vec!["SOL/EUR", "BTC/USD"]))
        .await
        .unwrap();
    println!("response: {:?}", response);
}
#[allow(dead_code)]
async fn trades_history() {
    let response = TradesHistory::get(None, None, None, None, None, None, None)
        .await
        .unwrap();
    println!("response: {:?}", response);
}
#[allow(dead_code)]
async fn trades_info() {
    let response = TradesInfo::get("TT6SUD-E3QJ7-L3ZNPR", None).await.unwrap();
    println!("response: {:?}", response);
}
#[allow(dead_code)]
async fn request_export_report() {
    let response = RequestExportReport::get(
        "ledgers",
        "testing new purposes",
        Some("csv"),
        None,
        None,
        None,
    )
    .await
    .unwrap();
    println!("response: {:?}", response);
}

#[allow(dead_code)]
async fn export_report_status() {
    let response = ExportReportStatus::get("trades").await.unwrap();
    println!("response: {:?}", response);
}
