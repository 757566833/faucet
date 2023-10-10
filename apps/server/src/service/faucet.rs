use crate::{
    error::{code_cache_error, timeout_error, ResponseError},
    utils, CODE_MAP,
};

use super::cache::{set_address_cool, set_email_cool, verify_address_cool, verify_email_cool};

pub async fn faucet(address: String, email: String, code: String) -> Result<String, ResponseError> {
    verify_email_cool(email.clone()).await?;
    verify_address_cool(address.clone()).await?;

    let map_option = CODE_MAP.get();
    if let Some(arc_map) = map_option {
        let map = arc_map.lock().await;
        let code_result = map.get(&email);
        if let Some(cache_code) = code_result {
            if *cache_code == code {
                let res = utils::eth::faucet(address.clone()).await?;
                set_email_cool(email).await?;
                set_address_cool(address).await?;
                return Ok(res);
            } else {
                return Err(timeout_error());
            }
        } else {
            return Err(timeout_error());
        }
    }

    return Err(code_cache_error());
}
