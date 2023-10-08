use std::env;

use ethers::{providers::Provider, signers::Wallet};
use ethereum_types::{H160};
use libsecp256k1::{PublicKey, SecretKey};
use tiny_keccak::{Keccak, Hasher};

use crate::constant::{FAUCET_NUMBER, PRIVATE_KEY, RPC};

pub async fn transaction()->Result<String, String> {
    let rpc_result = env::var(RPC);
    let private_key_result = env::var(PRIVATE_KEY);
    let faucet_number_result = env::var(FAUCET_NUMBER);
    if let (Ok(rpc), Ok(private_key), Ok(faucet_number)) =
        (rpc_result, private_key_result, faucet_number_result)
    {
        let provider_result = Provider::try_from(rpc);
        let provider;
        match provider_result {
            Ok(p) => provider = p,
            Err(e) => return Err(e.to_string()),
        }
        // Wallet::new(rng)
        return Ok(String::from("value"))
    } else {
        return Err(String::from("env error"));
    }
}

pub async fn get_address_by_private_key(private_key:String)->Result<String, String>{
    // let private_key_byte:[u8; 32];
    // if private_key.starts_with("0x"){
    //     // let private_key_byte_result:Result<&[u8; 32], _> = private_key[2..66].as_bytes().try_into();
    //     // match private_key_byte_result {
    //     //     Ok(p) => private_key_byte=p,
    //     //     Err(_) => return Err(String::from("private is error")),
    //     private_key_byte = private_key.as_bytes().try_into().expect("e");
    //     // }
    // }else{
    //     private_key_byte = hex::decode(private_key).expect("e");
    //     // let private_key_byte_result:Result<&[u8; 32], _> = private_key[0..64].as_bytes().try_into();
    //     // match private_key_byte_result {
    //     //     Ok(p) => private_key_byte=p,
    //     //     Err(_) => return Err(String::from("private is error")),
    //     // }
    // }
    let p;
    if private_key.starts_with("0x"){
        p = String::from(&private_key[2..]);
    }else{
        p = private_key
    }
    let private_key_bytes = hex::decode(p).expect("Invalid hex string");
    let mut private_key_array: [u8; 32] = [0; 32];
    private_key_array.copy_from_slice(&private_key_bytes);
    let secret_key = SecretKey::parse(&private_key_array).unwrap();
    
    // secp256k1
    let public_key = PublicKey::from_secret_key(&secret_key);

    let public_key_bytes = public_key.serialize();
    let mut public_keccak = Keccak::v256();
    let mut address_byte = [0u8; 32];
    // 去掉04开头
    public_keccak.update(&public_key_bytes[1..]);
    public_keccak.finalize(&mut address_byte);
    // 去掉前24个
    let address = H160::from_slice(&address_byte[12..]);
    let address_bytes = address.to_fixed_bytes();
    return Ok(hex::encode(address_bytes))
}

#[cfg(test)]
mod tests {
    use crate::utils::eth::get_address_by_private_key;

    #[tokio::test]
    async fn test_address() {
        let hex =get_address_by_private_key(String::from("0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80")).await;
       println!("{:?}",hex)
    }
}


// 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80