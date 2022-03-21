use crate::helper::*;
use crate::hls_ext_inf::HlsExtInf;
use crate::hls_stream_inf::*;
use crate::types::*;

#[derive(Debug)]
pub struct HLS {
    /** 协议根元素，在协议文件的第一行 */
    pub ext_m3u: String,
    /** 协议类型 */
    pub ext_playlist_type: PlayListType,
    /** 主文件多个分辨率的视频流 */
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
            match iter.next() {
                Some(str_hls) => match extract_tag(&str_hls) {
                    ProtocolTag::Extm3U => {}
                    ProtocolTag::ExtXEndlist => {}
                    ProtocolTag::ExtXPlaylistType => self.parse_playlist_type(&str_hls),
                    ProtocolTag::ExtXStreamInf => {
                        self.parse_stream_inf(&str_hls, iter.next());
                    }
                    ProtocolTag::Extinf => self.parse_ext_inf(&str_hls, iter.next()),
                    _ => {}
                },
                None => break,
            }
        }
    }

    fn parse_playlist_type(&mut self, str_protocol: &String) {
        destructure_params(str_protocol).map(|params| match params {
            ProtocolParam::Map(_map) => {}
            ProtocolParam::Array(arr) => match arr[0].as_str() {
                "VOD" => self.ext_playlist_type = PlayListType::Vod,
                _ => self.ext_playlist_type = PlayListType::Master,
            },
        });
    }

    ///
    /// 解析主文件的流媒体信息和对应的视频链接
    ///
    fn parse_stream_inf(&mut self, str_protocol: &String, str_value: Option<&String>) {
        let mut stream_inf = HlsStreamInf::new();
        stream_inf.destructure(str_protocol, str_value);
        self.ext_stream_inf.push(stream_inf);
    }

    fn parse_ext_inf(&mut self, str_protocol: &String, str_value: Option<&String>) {
        let mut ext_inf = HlsExtInf::new();
        ext_inf.destructure(str_protocol, str_value);
        // self.ext_stream_inf.push(stream_inf);
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn test_hls_parse_master() {
    //     // 测试数据

    // }
}
