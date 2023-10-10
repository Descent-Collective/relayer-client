use crate::fetch_prices::utils;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
struct Response {
    USD: f32,
}

pub async fn get_crypto_compare_price() -> eyre::Result<(f32, u64)> {
    let client = reqwest::Client::new();
    let body = client
        .get("https://min-api.cryptocompare.com/data/price?fsym=USDC&tsyms=USD")
        .send()
        .await?
        .json::<Response>()
        .await?;

    // Calculate the Unix timestamp
    let unix_timestamp = utils::get_unix_timestamp();

    Ok((body.USD, unix_timestamp))
}
