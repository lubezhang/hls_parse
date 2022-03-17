use crate::helper::*;
use crate::hls_stream_inf::*;
use crate::types::*;

#[derive(Debug)]
pub struct HLS {
    /** 协议根元素，在协议文件的第一行 */
    pub ext_m3u: String,
    /** 协议类型 */
    pub ext_playlist_type: PlayListType,
    pub ext_stream_inf: Vec<HlsStreamInf>,
}

impl HLS {
    pub fn new() -> HLS {
        HLS {
            ext_m3u: String::from("#EXTM3U"),
            ext_playlist_type: PlayListType::Master,
            ext_stream_inf: Vec::<HlsStreamInf>::new(),
        }
    }

    /// 结构化协议
    pub fn parse(&mut self, str_hls: &String) {
        let vec_hls = clean_content(str_hls);
        for elem in vec_hls {
            let tag_type = extract_tag(&elem);
            match tag_type {
                ProtocolTag::Extm3U => {}
                ProtocolTag::ExtXEndlist => {}
                ProtocolTag::ExtXStreamInf => {
                    self.parse_stream_inf(&elem);
                }
                _ => {}
            }
        }
    }

    fn parse_stream_inf(&mut self, str_protocol: &String) {
        destructure_params(str_protocol);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_data;

    #[test]
    fn test_hls_parse() {
        // 测试数据
        let str_master = test_data::get_data_master();

        let mut protocol1 = HLS::new();
        assert_eq!("#EXTM3U", protocol1.ext_m3u);

        protocol1.parse(&str_master);
        println!("struct json: {:#?}", protocol1)
        // assert_eq!(256, protocol1.ext_m3u.len());
    }
}
