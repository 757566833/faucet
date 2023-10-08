// const text =
//   "1";

// async function digestMessage(message) {
//   const msgUint8 = new TextEncoder().encode(message); // encode as (utf-8) Uint8Array
//   const hashBuffer = await crypto.subtle.digest("SHA-256", msgUint8); // hash the message
//   const hashArray = Array.from(new Uint8Array(hashBuffer)); // convert buffer to byte array
//   const hashHex = hashArray
//     .map((b) => b.toString(16).padStart(2, "0"))
//     .join(""); // convert bytes to hex string
//   return hashHex;
// }

// digestMessage(text).then((digestHex) => console.log(digestHex));
// 6b86b273ff34fce19d6b804eff5a3f5747ada4eaa22f1d49c01e52ddb7875b4b

use sha2::{Digest, Sha256};

pub fn sha256(str: String) -> String {
    return hex::encode(Sha256::digest(str.as_bytes()));
}

#[cfg(test)]
mod tests {
    use crate::utils::sha256::sha256;

    #[test]
    fn test_sha256() {
        let hex = sha256(String::from("1"));
        assert_eq!(
            hex,
            "6b86b273ff34fce19d6b804eff5a3f5747ada4eaa22f1d49c01e52ddb7875b4b"
        );
    }
}
