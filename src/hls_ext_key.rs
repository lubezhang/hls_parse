use super::types::*;
use crate::helper::*;

#[derive(Debug)]
pub struct HlsExtKey {
    pub index: u32,
    /** 文件加密方式 */
    pub method: String,
    /** 密钥链接 */
    pub uri: String,
    pub key: String,
    pub iv: String,
}

impl HlsExtKey {
    pub fn new() -> Self {
        HlsExtKey {
            index: 0,
            method: String::from(""),
            uri: String::from(""),
            key: String::from(""),
            iv: String::from(""),
        }
    }
    pub fn destructure(&mut self, str_protocol: &String) {
        let keys: Vec<&str> = vec!["method", "uri", "key", "iv"];
        destructure_params(str_protocol).map(|params| match params {
            ProtocolParam::Map(map) => {
                self.method = map_val(&map, keys[0]);
                self.uri = map_val(&map, keys[1]);
                self.iv = map_val(&map, keys[3]);
            }
            ProtocolParam::Array(_arr) => {}
        });
    }
}
