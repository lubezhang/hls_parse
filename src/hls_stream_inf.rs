/// HLS 协议主文件中不同清晰度的流地址
///
use crate::helper::*;

#[derive(Debug, Default)]
pub struct HlsStreamInf {
    pub bandwidth: String,
    pub program_id: String,
    pub codecs: String,
    /** 视频流协议文件链接 */
    pub url: String,
}

impl HlsStreamInf {
    pub fn new() -> Self {
        HlsStreamInf {
            bandwidth: String::from(""),
            program_id: String::from(""),
            codecs: String::from(""),
            url: String::from(""),
        }
    }
    pub fn destructure(&mut self, str_protocol: &String, str_value: &String) {
        let params = destructure_params(str_protocol);
        self.bandwidth = params.get("bandwidth").unwrap().to_string();
        self.program_id = params.get("program-id").unwrap().to_string();
        self.url = str_value.to_string();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stream_inf_parse() {
        let mut stream_inf = HlsStreamInf::new();
        assert_eq!(stream_inf.bandwidth, "");
        stream_inf.destructure(
            &String::from("#EXT-X-STREAM-INF:PROGRAM-ID=1,BANDWIDTH=1064000"),
            &String::from("sdfsdf"),
        );
        assert_eq!(stream_inf.bandwidth, "1064000");
        assert_eq!(stream_inf.program_id, "1");
        assert_eq!(stream_inf.url, "sdfsdf");
    }
}
