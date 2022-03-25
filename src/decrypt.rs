use aesstream::{AesReader, AesWriter};
use crypto::aessafe::{AesSafe128Decryptor, AesSafe128Encryptor};

use std::fs::File;
use std::io::prelude::*;
use std::io::{Cursor, Read};
// use std::path::Path;
// use std::fs::remove_file;
use std::string::ParseError;

#[allow(dead_code)]
pub fn encrypt_aes(data: &[u8], key: &str) -> Result<Vec<u8>, ParseError> {
    let encryptor = AesSafe128Encryptor::new(key.as_bytes());
    let mut encrypted = Vec::new();
    {
        let mut writer = AesWriter::new(&mut encrypted, encryptor).unwrap();
        writer.write_all(data).unwrap();
    }
    Ok(encrypted)
}
#[allow(dead_code)]
pub fn decrypt_aes(data: &[u8], key: &str) -> Result<Vec<u8>, ParseError> {
    let decryptor = AesSafe128Decryptor::new(key.as_bytes());
    let mut reader = AesReader::new(Cursor::new(data), decryptor).unwrap();
    let mut v = Vec::new();
    reader.read_to_end(&mut v).unwrap();
    Ok(v)
}

// pub fn encrypt_aes_file(src_file_path: &str, dist_file_path: &str, key: &str) {}
#[allow(dead_code)]
pub fn decrypt_aes_file(src_file_path: &str, dist_file_path: &str, key: &str) {
    let file = match File::open(&src_file_path) {
        // 处理打开文件可能潜在的错误
        Err(_why) => panic!("couldn't open"),
        Ok(file) => file,
    };

    let decryptor = AesSafe128Decryptor::new(key.as_bytes());
    let mut reader = AesReader::new(file, decryptor).unwrap();
    let mut v = Vec::new();
    reader.read_to_end(&mut v).unwrap();

    let mut output_file = match File::create(dist_file_path) {
        // 处理打开文件可能潜在的错误
        Err(_why) => panic!("couldn't open"),
        Ok(file) => file,
    };
    output_file.write_all(&v).expect("write failed");
}
#[allow(dead_code)]
pub fn encrypt_aes_str(msg: &str, key: &str) -> Result<String, ParseError> {
    let res = encrypt_aes(msg.as_bytes(), key).unwrap();
    Ok(hex::encode(&res))
    // Ok("".to_string())
}
#[allow(dead_code)]
pub fn decrypt_aes_str(msg: &str, key: &str) -> Result<String, ParseError> {
    let buffer1 = match hex::decode(msg) {
        Ok(val) => val,
        _ => Vec::<u8>::new(),
    };

    let res = decrypt_aes(&buffer1, key).unwrap();
    Ok(String::from_utf8(res.clone()).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    const CRYPTO_KEY: &str = "1000000000000000";

    #[test]
    fn test_encrypt_aes() {
        let en = encrypt_aes_str(&"1", &CRYPTO_KEY).unwrap();
        assert_eq!("1", decrypt_aes_str(&en, &CRYPTO_KEY).unwrap());
    }

    #[test]
    fn test_decrypt_aes() {
        assert_eq!(
            "1",
            decrypt_aes_str(
                &"0b95b23b8fe7f3a50722bfe5a841f3bef2ea97630e0fa6b8421437a33e8baa42",
                &CRYPTO_KEY
            )
            .unwrap()
        );
    }

    // #[test]
    // fn test_encrypt_aes_file() {
    //     let file_path = "/Users/zhangqinghong/Movies/1645153272333099.mp4";
    //     encrypt_aes_file(&file_path, &CRYPTO_KEY, &CRYPTO_IV);
    // }

    #[test]
    fn test_decrypt_aes_file() {
        let key = "44a7e2d6a8fdbaae";

        let root_dir = std::env::current_dir().unwrap();

        let mut src_path = root_dir.clone();
        src_path.push(PathBuf::from("tests/data/1.ts"));

        let mut dist_path = root_dir.clone();
        dist_path.push(PathBuf::from("tests/data/1_1.ts"));

        decrypt_aes_file(
            src_path.to_str().unwrap(),
            dist_path.to_str().unwrap(),
            &key,
        );

        // remove_file(dist_path).unwrap();
    }
}
