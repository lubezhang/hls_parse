use crate::helper::*;
use crate::types::*;

#[derive(Debug, Default, Clone)]
pub struct HlsExtInf {
    pub index: u32,
    /** 每个切片的实际时长。单位：秒 */
    pub duration: f32,
    /** 片的描述 */
    pub title: String,
    /** 每片的链接 */
    pub url: Option<String>,
    /** 当前链接在密钥队列的索引。值为-1 视频没有加密不需要密钥 */
    pub encrypt_index: i32,
}

impl HlsExtInf {
    pub fn new() -> Self {
        HlsExtInf {
            index: 0,
            duration: 0.0,
            title: String::from(""),
            url: Some("".to_string()),
            encrypt_index: -1,
        }
    }
    pub fn destructure(&mut self, str_protocol: &String, str_value: Option<&String>) {
        destructure_params(str_protocol).map(|params| match params {
            ProtocolParam::Map(_map) => {}
            ProtocolParam::Array(_arr) => {
                for (i, param) in _arr.iter().enumerate() {
                    match i {
                        0 => self.duration = str_to_float(param),
                        1 => self.title = param.to_string(),
                        _ => {}
                    }
                }
                if str_value != None {
                    self.url = Some(str_value.unwrap().to_string());
                }
                // self.encrypt_index =
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_destructure() {
        let url = String::from(
            "https://ts4.chinalincoln.com:9999/20210419/OvroTYry/1000kb/hls/YMgVK9tU.ts",
        );
        let mut ext_inf = HlsExtInf::new();
        ext_inf.destructure(&String::from(" #EXTINF:4.128,title"), Option::Some(&url));

        assert_eq!(4.128, ext_inf.duration);
        assert_eq!("title", ext_inf.title);
        assert_eq!(url, ext_inf.url.unwrap());
    }
}
