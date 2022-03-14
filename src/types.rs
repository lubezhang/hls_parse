#[derive(Debug, Clone)]

pub enum PlayListType {
    Master,
    Vod,
    Live
}

pub struct HLSProtocol {
    /** 协议根元素，在协议文件的第一行 */
    pub ext_m3u: String,
    /** 协议类型 */
    pub ext_playlist_type: PlayListType
}


// impl HLSProtocol {
//     pub fn new() -> Self {
//         // self::ext_m3u = "";
//         return Self::default();
//     }
// }
