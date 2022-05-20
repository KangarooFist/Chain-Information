use dotenv;
use reqwest;
use tokio;
use serde_json::Result;
use crate::block_status::BlockStatus;
use crate::block_address::BlockAddress;
use crate::block_tx::BlockTx;

const ROOT: &str = "https://eth-blockbook.nownodes.io/api/";

#[tokio::main]
pub async fn send_request(url: &str) -> String {

    let client = reqwest::Client::new();
    client
        .get(url)
        .header("api-key", dotenv::var("API_KEY").expect("Could not find API Key")) //the expect makes Rust throw out a panic.
        .send()
        .await
        .expect("Failed - No response")
        .text()
        .await
        .expect("Failed - Can't convert payload")
}

pub fn block_status_request() -> BlockStatus{
    let response = send_request(ROOT);
    serde_json::from_str(&response).expect("Failed to Parse")
}

pub fn address_request(address: &str) -> BlockAddress{
    let response = send_request(&[ROOT, "/v2/address/", &address].join(""));
    serde_json::from_str(&response).expect("Failed to Parse")
}

pub fn tx_request(transaction: &str) -> BlockTx{
    let response = send_request(&[ROOT, "/v2/tx/", &transaction].join(""));
    serde_json::from_str(&response).expect("Failed to Parse JSON")
}