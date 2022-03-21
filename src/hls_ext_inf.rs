use super::types::*;
use crate::helper::*;

#[derive(Debug, Default)]
pub struct HlsExtInf {
    pub index: u32,
    /** 每个切片的实际时长。单位：秒 */
    pub duration: f32,
    /** 片的描述 */
    pub title: String,
    /** 每片的链接 */
    pub url: String,
    /** 当前文件在加密信息数组中的索引 */
    pub encrypt_index: u32,
}

impl HlsExtInf {
    pub fn new() -> Self {
        HlsExtInf {
            index: 0,
            duration: 0.0,
            title: String::from(""),
            url: String::from(""),
            encrypt_index: 0,
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
                    self.url = str_value.unwrap().to_string();
                }
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
        assert_eq!(url, ext_inf.url);
    }
}
