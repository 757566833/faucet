use std::{env, str::FromStr};

use crate::{
    constant::{CHAIN_ID, FAUCET_NUMBER, GAS_MULTIPLE, PRIVATE_KEY, RPC},
    error::{insufficient_account_balance_error, ResponseError},
    utils::{rand::rand_num, sign},
};

use hyper::Client;
use hyper_tls::HttpsConnector;
use k256::{elliptic_curve::sec1::ToEncodedPoint, AffinePoint, SecretKey};
use serde::Deserialize;
use sha3::{Digest, Keccak256};

pub async fn faucet(to_str: String) -> Result<String, ResponseError> {
    let rpc = env::var(RPC)?;
    let chain_id = env::var(CHAIN_ID)?;
    let chain_id_bg = num_bigint::BigUint::from_str(&chain_id)?;
    let private_key = env::var(PRIVATE_KEY)?;
    let faucet_str = env::var(FAUCET_NUMBER)?;
    let gas_multiple = env::var(GAS_MULTIPLE)?;
    let multiple = num_bigint::BigUint::from_str(&gas_multiple)?;
    let to = hex::decode(to_str.clone())?;
    let faucet_number = num_bigint::BigUint::from_str(&faucet_str)?;

    let keys = private_key.split(",");
    let mut target_private_option = None;
    let mut target_address_option = None;

    for key in keys {
        let address = get_address_by_private_key(&hex::decode(key)?)?;
        let balance = get_balance(&rpc, &address).await?;
        if balance.ge(&faucet_number) {
            target_private_option = Some(hex::decode(key)?);
            target_address_option = Some(address);
            break;
        }
    }

    if let (Some(private_key), Some(from)) = (target_private_option, target_address_option) {
        let latest_base_fee_per_gas = get_latest_base_fee_per_gas().await;

        let nonce = get_nonce(&rpc, from.as_slice()).await?;
        // return Ok(String::from("value"));

        let tx;
        if let Some(base_fee_per_gas) = latest_base_fee_per_gas {
            let max_priority_fee_per_gas = get_max_priority_fee_per_gas(&rpc).await?;

            let gas_limit = estimate_gas(
                &rpc,
                EstimateGasTransactionRequest::Eip1559(EstimateGasEip1559transaction {
                    from,
                    to: to.clone(),
                    max_fee_per_gas: (&max_priority_fee_per_gas + &base_fee_per_gas) * &multiple,
                    max_priority_fee_per_gas: &max_priority_fee_per_gas * &multiple,
                    value: faucet_number.clone(),
                }),
            )
            .await?;
            tx = sign::sign(
                private_key.as_slice(),
                sign::SignTransactionRequest::Eip1559(sign::SignEip1559transaction {
                    chain_id: chain_id_bg,
                    to,
                    nonce,
                    gas_limit: gas_limit * &multiple,
                    max_fee_per_gas: (&max_priority_fee_per_gas + &base_fee_per_gas) * &multiple,
                    max_priority_fee_per_gas: &max_priority_fee_per_gas * &multiple,
                    value: faucet_number,
                }),
            )?;
        } else {
            let gas_price = get_gas_price(&rpc).await?;
            let gas_limit = estimate_gas(
                &rpc,
                EstimateGasTransactionRequest::Legacy(EstimateGasLegacyTransaction {
                    from,
                    to: to.clone(),
                    gas_price: &gas_price * &multiple,
                    value: faucet_number.clone(),
                }),
            )
            .await?;
            tx = sign::sign(
                private_key.as_slice(),
                sign::SignTransactionRequest::Legacy(sign::SignLegacyTransaction {
                    chain_id: chain_id_bg,
                    to,
                    nonce,
                    gas_limit: &gas_limit * &multiple,
                    gas_price: &gas_price * &multiple,
                    value: faucet_number,
                }),
            )?;
        }
        let result = send_raw_transaction(&rpc, tx.as_slice()).await?;
        return Ok(result);
    } else {
        return Err(insufficient_account_balance_error());
    }
    // return Ok(String::from("value"))

    // if let Ok(faucet_number) = faucet_number_result {
    //     // Wallet::new(rng)
    // } else {
    //     return Err(Box::new(EnvError));
    // }
    // if let (Ok(rpc), Ok(private_key), Ok(faucet_str), Ok(gas_multiple)) = (
    //     rpc_result,
    //     private_key_result,
    //     faucet_number_result,
    //     gas_multiple_result,
    // ) {

    // } else {
    //     return Err(env_error());
    // }
}

fn get_address_by_private_key(private_key: &[u8]) -> Result<Vec<u8>, ResponseError> {
    // let p;
    // if private_key.starts_with("0x") {
    //     p = String::from(&private_key[2..]);
    // } else {
    //     p = private_key
    // }
    // let private_key_bytes = hex::decode(private_key)?;
    let mut private_key_array: [u8; 32] = [0; 32];
    private_key_array.copy_from_slice(private_key);
    let secret_key = SecretKey::from_slice(&private_key_array)?;

    // 压缩公钥匙
    // let compress_public_key =  secret_key.public_key().to_sec1_bytes().to_vec();

    // 压缩公钥匙 hex
    // let compress_public_key_hex = hex::encode(public_key.to_sec1_bytes().to_vec());

    let affine_point = AffinePoint::from(secret_key.public_key());
    // 非压缩公钥
    let un_comporess_affine_point = affine_point.to_encoded_point(false).to_bytes();

    // let un_comporess_affine_point_hex = hex::encode(un_comporess_affine_point.clone());

    // println!(" new {:?}", un_comporess_affine_point_hex);
    let mut hasher = Keccak256::new();
    // 去掉开头的 02、03、04
    hasher.update(&un_comporess_affine_point[1..]);
    let address_vec: Vec<u8> = hasher.finalize().to_vec();

    let address = address_vec[12..].to_vec();
    return Ok(address);
    // 0x02f8728009843b9aca00843b9aca0782520894a303721f08b85af1fdf7c57152b9e31d4bca397b884563918244f4000080c080a0859d562fb534c5288e8b328ea7c199a815092184f4c963c1e9e4d2b23469567ba076ed5614f8afb4c6d092f98cf915253b219ee7aeb4ed5184acccb0da39e2df76
}

// pub fn address_to_hex(address: Vec<u8>) -> String {
//     return hex::encode(address);
// }
pub async fn get_latest_base_fee_per_gas() -> Option<num_bigint::BigUint> {
    let rpc_result = env::var(RPC);
    if let Ok(rpc) = rpc_result {
        let latest_result = get_latest_block(&rpc).await;
        let latest;

        match latest_result {
            Ok(b) => latest = b,
            Err(_) => return None,
        }
        if let Some(base_fee_str) = latest.result.base_fee_per_gas {
            // let base_fee = num_bigint::BigUint::from_str(&base_fee_str);
            let base_fee = num_bigint::BigUint::from_str(&base_fee_str[2..]);
            if let Ok(fee) = base_fee {
                return Some(fee);
            } else {
                return None;
            }
        } else {
            return None;
        }
    }
    return None;
}

#[derive(Debug, Deserialize)]
pub struct EthResponse<T> {
    result: T,
}

pub async fn get_balance(rpc: &str, address: &[u8]) -> Result<num_bigint::BigUint, ResponseError> {
    let request = hyper::Request::builder()
        .uri(rpc)
        .header(hyper::header::CONTENT_TYPE, "application/json")
        .method(hyper::Method::POST)
        .body(hyper::Body::from(
            serde_json::json!({
                "id":  rand_num(),
                "jsonrpc": "2.0",
                "method": "eth_getBalance",
                "params": [format!("0x{}",hex::encode(address)), "latest"]
            })
            .to_string(),
        ))?;
    let resp;
    if rpc.starts_with("https") {
        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, hyper::Body>(https);
        resp = client.request(request).await?;
    } else {
        let client = hyper::Client::new();
        resp = client.request(request).await?;
    }
    let body_bytes = hyper::body::to_bytes(resp.into_body()).await?;
    let body_str = String::from_utf8(body_bytes.to_vec())?;
    let result: EthResponse<String> = serde_json::from_str(&body_str)?;
    return hex_to_big_num(result.result);
}

// pub async fn get_block_number(rpc: &str) -> Result<num_bigint::BigUint, ResponseError> {
//     // let id= rand_num();
//     // println!("{}", id);
//     let request = hyper::Request::builder()
//         .uri(rpc.clone())
//         .header(hyper::header::CONTENT_TYPE, "application/json")
//         .method(hyper::Method::POST)
//         .body(hyper::Body::from(
//             serde_json::json!({
//                 "id":  rand_num(),
//                 "jsonrpc": "2.0",
//                 "method": "eth_blockNumber",
//                 "params": []
//             })
//             .to_string(),
//         ))?;
//     let resp;
//     if rpc.starts_with("https") {
//         let https = HttpsConnector::new();
//         let client = Client::builder().build::<_, hyper::Body>(https);
//         resp = client.request(request).await?;
//     } else {
//         let client = hyper::Client::new();
//         resp = client.request(request).await?;
//     }
//     let body_bytes = hyper::body::to_bytes(resp.into_body()).await?;
//     let body_str = String::from_utf8(body_bytes.to_vec())?;
//     let result: EthResponse<String> = serde_json::from_str(&body_str)?;
//     return hex_to_big_num(result.result);
// }

pub async fn get_nonce(rpc: &str, address: &[u8]) -> Result<num_bigint::BigUint, ResponseError> {
    // let id= rand_num();
    // println!("{}", id);
    let hex = hex::encode(address);
    let address = format!("0x{}", hex);
    let request = hyper::Request::builder()
        .uri(rpc)
        .header(hyper::header::CONTENT_TYPE, "application/json")
        .method(hyper::Method::POST)
        .body(hyper::Body::from(
            serde_json::json!({
                "id":  rand_num(),
                "jsonrpc": "2.0",
                "method": "eth_getTransactionCount",
                "params": [address,"latest"]
            })
            .to_string(),
        ))?;
    let resp;
    if rpc.starts_with("https") {
        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, hyper::Body>(https);
        resp = client.request(request).await?;
    } else {
        let client = hyper::Client::new();
        resp = client.request(request).await?;
    }
    let body_bytes = hyper::body::to_bytes(resp.into_body()).await?;
    let body_str = String::from_utf8(body_bytes.to_vec())?;
    let result: EthResponse<String> = serde_json::from_str(&body_str)?;

    return hex_to_big_num(result.result);
}

pub async fn send_raw_transaction(rpc: &str, raw: &[u8]) -> Result<String, ResponseError> {
    // let id= rand_num();
    // println!("{}", id);
    let hex = hex::encode(raw);
    let tx = format!("0x{}", hex);
    let request = hyper::Request::builder()
        .uri(rpc)
        .header(hyper::header::CONTENT_TYPE, "application/json")
        .method(hyper::Method::POST)
        .body(hyper::Body::from(
            serde_json::json!({
                "id":  rand_num(),
                "jsonrpc": "2.0",
                "method": "eth_sendRawTransaction",
                "params": [tx]
            })
            .to_string(),
        ))?;
    let resp;
    if rpc.starts_with("https") {
        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, hyper::Body>(https);
        resp = client.request(request).await?;
    } else {
        let client = hyper::Client::new();
        resp = client.request(request).await?;
    }
    let body_bytes = hyper::body::to_bytes(resp.into_body()).await?;
    let body_str = String::from_utf8(body_bytes.to_vec())?;
    let result: EthResponse<String> = serde_json::from_str(&body_str)?;
    return Ok(result.result);
}

#[derive(Debug, Deserialize)]
pub struct EthBlock {
    #[serde(rename = "baseFeePerGas")]
    base_fee_per_gas: Option<String>,
}

pub async fn get_latest_block(rpc: &str) -> Result<EthResponse<EthBlock>, ResponseError> {
    // let id= rand_num();
    // println!("{}", id);
    let request = hyper::Request::builder()
        .uri(rpc)
        .header(hyper::header::CONTENT_TYPE, "application/json")
        .method(hyper::Method::POST)
        .body(hyper::Body::from(
            serde_json::json!({
                "id":  rand_num(),
                "jsonrpc": "2.0",
                "method": "eth_getBlockByNumber",
                "params": ["latest",false]
            })
            .to_string(),
        ))?;
    let resp;
    if rpc.starts_with("https") {
        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, hyper::Body>(https);
        resp = client.request(request).await?;
    } else {
        let client = hyper::Client::new();
        resp = client.request(request).await?;
    }
    let body_bytes = hyper::body::to_bytes(resp.into_body()).await?;
    let body_str = String::from_utf8(body_bytes.to_vec())?;
    let result: EthResponse<EthBlock> = serde_json::from_str(&body_str)?;
    return Ok(result);
}

pub async fn get_max_priority_fee_per_gas(rpc: &str) -> Result<num_bigint::BigUint, ResponseError> {
    // let id= rand_num();
    // println!("{}", id);
    let request = hyper::Request::builder()
        .uri(rpc)
        .header(hyper::header::CONTENT_TYPE, "application/json")
        .method(hyper::Method::POST)
        .body(hyper::Body::from(
            serde_json::json!({
                "id":  rand_num(),
                "jsonrpc": "2.0",
                "method": "eth_maxPriorityFeePerGas",
                "params": []
            })
            .to_string(),
        ))?;
    let resp;
    if rpc.starts_with("https") {
        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, hyper::Body>(https);
        resp = client.request(request).await?;
    } else {
        let client = hyper::Client::new();
        resp = client.request(request).await?;
    }
    let body_bytes = hyper::body::to_bytes(resp.into_body()).await?;
    let body_str = String::from_utf8(body_bytes.to_vec())?;
    let result: EthResponse<String> = serde_json::from_str(&body_str)?;
    return Ok(hex_to_big_num(result.result)?);
}

pub async fn get_gas_price(rpc: &str) -> Result<num_bigint::BigUint, ResponseError> {
    // let id= rand_num();
    // println!("{}", id);
    let request = hyper::Request::builder()
        .uri(rpc)
        .header(hyper::header::CONTENT_TYPE, "application/json")
        .method(hyper::Method::POST)
        .body(hyper::Body::from(
            serde_json::json!({
                "id":  rand_num(),
                "jsonrpc": "2.0",
                "method": "eth_gasPrice",
                "params": []
            })
            .to_string(),
        ))?;
    let resp;
    if rpc.starts_with("https") {
        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, hyper::Body>(https);
        resp = client.request(request).await?;
    } else {
        let client = hyper::Client::new();
        resp = client.request(request).await?;
    }
    let body_bytes = hyper::body::to_bytes(resp.into_body()).await?;
    let body_str = String::from_utf8(body_bytes.to_vec())?;
    let result: EthResponse<String> = serde_json::from_str(&body_str)?;
    return Ok(hex_to_big_num(result.result)?);
}

#[derive(Debug)]
pub struct EstimateGasLegacyTransaction {
    pub to: Vec<u8>,
    pub from: Vec<u8>,
    pub gas_price: num_bigint::BigUint,
    pub value: num_bigint::BigUint,
}
#[derive(Debug)]
pub struct EstimateGasEip1559transaction {
    pub to: Vec<u8>,
    pub from: Vec<u8>,
    pub max_fee_per_gas: num_bigint::BigUint,
    pub max_priority_fee_per_gas: num_bigint::BigUint,
    pub value: num_bigint::BigUint,
}
pub enum EstimateGasTransactionRequest {
    Legacy(EstimateGasLegacyTransaction),
    Eip1559(EstimateGasEip1559transaction),
}

async fn estimate_gas(
    rpc: &str,
    request: EstimateGasTransactionRequest,
) -> Result<num_bigint::BigUint, ResponseError> {
    // let id= rand_num();
    // println!("{}", id);

    let params;

    match request {
        EstimateGasTransactionRequest::Legacy(legacy_transaction) => {
            let from = hex::encode(legacy_transaction.from);
            let to = hex::encode(legacy_transaction.to);
            let gas_price = legacy_transaction.gas_price.to_str_radix(16);
            let value = legacy_transaction.value.to_str_radix(16);
            params = serde_json::json!({
                "type":"0x0",
                "gasPrice":format!("0x{}", gas_price),
                "value":format!("0x{}", value),
                "from":format!("0x{}", from),
                "to":format!("0x{}", to),
            })
        }
        EstimateGasTransactionRequest::Eip1559(eip1559_transaction) => {
            let from = hex::encode(eip1559_transaction.from);
            let to = hex::encode(eip1559_transaction.to);
            let max_priority_fee_per_gas = eip1559_transaction
                .max_priority_fee_per_gas
                .to_str_radix(16);
            let max_fee_per_gas = eip1559_transaction.max_fee_per_gas.to_str_radix(16);
            let value = eip1559_transaction.value.to_str_radix(16);
            params = serde_json::json!({
                "type":"0x2",
                "maxFeePerGas":format!("0x{}", max_fee_per_gas),
                "maxPriorityFeePerGas":format!("0x{}", max_priority_fee_per_gas),
                "value":format!("0x{}", value),
                "from":format!("0x{}", from),
                "to":format!("0x{}", to),
            })
        }
    }
    let request = hyper::Request::builder()
        .uri(rpc)
        .header(hyper::header::CONTENT_TYPE, "application/json")
        .method(hyper::Method::POST)
        .body(hyper::Body::from(
            serde_json::json!({
                "id":  rand_num(),
                "jsonrpc": "2.0",
                "method": "eth_estimateGas",
                "params": [params]
            })
            .to_string(),
        ))?;
    let resp;
    if rpc.starts_with("https") {
        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, hyper::Body>(https);
        resp = client.request(request).await?;
    } else {
        let client = hyper::Client::new();
        resp = client.request(request).await?;
    }
    let body_bytes = hyper::body::to_bytes(resp.into_body()).await?;
    let body_str = String::from_utf8(body_bytes.to_vec())?;
    let result: EthResponse<String> = serde_json::from_str(&body_str)?;
    return Ok(hex_to_big_num(result.result)?);
}

fn hex_to_big_num(num: String) -> Result<num_bigint::BigUint, ResponseError> {
    let mut hex = num.clone();
    if hex.starts_with("0x") {
        hex = (hex[2..]).to_string();
    }
    if hex.len() % 2 != 0 {
        hex = format!("0{}", hex)
    }
    let vec = hex::decode(hex)?;
    return Ok(num_bigint::BigUint::from_bytes_be(&vec));
}
#[cfg(test)]
mod tests {
    use std::{env, str::FromStr};

    use crate::{
        constant::{FAUCET_NUMBER, GAS_MULTIPLE, RPC},
        utils::{
            self,
            eth::{
                estimate_gas, get_address_by_private_key, get_balance, get_gas_price,
                get_latest_base_fee_per_gas, get_latest_block, get_max_priority_fee_per_gas,
                get_nonce, hex_to_big_num, EstimateGasEip1559transaction,
                EstimateGasLegacyTransaction,
            },
        },
    };
    use num_bigint::BigUint;

    #[test]
    fn test_address() {
        let vec = get_address_by_private_key(
            &hex::decode("ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80").unwrap(),
        )
        .unwrap();
        assert_eq!(hex::encode(vec), "f39fd6e51aad88f6f4ce6ab8827279cfffb92266");
    }
    #[tokio::test]
    async fn test_gas_price() {
        dotenv::dotenv().ok();
        let rpc = env::var(RPC).unwrap();
        let gas_price = get_gas_price(&rpc).await.unwrap();
        println!("gas_price:{}", gas_price.to_string())
    }

    #[tokio::test]
    async fn test_block() {
        dotenv::dotenv().ok();
        let bool = get_latest_base_fee_per_gas().await;
        println!("{:?}", bool)
    }

    #[tokio::test]
    async fn test_u256() {
        dotenv::dotenv().ok();
        let gas_multiple = env::var(GAS_MULTIPLE).unwrap();
        let multiple = BigUint::from_str(&gas_multiple).unwrap();
        assert_eq!(gas_multiple, multiple.to_string());
    }
    #[tokio::test]
    async fn test_get_balance() {
        dotenv::dotenv().ok();
        let big_int = get_balance(
            "https://rpc.fzcode.com",
            hex::decode("307440e3BF25Fa0870266e09A37E417a7d03597E")
                .unwrap()
                .as_slice(),
        )
        .await
        .unwrap();
        println!("BigUint: {}", big_int);

        println!("BigUint: {}", big_int * BigUint::from_str("2").unwrap());
    }
    // #[tokio::test]
    // async fn test_get_block_number() {
    //     dotenv::dotenv().ok();
    //     let res = get_block_number("https://rpc.fzcode.com").await.unwrap();
    //     println!("block number: {}", res);
    //     // 0x188e2
    // }

    #[test]
    fn test_odd_vec_hex() {
        let vec = hex::decode("0189c6").unwrap();
        let num = num_bigint::BigUint::from_bytes_be(&vec);
        println!("{}", num)
    }
    #[tokio::test]
    async fn test_get_gas_price() {
        dotenv::dotenv().ok();
        let res = get_gas_price("https://rpc.fzcode.com").await.unwrap();
        println!("gas price: {}", res);
        // 0x188e2
    }
    #[tokio::test]
    async fn test_get_max_priority_fee_per_gas() {
        dotenv::dotenv().ok();
        let support_1559 = get_latest_base_fee_per_gas().await;
        if let Some(_) = support_1559 {
            let res = get_max_priority_fee_per_gas("https://rpc.fzcode.com")
                .await
                .unwrap();
            println!("get_max_priority_fee_per_gas: {}", res);
            println!("hex:{}", res.to_str_radix(16));
        }

        // 0x188e2
    }
    #[tokio::test]
    async fn test_get_latest() {
        dotenv::dotenv().ok();
        let res = get_latest_block("https://rpc.fzcode.com").await.unwrap();
        println!("get_latest_block: {:?}", res);
        // 0x188e2
    }

    #[tokio::test]
    async fn test_get_nonce() {
        dotenv::dotenv().ok();
        let address = get_address_by_private_key(
            &hex::decode("f40bb21badf540a80c9cdadf38706408759786b6f991cfbc93556ac95baaf041").unwrap(),
        )
        .unwrap();
        let nonce = get_nonce("https://rpc.fzcode.com", address.as_slice())
            .await
            .unwrap();
        println!("test_get_nonce: {:?}", nonce);
        // 0x188e2
    }

    #[tokio::test]
    async fn test_estimate_gas() {
        dotenv::dotenv().ok();
        let rpc = env::var(RPC).unwrap();
        let from = get_address_by_private_key(
            &hex::decode("f40bb21badf540a80c9cdadf38706408759786b6f991cfbc93556ac95baaf041").unwrap(),
        )
        .unwrap();
        let to = hex::decode("A303721F08B85af1Fdf7C57152b9e31D4BCa397B").unwrap();
        let latest_base_fee_per_gas = get_latest_base_fee_per_gas().await;
        let request;
        let faucet_number = env::var(FAUCET_NUMBER).unwrap();
        if let Some(base_fee_per_gas) = latest_base_fee_per_gas {
            let max_priority_fee_per_gas = get_max_priority_fee_per_gas(&rpc).await.unwrap();
            request =
                utils::eth::EstimateGasTransactionRequest::Eip1559(EstimateGasEip1559transaction {
                    from,
                    to,
                    max_fee_per_gas: (max_priority_fee_per_gas.clone() + base_fee_per_gas),
                    max_priority_fee_per_gas,
                    value: hex_to_big_num(faucet_number).unwrap(),
                });
        } else {
            let gas_price = get_gas_price(&rpc).await.unwrap();
            request =
                utils::eth::EstimateGasTransactionRequest::Legacy(EstimateGasLegacyTransaction {
                    from,
                    to,
                    gas_price: gas_price,
                    value: hex_to_big_num(faucet_number).unwrap(),
                });
        }
        let res = estimate_gas(&rpc, request).await.unwrap();
        println!("test_estimate_gas: {:?}", res);
        // 0x188e2
    }
}

// 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80
