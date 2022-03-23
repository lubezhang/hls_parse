use crypto::aes::{cbc_decryptor, cbc_encryptor, KeySize};
use crypto::blockmodes::PkcsPadding;
use crypto::buffer::{ReadBuffer, RefReadBuffer, RefWriteBuffer, WriteBuffer};

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn encrypt_aes(data: &[u8], key: &str, iv: &str) -> Vec<u8> {
    let key1 = key.as_bytes();
    let iv1 = iv.as_bytes();
    let mut encryptor = cbc_encryptor(KeySize::KeySize128, &key1, &iv1, PkcsPadding);

    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = RefReadBuffer::new(data);

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

    final_result
}

fn decrypt_aes(data: &[u8], key: &str, iv: &str) -> Vec<u8> {
    let key1 = key.as_bytes();
    let iv1 = iv.as_bytes();
    let mut decryptor = cbc_decryptor(KeySize::KeySize128, &key1, &iv1, PkcsPadding);

    // let buffer1 = <[u8; 16]>::from_hex(msg);
    // let buffer1 = match hex::decode(data) {
    //     Ok(val) => val,
    //     _ => Vec::<u8>::new(),
    // };

    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = RefReadBuffer::new(data);
    let mut buffer = [0; 4096];
    let mut write_buffer = RefWriteBuffer::new(&mut buffer);

    // loop {
    let _result = decryptor.decrypt(&mut read_buffer, &mut write_buffer, true);
    final_result.extend(
        write_buffer
            .take_read_buffer()
            .take_remaining()
            .iter()
            // .filter(|&i| *i != 0)
            .map(|&i| i),
    );
    // String::from_utf8(final_result.clone()).unwrap()
    final_result
}

pub fn encrypt_aes_file(file_path: &str, key: &str, iv: &str) {
    let _key1 = key.as_bytes();
    let _iv1 = iv.as_bytes();

    let path = Path::new(file_path);
    let display = path.display();

    let mut file = match File::open(&path) {
        // 处理打开文件可能潜在的错误
        Err(_why) => panic!("couldn't open {}", display),
        Ok(file) => file,
    };

    let output_path = Path::new("/Users/zhangqinghong/study/rust/hls_parse/tests/test1.png");
    let display1 = output_path.display();
    let mut output_file = match File::create(&output_path) {
        // 处理打开文件可能潜在的错误
        Err(_why) => panic!("couldn't open {}", display1),
        Ok(file) => file,
    };

    let mut count = 0;
    loop {
        let mut buf = vec![0; 16];
        let n = file.read(&mut buf[..]).unwrap();
        // buf = buf.iter().filter(|&i| *i != 0).map(|&i| i).collect();

        if n == 0 {
            break;
        }
        // if count == 4981 {
        //     break;
        // }

        let res = encrypt_aes(&buf, key, iv);
        println!("=={}=={}=={:?}", count, n, buf);
        // println!(
        //     "=={}==: {:?} : {}",
        //     buf.len(),
        //     String::from_utf8(buf.clone()),
        //     hex::encode(&res)
        // );

        output_file.write_all(&res).expect("write failed");

        // if
        count += 1;
    }
}

pub fn decrypt_aes_file(file_path: &str, key: &str, iv: &str) {
    let _key1 = key.as_bytes();
    let _iv1 = iv.as_bytes();

    let path = Path::new(file_path);
    let display = path.display();

    let mut file = match File::open(&path) {
        // 处理打开文件可能潜在的错误
        Err(_why) => panic!("couldn't open {}", display),
        Ok(file) => file,
    };

    let output_path = Path::new("/Users/zhangqinghong/study/rust/hls_parse/tests/test2.png");
    let display1 = output_path.display();
    let mut output_file = match File::create(&output_path) {
        // 处理打开文件可能潜在的错误
        Err(_why) => panic!("couldn't open {}", display1),
        Ok(file) => file,
    };

    let mut count = 0;
    loop {
        let mut buf = vec![0; 32];
        let n = file.read(&mut buf[..]).unwrap();
        // buf = buf.iter().filter(|&i| *i != 0).map(|&i| i).collect();

        if n == 0 {
            break;
        }

        // if count == 4981 {
        //     break;
        // }
        let res = decrypt_aes(&buf, key, iv);
        println!("=={}=={}=={:?}", count, n, res);
        output_file.write_all(&res).expect("write failed");
        // println!(
        //     "=={}==: {:?} : {}",
        //     buf.len(),
        //     String::from_utf8(buf.clone()),
        //     hex::encode(&res)
        // );
        count += 1;
    }
}

pub fn encrypt_aes_str(msg: &str, key: &str, iv: &str) -> String {
    let res = encrypt_aes(msg.as_bytes(), key, iv);
    hex::encode(&res)
}

pub fn decrypt_aes_str(msg: &str, key: &str, iv: &str) -> String {
    // let key1 = key.as_bytes();
    // let iv1 = iv.as_bytes();
    // let mut decryptor = cbc_decryptor(KeySize::KeySize128, &key1, &iv1, PkcsPadding);

    // let buffer1 = <[u8; 16]>::from_hex(msg);
    let buffer1 = match hex::decode(msg) {
        Ok(val) => val,
        _ => Vec::<u8>::new(),
    };
    let res = decrypt_aes(&buffer1, key, iv);
    String::from_utf8(res.clone()).unwrap()

    // let mut final_result = Vec::<u8>::new();
    // let mut read_buffer = RefReadBuffer::new(&buffer1);
    // let mut buffer = [0; 4096];
    // let mut write_buffer = RefWriteBuffer::new(&mut buffer);

    // // loop {
    // let _result = decryptor.decrypt(&mut read_buffer, &mut write_buffer, true);
    // final_result.extend(
    //     write_buffer
    //         .take_read_buffer()
    //         .take_remaining()
    //         .iter()
    //         .filter(|&i| *i != 0)
    //         .map(|&i| i),
    // );
    // String::from_utf8(final_result.clone()).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const CRYPTO_KEY: &str = "1000000000000000";
    const CRYPTO_IV: &str = "0000000000000000";

    #[test]
    fn test_encrypt_aes() {
        // assert_eq!(
        //     "ab6c513b8f4e04d86a932b4552a09a46",
        //     encrypt_aes_str(&"你好", &CRYPTO_KEY, &CRYPTO_IV)
        // );

        assert_eq!(
            "e6996765a06345cf5860efa14ea8d36e",
            encrypt_aes_str(&"#EXTM3U#", &CRYPTO_KEY, &CRYPTO_IV)
        );
    }

    #[test]
    fn test_decrypt_aes() {
        assert_eq!(
            "你好",
            decrypt_aes_str(&"ab6c513b8f4e04d86a932b4552a09a46", &CRYPTO_KEY, &CRYPTO_IV)
        );
    }

    #[test]
    fn test_encrypt_aes_file() {
        let file_path = "/Users/zhangqinghong/Movies/1645153272333099.mp4";
        encrypt_aes_file(&file_path, &CRYPTO_KEY, &CRYPTO_IV);
    }

    #[test]
    fn test_decrypt_aes_file() {
        let file_path = "/Users/zhangqinghong/study/rust/hls_parse/tests/test1.png";
        decrypt_aes_file(&file_path, &CRYPTO_KEY, &CRYPTO_IV);
    }
}
