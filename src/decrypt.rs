use crypto::aes::{cbc_decryptor, cbc_encryptor, KeySize};
use crypto::blockmodes::PkcsPadding;
use crypto::buffer::{ReadBuffer, RefReadBuffer, RefWriteBuffer, WriteBuffer};

pub fn encrypt_aes_str(msg: &str, key: &str, iv: &str) -> String {
    let key1 = key.as_bytes();
    let iv1 = iv.as_bytes();
    let mut encryptor = cbc_encryptor(KeySize::KeySize128, &key1, &iv1, PkcsPadding);

    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = RefReadBuffer::new(msg.as_bytes());

    let mut buffer = [0; 4096];
    let mut write_buffer = RefWriteBuffer::new(&mut buffer);

    let _result = encryptor.encrypt(&mut read_buffer, &mut write_buffer, true);

    final_result.extend(
        write_buffer
            .take_read_buffer()
            .take_remaining()
            .iter()
            .map(|&i| i),
    );
    hex::encode(&final_result)
}

pub fn decrypt_aes_str(msg: &str, key: &str, iv: &str) -> String {
    let key1 = key.as_bytes();
    let iv1 = iv.as_bytes();
    let mut decryptor = cbc_decryptor(KeySize::KeySize128, &key1, &iv1, PkcsPadding);

    // let buffer1 = <[u8; 16]>::from_hex(msg);
    let buffer1 = match hex::decode(msg) {
        Ok(val) => val,
        _ => Vec::<u8>::new(),
    };

    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = RefReadBuffer::new(&buffer1);
    let mut buffer = [0; 4096];
    let mut write_buffer = RefWriteBuffer::new(&mut buffer);

    // loop {
    let _result = decryptor.decrypt(&mut read_buffer, &mut write_buffer, true);
    final_result.extend(
        write_buffer
            .take_read_buffer()
            .take_remaining()
            .iter()
            .filter(|&i| *i != 0)
            .map(|&i| i),
    );
    String::from_utf8(final_result.clone()).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const CRYPTO_KEY: &str = "1000000000000000";
    const CRYPTO_IV: &str = "0000000000000000";

    #[test]
    fn test_encrypt_aes() {
        assert_eq!(
            "ab6c513b8f4e04d86a932b4552a09a46",
            encrypt_aes_str(&"你好", &CRYPTO_KEY, &CRYPTO_IV)
        );
    }

    #[test]
    fn test_decrypt_aes() {
        assert_eq!(
            "你好",
            decrypt_aes_str(&"ab6c513b8f4e04d86a932b4552a09a46", &CRYPTO_KEY, &CRYPTO_IV)
        );
    }
}
