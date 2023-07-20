// use std::{path::PathBuf, str::FromStr};

// use env_logger::Env;
// use ethers::{types::Address, utils};
// use eyre::Result;
// use helios::{config::networks::Network, prelude::*};

use beerus_core::{
    config::Config,
    lightclient::{
        beerus::BeerusLightClient, ethereum::helios_lightclient::HeliosLightClient,
        starknet::StarkNetLightClientImpl,
    },
};
// use beerus_rpc::BeerusRpc;
use env_logger::Env;
use log::{error, info};
// use std::process::exit;
use eyre::Result;
use std::env;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<()> {
    // Set helios client
    // Set the network to mainnet
    env::set_var("ETHEREUM_NETWORK", "mainnet");
    // Set the ethereum consensus rpc url
    env::set_var(
        "ETHEREUM_CONSENSUS_RPC_URL",
        "https://www.lightclientdata.org",
    );
    // Set the ethereum execution rpc url
    env::set_var(
        "ETHEREUM_EXECUTION_RPC_URL",
        "https://eth-mainnet.g.alchemy.com/v2/gVuEP7oUExvEzNWpUf5L3Gc-BC1NYqSi",
    );
    // Set the data dir
    env::set_var("DATA_DIR", "~/.beerus/tmp");
    // Set the checkpoint to the last known checkpoint
    env::set_var(
        "ETHEREUM_CHECKPOINT",
        "0x419347336a423e0ad7ef3a1e8c0ca95f8b4f525122eea0178a11f1527ba38c0f",
    );

    // Set the Starknet rpc url
    env::set_var(
        "STARKNET_RPC_URL",
        "https://starknet-mainnet.infura.io/v3/b0eeb1b7e0704005ae91bae55bf527w9c",
    );
    // Set Beerus rpc address
    env::set_var("BEERUS_RPC_ADDR", "0.0.0.0:3030");

    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let config = Config::from_env();

    let ethereum_lightclient = HeliosLightClient::new(config.clone()).await?;
    let starknet_lightclient = StarkNetLightClientImpl::new(&config)?;
    let mut beerus = BeerusLightClient::new(
        config.clone(),
        Box::new(ethereum_lightclient),
        Box::new(starknet_lightclient),
    );

    println!("Constructed Beerus client!");
    Ok(())
}
