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
    pub fn new() -> Self {
        HLS {
            ext_m3u: String::from("#EXTM3U"),
            ext_playlist_type: PlayListType::Master,
            ext_stream_inf: Vec::<HlsStreamInf>::new(),
        }
    }

    /// 结构化协议
    pub fn parse(&mut self, str_hls: &String) {
        let vec_hls = clean_content(str_hls);
        let mut iter = vec_hls.iter();

        loop {
            let tag_str = iter.next();
            // 如果没有数据，结束循环
            if tag_str == None {
                break;
            }

            let tmp = tag_str.unwrap();
            let tag_type = extract_tag(&tmp);
            match tag_type {
                ProtocolTag::Extm3U => {}
                ProtocolTag::ExtXEndlist => {}
                ProtocolTag::ExtXStreamInf => {
                    self.parse_stream_inf(&tmp, &(iter.next().unwrap()));
                }
                _ => {}
            }
        }
        println!("hls: {:#?}", self)
    }

    ///
    /// 解析主文件的流媒体信息和对应的视频链接
    ///
    fn parse_stream_inf(&mut self, str_protocol: &String, str_value: &String) {
        let mut stream_inf = HlsStreamInf::new();
        stream_inf.destructure(str_protocol, str_value);
        self.ext_stream_inf.push(stream_inf);
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
        // println!("struct json: {:#?}", protocol1)
    }
}
