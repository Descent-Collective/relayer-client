use dotenv::dotenv;
use ethers::{
    abi::{encode, Token},
    prelude::abigen,
    providers::{Http, Provider},
    signers::{LocalWallet, Signer},
    types::{Address, Bytes, H256, U256},
    utils::{parse_units, ParseUnits},
};
use std::{str::FromStr, sync::Arc};

pub mod fetch_prices;

pub async fn run() -> eyre::Result<()> {
    dotenv().ok();

    // get all prices from fetch_prices module
    let prices_and_timestamps = fetch_prices::fetch_prices().await?;

    // print out all prices and timestamps
    prices_and_timestamps.iter().enumerate().for_each(|(i, v)| {
        println!("price at index {}: {:?}", i, v);
    });

    // define vectors to be used to store variables to parse to the function onchain
    let mut prices: Vec<U256> = Vec::new();
    let mut timestamps: Vec<U256> = Vec::new();
    let mut signatures: Vec<Bytes> = Vec::new();

    // filter the price and timestamp info and push to their respective vectors
    for (p, t) in prices_and_timestamps.iter() {
        // turn into a 6 fixed-point unsigned integer
        let first = parse_units(p, 6).expect("failed to convert float to ethers unit");

        // match to only take U256 types
        let second = match first {
            ParseUnits::U256(a) => a,
            ParseUnits::I256(_) => panic!("Negative values not allowed"),
        };

        // push to respective arrays
        prices.push(second);
        timestamps.push(U256::try_from(*t).expect("could not convert from u64 to timestamp"));
    }

    // define interface for update fn call
    abigen!(
        IOracleModule,
        r#"[
            function update(uint256[] calldata prices, uint256[] calldata timestamps, bytes[] calldata signatures) external
        ]"#
    );

    // define rpc url and oracle module address
    let rpc_url: String = std::env::var("RPC_URL").expect("RPC_URL must be set in your .env file");
    let oracle_module_address: Address = "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2".parse()?;

    // define provider to use from rpc url
    let provider = Provider::<Http>::try_from(rpc_url)?;
    let client = Arc::new(provider);

    // create an instance of oracle module
    let _oracle_module = IOracleModule::new(oracle_module_address, client);

    // wallet to sign tx. get private key from env, and revert if any unexpected stuff happens
    let wallet = LocalWallet::from_bytes(
        H256::from_str(
            &std::env::var("PRIVATE_KEY").expect("RPC_URL must be set in your .env file"),
        ).expect("invalid hex private key")
        .as_bytes(),
    )
    .unwrap();

    // loop through price and timestamp array, abi encode them concatenated together, hash and sign it as "\x19Ethereum Signed Message:\n" + len + encoded message
    for i in 0..prices.len() {
        // encode price + timestamp. where + is concatenation
        let message = encode(&[Token::Uint(prices[i]), Token::Uint(timestamps[i])]);

        // sign message with wallet
        let signature = wallet.sign_message(&message).await?;

        // convert to bytes and push to signatures vector
        signatures
            .push(Bytes::try_from(signature.to_vec()).expect("could not convert sig to bytes"));
    }

    // print values
    println!("addr: {:?}", wallet.address());
    println!("prices: {:?}", prices);
    println!("time: {:?}", timestamps);
    println!("sigs: {:?}", signatures);

    Ok(())
}
