use std::collections::HashMap;
/**
 * HLS协议的文件类型
 */
#[derive(Debug, Clone, PartialEq)]
pub enum PlayListType {
    /** 主文件 */
    Master,
    /** 回放视频 */
    Vod,
    /** 直播 */
    Live,
}

/** HLS协议标签 */
#[derive(Debug, Clone, PartialEq)]
pub enum ProtocolTag {
    /// 协议内容
    Value,
    /** 文件首行的标记 */
    Extm3U,
    /** 视频流地址 */
    Extinf,
    /** 主文件 master playlist */
    ExtXStreamInf,
    /** 文件类型 */
    ExtXPlaylistType,
    /** 视频流结束标示 */
    ExtXEndlist,
    /** 文件加密 */
    ExtXKey,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ProtocolParam {
    // Tag(ProtocolTag),
    Map(HashMap<String, String>),
    Array(Vec<String>),
}
