use crate::handle_token;
use crate::CONFIG; 

pub async fn get_tokens(sign: &str, program: String) {
    let result = handle_token::get_transaction(sign, "jsonParsed", CONFIG.https_url.as_str())
        .await
        .unwrap();

    let instructions = handle_token::get_instructions_with_program_id(result, program);
    
    handle_token::token_show_info(instructions);
}