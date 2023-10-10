use std::time::Duration;

use tokio::{task, time::sleep};

use crate::{
   
    utils, CODE_COOL_DOWN, FAUCET_ADDRESS_COOL_DOWN, FAUCET_EMAIL_COOL_DOWN, CODE_MAP, error::{ResponseError, code_cache_error, code_cooling_error, email_cool_cache_error, email_cooling_error, address_cool_cache_error, address_cooling_error},
};
/**
 * 验证发送验证码是否冷却
 */
pub async fn verify_code_cool(email: String) -> Result<(), ResponseError> {
    let code_cool_option = CODE_COOL_DOWN.get();
    let arc_code_cool;
    match code_cool_option {
        Some(p) => arc_code_cool = p,
        None => return Err(code_cache_error()),
    }
    let code_cool = arc_code_cool.lock().await;
    let time_option = code_cool.get(&email);
    if let Some(_) = time_option {
        return Err(code_cooling_error());
    }
    return Ok(());
}
/**
 * 验证领取水龙头的邮箱是否冷却
 */
pub async fn verify_email_cool(email: String) -> Result<(), ResponseError> {
    let cool_option = FAUCET_EMAIL_COOL_DOWN.get();
    let arc_cool;
    match cool_option {
        Some(p) => arc_cool = p,
        None => return Err(email_cool_cache_error()),
    }
    let code_cool = arc_cool.lock().await;
    let time_option = code_cool.get(&email);
    if let Some(_) = time_option {
        return Err(email_cooling_error());
    }
    return Ok(());
}
/**
 * 验证领取水龙头的地址是否冷却
 */
pub async fn verify_address_cool(address: String) -> Result<(), ResponseError> {
    let cool_option = FAUCET_ADDRESS_COOL_DOWN.get();
    let arc_cool;
    match cool_option {
        Some(p) => arc_cool = p,
        None => return Err(address_cool_cache_error()),
    }
    let code_cool = arc_cool.lock().await;
    let time_option = code_cool.get(&address);
    if let Some(_) = time_option {
        return Err(address_cooling_error());
    }
    return Ok(());
}

/**
 * 定时删除验内存验证码冷却
 */
async fn run_del_code_cool(email: String) {
    sleep(Duration::from_millis(1000 * 60 * 60 * 24)).await;
    let map_option = CODE_COOL_DOWN.get();
    if let Some(arc_map) = map_option {
        let mut map = arc_map.lock().await;
        map.remove(&email);
    }
}
/**
 * 定时删除领水的邮箱冷却
 */
async fn run_del_faucet_email_cool(to: String) {
    sleep(Duration::from_millis(1000 * 60 * 60 * 24)).await;
    let map_option = FAUCET_EMAIL_COOL_DOWN.get();
    if let Some(arc_map) = map_option {
        let mut map = arc_map.lock().await;
        map.remove(&to);
    }
}
/**
 * 定时删除领水的地址冷却
 */
async fn run_del_faucet_address_cool(to: String) {
    sleep(Duration::from_millis(1000 * 60 * 60 * 24)).await;
    let map_option = FAUCET_ADDRESS_COOL_DOWN.get();
    if let Some(arc_map) = map_option {
        let mut map = arc_map.lock().await;
        map.remove(&to);
    }
}

/**
 * 设置邮箱发送验证码发送冷却
 */
pub async fn set_code_cool(email: String) -> Result<(), ResponseError> {
    let code_cool_option = CODE_COOL_DOWN.get();
    let arc_code_cool;
    match code_cool_option {
        Some(p) => arc_code_cool = p,
        None => return Err(code_cache_error()),
    }
    let mut code_cool = arc_code_cool.lock().await;
    let current_time = utils::time::get_current_time().await?;
    code_cool.insert(email.clone(), current_time);
    task::spawn(run_del_code_cool(email.clone()));
    return Ok(());
}
/**
 * 设置领水的邮箱领水冷却
 */
pub async fn set_email_cool(email: String) -> Result<bool, ResponseError> {
    let arc_faucet_email_cool_option = FAUCET_EMAIL_COOL_DOWN.get();
    let arc_faucet_email_cool;
    match arc_faucet_email_cool_option {
        Some(a) => arc_faucet_email_cool = a,
        None => return Err(email_cool_cache_error()),
    }
    let current_time = utils::time::get_current_time().await?;
    let mut faucet_email_cool = arc_faucet_email_cool.lock().await;
    faucet_email_cool.insert(email.clone(), current_time);
    task::spawn(run_del_faucet_email_cool(email.clone()));
    return Ok(true);
}
/**
 * 设置领水的地址领水冷却
 */
pub async fn set_address_cool(address: String) -> Result<bool, ResponseError> {
    let arc_faucet_address_cool_option = FAUCET_ADDRESS_COOL_DOWN.get();
    let arc_faucet_address_cool;
    match arc_faucet_address_cool_option {
        Some(a) => arc_faucet_address_cool = a,
        None => return Err(address_cool_cache_error()),
    }
    let current_time = utils::time::get_current_time().await?;
    let mut faucet_address_cool = arc_faucet_address_cool.lock().await;
    faucet_address_cool.insert(address.clone(), current_time);
    task::spawn(run_del_faucet_address_cool(address.clone()));
    return Ok(true);
}

pub async fn del_code(email:String){
    let map_option = CODE_MAP.get();
    if let Some(arc_map) = map_option {
        let mut map = arc_map.lock().await;
        map.remove(&email);
    }
}
/**
 * 定时删除验证码
 */
async fn run_del(email: String) {
    sleep(Duration::from_millis(1000 * 60)).await;
    del_code(email).await
}
/**
 * 
 */
pub async fn set_code(email: String, code: String) -> Result<(), ResponseError> {
    let map_option = CODE_MAP.get();
    if let Some(arc_map) = map_option {
        let mut map = arc_map.lock().await;
        map.insert(email.clone(), code);
        task::spawn(run_del(email));
        Ok(())
    } else {
        return Err(code_cache_error());
    }
}
