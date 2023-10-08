use crate::utils;

pub async fn get_root_code() -> Result<String, String> {
    let time_result = utils::time::get_current_time().await;
    let time;
    match time_result {
        Ok(t) => time = t,
        Err(_) => return Err(String::from("cant get time")),
    }
    let code_result = utils::aes::encrypt_data(time);
    let code;
    match code_result {
        Ok(c) => code = c,
        Err(_) => return Err(String::from("cant get code")),
    }
    return Ok(code);
}

pub async fn get_verification_code(
    hash: String,
    root: String,
    nonce: String,
) -> Result<String, String> {
    let str = format!("{}{}", root, nonce);
    if utils::sha256::sha256(str) != hash {
        return Err(String::from("hash is error"));
    }
    let parse_result = utils::aes::decrypt_data(root);
    let parse;
    match parse_result {
        Ok(p) => parse = p,
        Err(_) => return Err(String::from("cant parse")),
    }
    let current_time_result = utils::time::get_current_time().await;
    match current_time_result {
        Ok(current_time) => {
            if current_time - parse > 10 {
                return Err(String::from("timeout"));
            } else {
                return Ok(String::from("6666"));
            }
        }
        Err(_) => return Err(String::from("cant get time")),
    }
}
