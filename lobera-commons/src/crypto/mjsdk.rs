use aes::cipher::{block_padding::Pkcs7, BlockDecryptMut, BlockEncryptMut, KeyIvInit};
use cbc::{Decryptor, Encryptor};
use flate2::read::GzDecoder;
use md5::{Digest, Md5};
use serde_json::Value;
use std::collections::BTreeMap;
use std::io::Read;

type Aes128CbcEnc = Encryptor<aes::Aes128>;
type Aes128CbcDec = Decryptor<aes::Aes128>;

pub struct MJSdk;

impl MJSdk {
    pub const SDK_KEY: &'static str = "A31B867B945ED736";

    pub fn sign_data(data: &BTreeMap<String, Value>) -> String {
        let mut sign_raw_parts = Vec::new();

        for (key, value) in data {
            if key == "sign" || key == "originalJSON" || key == "timeSecs" {
                continue;
            }

            let value_str = serde_json::to_string(value).unwrap_or_default();
            sign_raw_parts.push(format!("{}={}", key, value_str));
        }

        let time_secs = data
            .get("timeSecs")
            .and_then(|v| v.as_u64())
            .map(|v| v.to_string())
            .unwrap_or_else(|| {
                data.get("timeSecs")
                    .and_then(|v| v.as_str())
                    .map(|v| v.to_string())
                    .unwrap_or_default()
            });

        let mut sign_str = sign_raw_parts.join("&");
        sign_str.push_str(&time_secs);
        sign_str.push_str(Self::SDK_KEY);

        let sign_str = sign_str
            .replace('\n', "\\n")
            .replace('\r', "\\r")
            .replace("True", "true")
            .replace("False", "false");

        let mut hasher = Md5::new();
        hasher.update(sign_str.as_bytes());
        let result = hasher.finalize();
        let hex_result = hex::encode(result);

        hex_result[8..24].to_string()
    }

    pub fn parse_request_data(data: &[u8]) -> Vec<u8> {
        let xored: Vec<u8> = data.iter().map(|&d| d ^ 0x80).collect();
        let mut decoder = GzDecoder::new(&xored[..]);
        let mut decompressed = Vec::new();
        let _ = decoder.read_to_end(&mut decompressed);
        decompressed
    }

    pub fn parse_response_data(data: &[u8]) -> Vec<u8> {
        let key = Self::SDK_KEY.as_bytes();
        let iv = Self::SDK_KEY.as_bytes();
        let dec = Aes128CbcDec::new(key.into(), iv.into());
        let mut buf = data.to_vec();
        match dec.decrypt_padded_mut::<Pkcs7>(&mut buf) {
            Ok(pt) => pt.to_vec(),
            Err(_) => Vec::new(),
        }
    }

    pub fn build_response_data(data: &[u8]) -> Vec<u8> {
        let key = Self::SDK_KEY.as_bytes();
        let iv = Self::SDK_KEY.as_bytes();
        let enc = Aes128CbcEnc::new(key.into(), iv.into());
        let mut buf = vec![0u8; data.len() + 16];
        buf[..data.len()].copy_from_slice(data);
        match enc.encrypt_padded_mut::<Pkcs7>(&mut buf, data.len()) {
            Ok(ct) => ct.to_vec(),
            Err(_) => Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_sign_data() {
        let mut data = BTreeMap::new();
        data.insert("a".to_string(), json!(1));
        data.insert("b".to_string(), json!("hello"));
        data.insert("timeSecs".to_string(), json!(1625068800));
        let sign = MJSdk::sign_data(&data);
        println!("{}", sign);
    }
}