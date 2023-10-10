use aes_gcm::aead::Aead;
use base64::{engine::general_purpose, Engine};

use crate::{
    error::{ResponseError, aes_cache_error},
    CIPHER_ONCE, NONCE_ONCE,
};

pub fn encrypt_data(timestamp: u64) -> Result<String, ResponseError> {
    let cipher_option = CIPHER_ONCE.get();
    let nonce_option = NONCE_ONCE.get();
    if let (Some(cipher), Some(nonce)) = (cipher_option, nonce_option) {
        let binding = timestamp.to_string();
        let ciphertext = cipher.encrypt(&nonce, binding.as_bytes().as_ref())?;
        return Ok(general_purpose::STANDARD.encode(ciphertext));
    } else {
        return Err(aes_cache_error());
    }
}

pub fn decrypt_data(b64: String) -> Result<u64, ResponseError> {
    let vec = general_purpose::STANDARD.decode(b64)?;
    let cipher_option = CIPHER_ONCE.get();
    let nonce_option = NONCE_ONCE.get();
    if let (Some(cipher), Some(nonce)) = (cipher_option, nonce_option) {
        let ciphertext = cipher.decrypt(&nonce, vec.as_ref())?;
        let string = String::from_utf8(ciphertext)?;
        let u64 = string.parse::<u64>()?;
        return Ok(u64);
    }
    return Err(aes_cache_error());
}

#[cfg(test)]
mod tests {
    // use std::time::{SystemTime, UNIX_EPOCH};

    use aes_gcm::{
        aead::{
            consts::{B0, B1},
            generic_array::GenericArray,
            Aead, AeadCore, KeyInit, OsRng,
        },
        aes::{
            cipher::typenum::{UInt, UTerm},
            Aes256,
        },
        Aes256Gcm,
        AesGcm, // Or `Aes128Gcm`
    };
    use base64::{engine::general_purpose, Engine};

    use crate::{CIPHER_ONCE, NONCE_ONCE};

    use super::encrypt_data;

    #[test]
    fn test_aes() {
        let key: GenericArray<
            u8,
            UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>,
        > = Aes256Gcm::generate_key(OsRng);

        let cipher: AesGcm<Aes256, UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>> =
            Aes256Gcm::new(&key);
        let _ = CIPHER_ONCE.set(cipher);

        let nonce: GenericArray<u8, UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>> =
            Aes256Gcm::generate_nonce(&mut OsRng);
        let _ = NONCE_ONCE.set(nonce);
        // let current_time = SystemTime::now();

        // // 获取UNIX纪元（1970-01-01T00:00:00Z）以来的持续时间
        // let duration_since_epoch = current_time.duration_since(UNIX_EPOCH).expect("时间错误");

        // // 获取持续时间的总秒数
        // let timestamp = duration_since_epoch.as_secs();
        // println!("{}", timestamp);
        let mut result: u64 = 0;
        let time: u64 = 1696685465;
        let res = encrypt_data(time);
        let mut b64 = String::from("");
        match res {
            Ok(r) => {
                b64 = r;
            }
            Err(_) => {}
        }
        let vec_result = general_purpose::STANDARD.decode(b64);
        let cipher_option = CIPHER_ONCE.get();
        if let (Ok(vec), Some(cipher)) = (vec_result, cipher_option) {
            let ciphertext_result = cipher.decrypt(&nonce, vec.as_ref());
            if let Ok(ciphertext) = ciphertext_result {
                let string_result = String::from_utf8(ciphertext);
                if let Ok(string) = string_result {
                    let u64_result = string.parse::<u64>();
                    if let Ok(u64) = u64_result {
                        result = u64
                    }
                }
            }
        }
        assert_eq!(time, result);
    }
}
