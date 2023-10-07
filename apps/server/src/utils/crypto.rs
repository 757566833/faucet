use base64::{engine::general_purpose, Engine};
use crypto::{
    aes::{self, KeySize},
    blockmodes::NoPadding,
    buffer::{ReadBuffer, WriteBuffer},
};

const KEY: &[u8; 16] = b"supersecretkey16";
pub fn encrypt_data(timestamp: u64) -> Result<String, String> {
    let mut encryptor = aes::ecb_encryptor(aes::KeySize::KeySize128, KEY, NoPadding);

    // // 要加密的数据
    // let plaintext = b"Hello, AES!";

    let mut buffer = [0; 16];
    let binding = timestamp.to_string();
    let mut read_buffer = crypto::buffer::RefReadBuffer::new(binding.as_bytes());
    let mut write_buffer = crypto::buffer::RefWriteBuffer::new(&mut buffer);

    // 执行加密操作
    let res = encryptor.encrypt(&mut read_buffer, &mut write_buffer, true);
    if let Err(r) = res {
        println!("{:?}", r);
        return Err(String::from("can't encrypt"));
    }
    let mut binding = write_buffer.take_read_buffer();
    let encrypted_data = binding.take_remaining();
    let b64 = general_purpose::STANDARD.encode(encrypted_data);
    return Ok(b64);

    // const CUSTOM_ENGINE: engine::GeneralPurpose =
    //     engine::GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::NO_PAD);

    // let b64_url = CUSTOM_ENGINE.encode(b"hello internet~");
    //     return base64::encode(encrypted_data);
}

pub fn decrypt_data(b64: String) -> Result<u64, String> {
    let bytes_result = general_purpose::STANDARD.decode(b64);
    let bytes;
    match bytes_result {
        Ok(b) => bytes = b,
        Err(e) => return Err(e.to_string()),
    }
    let mut decryptor = aes::ecb_decryptor(KeySize::KeySize128, KEY, NoPadding);

    // 创建一个缓冲区来存储解密后的数据
    let mut decrypted_buffer = [0; 16];
    let mut read_buffer = crypto::buffer::RefReadBuffer::new(&bytes);
    let mut write_buffer = crypto::buffer::RefWriteBuffer::new(&mut decrypted_buffer);

    // 执行解密操作
    let parse_result = decryptor.decrypt(&mut read_buffer, &mut write_buffer, true);
    if let Err(_) = parse_result {
        return Err(String::from("can't decrypt"));
    }

    // 获取解密后的数据
    let mut binding = write_buffer.take_read_buffer();
    let decrypted_data = binding.take_remaining();

    // 将解密后的数据转换为字符串并打印
    let decrypted_result = String::from_utf8(decrypted_data.to_vec());
    // println!("解密后的数据: {}", decrypted_str);
    match decrypted_result {
        Ok(time) => {
            if let Ok(parsed_u64) = time.parse::<u64>() {
                println!("解析后的 u64 值: {}", parsed_u64);
                return Ok(parsed_u64);
            } else {
                return Err(String::from("can't parse"));
            }
        }
        Err(e) => return Err(e.to_string()),
    }
}


#[cfg(test)]
mod tests {
    // use std::time::{SystemTime, UNIX_EPOCH};

    use super::encrypt_data;


  #[test]
  fn d() {
    // let current_time = SystemTime::now();
    
    // // 获取UNIX纪元（1970-01-01T00:00:00Z）以来的持续时间
    // let duration_since_epoch = current_time.duration_since(UNIX_EPOCH).expect("时间错误");
    
    // // 获取持续时间的总秒数
    // let timestamp = duration_since_epoch.as_secs();
    // println!("{}", timestamp);
    let time:u64 = 1696685465;
    let res = encrypt_data(time);
    match res {
        Ok(r) => {
            println!("{}", r)
        },
        Err(e) => {
            println!("{}", e)
        },
    }
  }

 
}
