use crate::helper::*;
use crate::hls_ext_inf::HlsExtInf;
use crate::hls_ext_key::HlsExtKey;
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
    /** 视频流队列 */
    pub ext_inf: Vec<HlsExtInf>,
    /** 加密密钥队列 */
    pub ext_key: Vec<HlsExtKey>,

    base_url: String,
}

impl HLS {
    pub fn new() -> Self {
        HLS {
            ext_m3u: String::from("#EXTM3U"),
            ext_playlist_type: PlayListType::Master,
            ext_stream_inf: Vec::<HlsStreamInf>::new(),
            ext_inf: Vec::<HlsExtInf>::new(),
            ext_key: Vec::<HlsExtKey>::new(),
            base_url: String::from(""),
        }
    }

    pub fn set_base_url(&mut self, base_url: &String) {
        self.base_url = base_url.to_string();
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
                    ProtocolTag::ExtXKey => self.parse_ext_key(&str_hls),
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

        // TODO 处理相对链接地址
        stream_inf.url = join_url(&stream_inf.url, &self.base_url).unwrap();

        self.ext_stream_inf.push(stream_inf);
    }

    fn parse_ext_inf(&mut self, str_protocol: &String, str_value: Option<&String>) {
        let mut ext_inf = HlsExtInf::new();
        ext_inf.destructure(str_protocol, str_value);

        // 设置此条视频流的加密密钥在密钥队列中的索引
        ext_inf.encrypt_index = (self.ext_key.len() as i32) - 1;

        // TODO 处理相对链接地址
        ext_inf.url = join_url(&ext_inf.url, &self.base_url).unwrap();

        self.ext_inf.push(ext_inf);
    }

    fn parse_ext_key(&mut self, str_protocol: &String) {
        let mut ext_key = HlsExtKey::new();
        ext_key.destructure(str_protocol);
        self.ext_key.push(ext_key);
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
