use std::time::Duration;

use tokio::{task, time::sleep};

use crate::{
    utils::{self, time::get_current_time},
    CODE_COOL_DOWN, CODE_MAP,
};

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

async fn run_del(email: String) {
    sleep(Duration::from_millis(1000 * 60 * 5)).await;
    let map_option = CODE_MAP.get();
    if let Some(arc_map) = map_option {
        let mut map = arc_map.lock().await;
        map.remove(&email);
    }
}
async fn run_del_code_cool(email: String) {
    sleep(Duration::from_millis(1000 * 60)).await;
    let map_option = CODE_COOL_DOWN.get();
    if let Some(arc_map) = map_option {
        let mut map = arc_map.lock().await;
        map.remove(&email);
    }
}

pub async fn send_verification_code(
    hash: String,
    root: String,
    nonce: String,
    email: String,
) -> Result<String, String> {
    let code_cool_option = CODE_COOL_DOWN.get();
    let arc_code_cool;
    match code_cool_option {
        Some(p) => arc_code_cool = p,
        None => return Err(String::from("code cool error")),
    }
    let code_cool = arc_code_cool.lock().await;
    let time_option = code_cool.get(&email);
    if let Some(_) = time_option {
        return Err(String::from("code cool ing"));
    }
    let str = format!("{}{}", root, nonce);
    if utils::sha256::sha256(str) != hash && !hash.starts_with("0000") {
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
                let rand = utils::rand::rand_num().await;
                let map_option = CODE_MAP.get();
                let current_time_result = get_current_time().await;
                if let (Some(arc_map), Ok(current_time)) = (map_option, current_time_result) {
                    let mut map = arc_map.lock().await;
                    map.insert(email.clone(), rand.clone());
                    task::spawn(run_del(email.clone()));
                    let mut code_cool = arc_code_cool.lock().await;
                    code_cool.insert(email.clone(), current_time);
                    task::spawn(run_del_code_cool(email.clone()));

                    return utils::mail::send_email(email, rand).await;
                } else {
                    return Err(String::from("cache err"));
                }
            }
        }
        Err(_) => return Err(String::from("cant get time")),
    }
}
