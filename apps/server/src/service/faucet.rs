use crate::{utils, CODE_MAP};

pub async fn faucet(address: String, email: String, code: String) -> Result<String, String> {
    let map_option = CODE_MAP.get();
    if let Some(arc_map) = map_option {
        let map = arc_map.lock().await;
        let code_result = map.get(&email);
        if let Some(cache_code) = code_result {
            if *cache_code == code {
                return utils::eth::faucet(address).await;
            }
        }
    }

    return Err(String::from("cache err"));
}
