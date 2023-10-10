use crate::{
    error::{ResponseError, timeout_error, verify_hash_error},
    utils
};

use super::cache::{set_code, set_code_cool, verify_code_cool, verify_email_cool};

pub async fn get_root_code(email: String) -> Result<String, ResponseError> {
    verify_email_cool(email).await?;
    let time = utils::time::get_current_time().await?;
    return utils::aes::encrypt_data(time);
}


pub async fn send_verification_code(
    hash: String,
    root: String,
    nonce: String,
    email: String,
) -> Result<bool, ResponseError> {
    verify_email_cool(email.clone()).await?;

    verify_code_cool(email.clone()).await?;

    let str = format!("{}{}", root, nonce);
    if utils::sha256::sha256(str) != hash && !hash.starts_with("00000") {
        return Err(verify_hash_error());
    }
    let parse = utils::aes::decrypt_data(root)?;
    let current_time = utils::time::get_current_time().await?;
    if current_time - parse > 60 {
        return Err(timeout_error());
    } else {
        let rand = utils::rand::rand_num().await;

        set_code(email.clone(), rand.clone()).await?;

        set_code_cool(email.clone()).await?;

        return utils::mail::send_email(email.clone(), rand).await;
    }
}
