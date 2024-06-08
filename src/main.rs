use anyhow::Result;
use dotenv::dotenv;
use futures::prelude::*;
use solana_client::{
    nonblocking::pubsub_client::PubsubClient,
    rpc_config::{RpcTransactionLogsConfig, RpcTransactionLogsFilter},
    rpc_response::{Response, RpcLogsResponse},
};
use solana_sdk::commitment_config::CommitmentConfig;
pub mod config;
pub mod handle_token;
mod token_handler;
mod process_messages;
pub mod utils;
use config::Configuration;
use process_messages::process_message;

lazy_static::lazy_static! {
    static ref CONFIG: Configuration = Configuration::new();
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let logger = utils::Logger::new("Setup".to_string());

    logger.log(format!(
        "Solana RPC websocket: {:?}",
        CONFIG.wss_url.as_str()
    ));
    logger.log(format!("Solana RPC http: {:?}", CONFIG.https_url.as_str()));
    logger.log(format!(
        "Log instruction: {:?}",
        CONFIG.log_instruction.as_str()
    ));

    let ws_client = PubsubClient::new(CONFIG.wss_url.as_str()).await?;
    let (mut stream, _) = ws_client
        .logs_subscribe(
            RpcTransactionLogsFilter::Mentions(vec![CONFIG.raydium_lpv4.to_string()]),
            RpcTransactionLogsConfig {
                commitment: Some(CommitmentConfig::finalized()),
            },
        )
        .await?;

    logger.log("Subscribed to Raydium Liquidity Pool".to_string());
    loop {
        let response: Response<RpcLogsResponse> = stream.next().await.unwrap();
        process_message(response).await;
    }
}