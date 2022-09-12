use ethers::prelude::*;
use ethers::utils::Anvil;
use eyre::Result;
use std::{convert::TryFrom, sync::Arc, time::Duration};

abigen!(SimpleStorage, "abis/SimpleStorage.json");

#[tokio::main]
async fn main() -> Result<()> {
    let anvil = Anvil::new().fork("http://127.0.0.1:8545").spawn();
    let wallet: LocalWallet = anvil.keys()[0].clone().into();
    let provider =
        Provider::<Http>::try_from(anvil.endpoint())?.interval(Duration::from_millis(10u64));
    let client = Arc::new(SignerMiddleware::new(provider, wallet));

    let contract_address = "0x5FbDB2315678afecb367f032d93F642f64180aa3".parse::<Address>()?;
    let simple_storage = SimpleStorage::new(contract_address, Arc::clone(&client));

    let mut value = simple_storage.get().call().await?;

    println!("value is {:?}", value);

    simple_storage.set(U256::from(54321)).send().await?.await?;

    value = simple_storage.get().call().await?;

    println!("value is {:?}", value);
    Ok(())
}
