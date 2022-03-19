use super::types::*;
use crate::helper::*;
/// HLS 协议主文件中不同清晰度的流地址
#[derive(Debug, Default)]
pub struct HlsStreamInf {
    /// 带宽。表示对于每个媒体文件所有比特率的上限，单位是 比特/秒
    pub bandwidth: u32,
    /// 唯一地标识一个在 Playlist 文件范围内的特定的描述。一个 Playlist 文件中可能包含多个有相同 ID 的此 tag
    pub program_id: u32,
    /// 编码类型
    pub codecs: String,
    /// 分辨率
    pub resolution: String,
    /// 视频流协议文件链接
    pub url: String,
}

impl HlsStreamInf {
    pub fn new() -> Self {
        HlsStreamInf {
            bandwidth: 0,
            program_id: 0,
            codecs: String::from(""),
            resolution: String::from(""),
            url: String::from(""),
        }
    }
    pub fn destructure(&mut self, str_protocol: &String, str_value: Option<&String>) {
        let keys: Vec<&str> = vec!["bandwidth", "program-id", "codecs"];
        // 使用模式匹配的方式处理Option
        // match destructure_params(str_protocol) {
        //     Some(params) => match params {
        //         ProtocolParam::Map(map) => {
        //             self.bandwidth = map_val(&map, keys[0]).parse::<u32>().unwrap();
        //             self.program_id = map_val(&map, keys[1]).parse::<u32>().unwrap();
        //             self.codecs = map_val(&map, keys[2]);
        //             self.url = str_value.to_string();
        //         }
        //         ProtocolParam::Array(_arr) => {}
        //     },
        //     None => {}
        // }

        // 使用map的方式处理Option
        destructure_params(str_protocol).map(|params| match params {
            ProtocolParam::Map(map) => {
                self.bandwidth = str_to_int(&map_val(&map, keys[0]));
                self.program_id = str_to_int(&map_val(&map, keys[1]));
                self.codecs = map_val(&map, keys[2]);

                match str_value {
                    Some(val) => self.url = val.to_string(),
                    None => {}
                }
            }
            ProtocolParam::Array(_arr) => {}
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stream_inf_parse_map() {
        let mut stream_inf = HlsStreamInf::new();
        assert_eq!(stream_inf.bandwidth, 0);
        stream_inf.destructure(
            &String::from("#EXT-X-STREAM-INF:PROGRAM-ID=1,BANDWIDTH=1064000"),
            Option::Some(&String::from("1000kbps.m3u8")),
        );
        assert_eq!(stream_inf.bandwidth, 1064000);
        assert_eq!(stream_inf.program_id, 1);
        assert_eq!(stream_inf.url, "1000kbps.m3u8");

        stream_inf = HlsStreamInf::new();
        stream_inf.destructure(
            &String::from("#EXT-X-STREAM-INF:PROGRAM-ID=1,BANDWIDTH=1064000"),
            Option::None,
        );
        assert_eq!(stream_inf.url, "");
    }
}
