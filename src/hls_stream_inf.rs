#[derive(Debug)]
pub struct HlsStreamInf {
    pub bandwidth: String,
    pub program_id: String,
    pub codecs: String,
    /** 视频流协议文件链接 */
    pub url: String,
}

impl HlsStreamInf {
    pub fn destructure() {}
}
