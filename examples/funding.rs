use kwi_kraken_rest::funding::*;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().unwrap();

    // deposit_methods().await;
    // deposit_addresses().await;
}

#[allow(dead_code)]
async fn deposit_methods() {
    let resp = DepositMethods::get("SOL", None).await.unwrap();
    println!("{:?}", resp);
}
#[allow(dead_code)]
async fn deposit_addresses() {
    let resp = DepositAddresses::get("XBT", "Bitcoin", Some(true), None)
        .await
        .unwrap();
    println!("{:?}", resp);
}
