use ethers::prelude::*;
use ethers::utils::Anvil;
use eyre::Result;
use std::{convert::TryFrom, sync::Arc, time::Duration};
use dotenv::dotenv;
use std::env;

abigen!(SimpleStorage, "../abis/SimpleStorage.json");

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let anvil = Anvil::new().fork("http://127.0.0.1:8545").spawn();
    let provider =
        Provider::<Http>::try_from(anvil.endpoint())?.interval(Duration::from_millis(10u64));

    let wallet: LocalWallet = anvil.keys()[0].clone().into();
    let client = Arc::new(SignerMiddleware::new(provider, wallet.with_chain_id(anvil.chain_id())));

    println!("Contract Address {:?}", env::var("SIMPLE_STORAGE").unwrap());
    let contract_address = env::var("SIMPLE_STORAGE").unwrap().parse::<Address>()?;
    let simple_storage = SimpleStorage::new(contract_address, Arc::clone(&client));

    let mut value = simple_storage.get().call().await?;

    println!("value before is {:?}", value);

    simple_storage.set(U256::from(54321)).send().await?.await?;

    value = simple_storage.get().call().await?;

    println!("value after is {:?}", value);
    Ok(())
}
