use crate::{utils, CODE_MAP};

pub async fn faucet() -> Result<String, String> {
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
