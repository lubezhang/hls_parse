use super::types::*;
use regex::Regex;

/// 提取协议中的协议标签
pub fn extract_tag(tag_line: &String) -> ProtocolTag {
    let reg = Regex::new("(^#E([^:])+)").unwrap();
    let mut tag_name = String::new();

    for elem in reg.captures_iter(tag_line) {
        tag_name = String::from(&elem[0])
    }

    match tag_name.as_str() {
        "#EXTM3U" => return ProtocolTag::Extm3U,
        "#EXTINF" => return ProtocolTag::Extinf,
        "#EXT-X-STREAM-INF" => return ProtocolTag::ExtXStreamInf,
        "#EXT-X-PLAYLIST-TYPE" => return ProtocolTag::ExtXPlaylistType,
        "#EXT-X-ENDLIST" => return ProtocolTag::ExtXEndlist,
        "#EXT-X-KEY" => return ProtocolTag::ExtXKey,
        _ => return ProtocolTag::Value,
    }
}

///
/// 清洗数据，将字符串形式的内容转成字符串数组
///
/// - `str_hls1` 字符串形式的协议内容
///
pub fn clean_content(str_hls: &String) -> Vec<String> {
    let mut res: Vec<String> = Vec::<String>::new();
    for token in str_hls.split("\n") {
        if token.trim().len() != 0 {
            res.push(token.trim().to_string());
        }
    }
    res
}

pub fn destructure_params(str_protocol: &String) {
    let vec_proto1: Vec<&str> = str_protocol.split(":").collect();
    if vec_proto1.len() == 2 {}
    // for token in str_protocol.split(":") {
    println!("======== {:?}", vec_proto1.len());
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_data;
    #[test]
    fn test_clean_content() {
        let str_master = test_data::get_data_master();
        let list = super::clean_content(&str_master);
        assert_eq!("#EXTM3U", list[0]);
        assert_eq!("#EXT-X-STREAM-INF:PROGRAM-ID=1,BANDWIDTH=1064000", list[1]);
        assert_eq!(9, list.len());
    }

    #[test]
    fn test_extract_tag() {
        let tag_type = super::extract_tag(&String::from("#EXTM3U"));
        assert_eq!(ProtocolTag::Extm3U, tag_type);

        let tag_type = super::extract_tag(&String::from(
            "#EXT-X-STREAM-INF:PROGRAM-ID=1,BANDWIDTH=1064000",
        ));
        assert_eq!(ProtocolTag::ExtXStreamInf, tag_type);
    }
}
