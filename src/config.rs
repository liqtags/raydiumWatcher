pub struct Configuration {
    pub wss_url: String,
    pub https_url: String,
    pub log_instruction: String,
    pub raydium_lpv4: String,
}

impl Configuration {
    pub fn new() -> Self {
        let wss_url = std::env::var("SOL_WSS")
        .unwrap_or_else(|_| "wss://api.mainnet-beta.solana.com".to_string());
        let https_url = std::env::var("SOL_HTTPS")
        .unwrap_or_else(|_| "https://api.mainnet-beta.solana.com".to_string());
        let log_instruction =
            std::env::var("LOG_INSTRUCTION").unwrap_or_else(|_| "initialize2".to_string());
        let raydium_lpv4 = std::env::var("RAYDIUM_LPV4")
            .unwrap_or_else(|_| "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8".to_string());

        Configuration {
            wss_url,
            https_url,
            log_instruction,
            raydium_lpv4,
        }
    }
}
