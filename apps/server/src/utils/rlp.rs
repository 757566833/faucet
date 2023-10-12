fn strip_zeros(value: &str) -> Vec<u8> {
    let mut hex = value.to_string();
    if hex.len() % 2 != 0 {
        hex = format!("0{}", hex)
    }

    let mut result = hex::decode(hex).unwrap_or(Vec::new());

    if result.is_empty() {
        return result;
    }

    // Find the first non-zero entry
    let mut start = 0;
    while start < result.len() && result[start] == 0 {
        start += 1;
    }

    // If we started with zeros, strip them
    if start > 0 {
        result.drain(0..start);
    }

    result
}
fn rlp_encode(data: &Vec<u8>) -> Vec<u8> {
    let mut result = Vec::new();

    if data.len() == 1 && data[0] < 128 {
        // For single bytes less than 128, RLP encoding is the same as the byte itself
        result.push(data[0]);
    } else if data.len() <= 55 {
        // For short strings (up to 55 bytes), RLP encoding starts with 0x80 + length
        result.push(0x80 + data.len() as u8);
        result.extend_from_slice(data);
    } else {
        // For longer strings, RLP encoding starts with 0xb7 + length_length and then length bytes
        let length = data.len();
        let length_bytes = length.to_be_bytes();
        let length_length = length_bytes.iter().position(|&x| x != 0).unwrap_or(0);
        result.push(0xb7 + length_length as u8);
        result.extend_from_slice(&length_bytes[(4 - length_length)..]);
        result.extend_from_slice(data);
    }

    result
}
pub fn big_num_to_vec(bg: num_bigint::BigUint) -> Vec<u8> {
    return strip_zeros(&bg.to_str_radix(16));
}

fn rlp_encode_list(data: &Vec<u8>) -> Vec<u8> {
    let mut result = Vec::new();

    if data.is_empty() {
        // Handle empty list
        result.push(0xc0);
    } else if data.len() == 1 && data[0] < 128 {
        // For single bytes less than 128, RLP encoding is the same as the byte itself
        result.push(data[0]);
    } else if data.len() <= 55 {
        // For short strings (up to 55 bytes), RLP encoding starts with 0x80 + length
        result.push(0x80 + data.len() as u8);
        result.extend_from_slice(data);
    } else {
        // For longer strings, RLP encoding starts with 0xb7 + length_length and then length bytes
        let length = data.len();
        let length_bytes = length.to_be_bytes();
        let length_length = length_bytes.iter().position(|&x| x != 0).unwrap_or(0);
        result.push(0xb7 + length_length as u8);
        result.extend_from_slice(&length_bytes[(4 - length_length)..]);
        result.extend_from_slice(data);
    }

    result
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use k256::pkcs8::der::Encode;

    use crate::utils::rlp::{big_num_to_vec, rlp_encode_list, strip_zeros};

    use super::rlp_encode;

    #[test]
    fn test_rlp() {
        let data = vec![1, 2, 3];
        let encode = rlp_encode(&data);
        println!("{}", hex::encode(encode))
    }
    #[test]
    fn test_format() {
        // let data = ["1", "2", "3"];
        let encode = num_bigint::BigUint::from_str("21000")
            .unwrap()
            .to_str_radix(16);

        println!("{:?}", strip_zeros(&encode))
    }
    #[test]
    fn test_array() {
        let nonce = big_num_to_vec(num_bigint::BigUint::from_str("9").unwrap());
        let max_priority_fee_per_gas =
            big_num_to_vec(num_bigint::BigUint::from_str("1000000000").unwrap());
        let max_fee_per_gas = big_num_to_vec(num_bigint::BigUint::from_str("1000000007").unwrap());
        let gas_limit = big_num_to_vec(num_bigint::BigUint::from_str("21000").unwrap());

        let to = hex::decode("A303721F08B85af1Fdf7C57152b9e31D4BCa397B").unwrap();
        let value = big_num_to_vec(num_bigint::BigUint::from_str("5000000000000000000").unwrap());
        // let to = big_num_to_vec(num_bigint::BigUint::from_str("0xA303721F08B85af1Fdf7C57152b9e31D4BCa397B").unwrap());

        let nested_array = vec![
            [].to_vec(),
            nonce,
            max_priority_fee_per_gas,
            max_fee_per_gas,
            gas_limit,
            to,
            value,
            [].to_vec(),
        ];
        let mut length = 0;
        let mut result = Vec::new();

        for item in nested_array {
            length += item.len();
            result.extend_from_slice(&rlp_encode(&item));
        }
        println!("{}", length);
        println!("{:?}", hex::encode(result));
        let access_list: Vec<u8> = vec![];
        let result2 = rlp_encode_list(&access_list);

        println!("{:?}", hex::encode(result2));
        // 默认b7+
    }
}
// fn main() {
//     let data = hex::decode("c883d1c182c701");
//     let (item, _) = RlpItem::decode(&data.unwrap()).unwrap();
//     let encoded = item.encode();
//     println!("Decoded: {:?}", item);
//     println!("Re-encoded: {:x?}", encoded);
// }
