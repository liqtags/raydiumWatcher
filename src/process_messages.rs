use crate::CONFIG; 
use crate::token_handler::get_tokens;
use solana_client::rpc_response::{Response, RpcLogsResponse};

pub async fn process_message(response: Response<RpcLogsResponse>) {
    let value = response.value;
    
    for log in value.logs {
        if !log.contains(CONFIG.log_instruction.as_str()) {
            continue;
        }
        let signature_str = &value.signature;
        get_tokens(&signature_str, CONFIG.raydium_lpv4.to_string()).await;
    }
}