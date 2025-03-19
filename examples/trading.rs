use kwi_kraken_rest::trading::*;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().unwrap();

    // token().await;
    // order_buy_limit().await;
    cancel_order().await;
    // amend_order().await;
}

#[allow(dead_code)]
async fn token() {
    let response = Token::get().await.unwrap();
    println!("response: {:?}", response);
}

#[allow(dead_code)]
async fn order_buy_limit() {
    let response = AddOrder::buy_limit("SOL/EUR", "0.02", "100.0", Some("solbl"))
        .await
        .unwrap();
    println!("response: {:?}", response);
}

#[allow(dead_code)]
async fn cancel_order() {
    let response = CancelOrder::cancel_txid("OEWPLD-VQLXP-44EJII")
        .await
        .unwrap();
    println!("response: {:?}", response);
}

#[allow(dead_code)]
async fn amend_order() {
    let response = AmendOrder::get(
        Some("OEWPLD-VQLXP-44EJII"),
        None,
        Some("0.02"),
        None,
        Some("90.0"),
        None,
        None,
    )
    .await
    .unwrap();
    println!("response: {:?}", response);
}
