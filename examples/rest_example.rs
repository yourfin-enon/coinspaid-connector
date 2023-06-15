use coinspaid_connector::rest::rest_client::CoinsPaidRestClient;
use coinspaid_connector::rest::config::CoinsPaidConfig;

#[tokio::main]
async fn main() {
    let client = CoinsPaidRestClient::new_with_config(
        "test".to_owned(),
        "test".to_owned(),
        CoinsPaidConfig::test_env());
    get_balances(&client).await;
    take_address(&client).await;
    withdraw_crypto(&client).await;
}

async fn take_address(client: &CoinsPaidRestClient) {
    let address = client.take_address("BNB", "TEST").await;
    println!("take_address result: {address:?}");
}

async fn withdraw_crypto(client: &CoinsPaidRestClient) {
    let address = "2Mxsqy9d6LuW2VYQPsojmPWXaRznMQ7Nifr";
    let currency = "BTC";
    let foreign_id = "6a6c5ee305814ecdb98e9a2fa9c44123";
    let amount = "0.0003";
    let result = client.withdraw_crypto(address, currency, foreign_id, amount, None).await;
    println!("withdraw_crypto result: {result:?}");
}

async fn get_balances(client: &CoinsPaidRestClient) {
    let result = client.get_balances().await;
    println!("get_balances result: {result:?}");
}