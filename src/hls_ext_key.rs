use super::types::*;
use crate::helper::*;

#[derive(Debug)]
pub struct HlsExtKey {
    pub index: u32,
    /** 文件加密方式 */
    pub method: String,
    /** 密钥链接 */
    pub uri: Option<String>,
    pub key: String,
    pub iv: String,
}

impl HlsExtKey {
    pub fn new() -> Self {
        HlsExtKey {
            index: 0,
            method: String::from(""),
            uri: Some("".to_string()),
            key: String::from(""),
            iv: String::from(""),
        }
    }
    pub fn destructure(&mut self, str_protocol: &String) {
        let keys: Vec<&str> = vec!["method", "uri", "key", "iv"];
        destructure_params(str_protocol).map(|params| match params {
            ProtocolParam::Map(map) => {
                self.method = map_val(&map, keys[0]);
                self.uri = Some(map_val(&map, keys[1]));
                self.iv = map_val(&map, keys[3]);
            }
            ProtocolParam::Array(_arr) => {}
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_destructure() {
        let mut ext_key = HlsExtKey::new();
        ext_key.destructure(&String::from(" #EXT-X-KEY:METHOD=AES-128,IV=123456,URI=\"https://ts4.chinalincoln.com:9999/20210419/OvroTYry/1000kb/hls/key.key\""));

        assert_eq!("AES-128", ext_key.method);
        assert_eq!("123456", ext_key.iv);
        assert_eq!(
            "https://ts4.chinalincoln.com:9999/20210419/OvroTYry/1000kb/hls/key.key",
            ext_key.uri.unwrap()
        );
    }
}
