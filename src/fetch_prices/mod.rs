mod get_crypto_compare_price;
pub mod utils;

// fetches prices from all data sources added here and returns to lib.rs for filtering, encoding, signing and updating
pub async fn fetch_prices() -> eyre::Result<Vec<(f32, u64)>> {
    // initialize return vector
    let mut prices_and_timestamp: Vec<(f32, u64)> = Vec::new();

    // push crypto compare price to vector
    prices_and_timestamp.push(get_crypto_compare_price::get_crypto_compare_price().await?);

    // return it
    Ok(prices_and_timestamp)
}
