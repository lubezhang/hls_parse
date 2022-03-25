use crate::types::*;
use regex::Regex;
use std::collections::HashMap;
use url::{ParseError, Url};

pub fn map_val(map: &HashMap<String, String>, key: &str) -> String {
    let val = map.get(key);
    match val {
        None => String::from(""),
        Some(value) => value.trim().to_string(),
    }
}

pub fn str_to_int(str1: &String) -> u32 {
    // str1.parse::<F>();
    match str1.trim().parse::<u32>() {
        Ok(val) => val,
        Err(_e) => 0,
    }
}

pub fn str_to_float(str1: &String) -> f32 {
    // str1.parse::<F>();
    match str1.trim().parse::<f32>() {
        Ok(val) => val,
        Err(_e) => 0.0,
    }
}

/// 提取协议中的协议标签
pub fn extract_tag(tag_line: &String) -> ProtocolTag {
    let reg = Regex::new("(^#E([^:])+)").unwrap();
    let mut tag_name = String::new();

    for elem in reg.captures_iter(tag_line.trim()) {
        tag_name = String::from(&elem[0])
    }

    match tag_name.as_str() {
        "#EXTM3U" => ProtocolTag::Extm3U,
        "#EXTINF" => ProtocolTag::Extinf,
        "#EXT-X-STREAM-INF" => ProtocolTag::ExtXStreamInf,
        "#EXT-X-PLAYLIST-TYPE" => ProtocolTag::ExtXPlaylistType,
        "#EXT-X-ENDLIST" => ProtocolTag::ExtXEndlist,
        "#EXT-X-KEY" => ProtocolTag::ExtXKey,
        _ => ProtocolTag::Value,
    }
}

///
/// 清洗数据，将字符串形式的内容转成字符串数组
///
/// - `str_hls` 字符串形式的协议内容
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

///
/// 结构化协议标签内的参数
///
/// - `str_protocol` 字符串形式的协议标签
///
pub fn destructure_params(str_protocol: &String) -> Option<ProtocolParam> {
    // 是否为标签协议
    if !is_hls_tag(str_protocol) {
        return None;
    }

    let reg: Regex = Regex::new("(^#E([^:])+)").unwrap();
    let mut vec_proto1: Vec<&str> = reg.split(str_protocol.trim()).collect(); // 拆分协议标签和参数

    // let vec_proto1: Vec<&str> = str_protocol.split(":").collect();

    // 标签是否有参数
    if (vec_proto1[0].chars().count() == 0) && (vec_proto1[1].chars().count() == 0) {
        return None;
    }

    vec_proto1[1] = &vec_proto1[1][1..];
    let reg: Regex = Regex::new("[',\"]").unwrap();
    let vec_params: Vec<&str> = vec_proto1[1].split(",").collect(); // sdfs
    let mut protocol_map: HashMap<String, String> = HashMap::new();
    let mut protocol_arr: Vec<String> = vec![];
    for params in vec_params {
        let vec_p: Vec<&str> = params.split("=").collect();
        if vec_p.len() < 2 {
            // 数组形式
            if params.trim().len() != 0 {
                protocol_arr.push(params.trim().to_string());
            }
        } else {
            // key/value形式的参数
            protocol_map.insert(
                (vec_p[0]).trim().to_string().to_lowercase(),
                (reg.replace_all(vec_p[1], "")).trim().to_string(),
            );
        }
    }
    if protocol_map.len() > 0 {
        return Some(ProtocolParam::Map(protocol_map));
    } else if protocol_arr.len() > 0 {
        return Some(ProtocolParam::Array(protocol_arr));
    }
    None
}

///
/// 是否为hls协议的标签
///
fn is_hls_tag(str_protocol: &String) -> bool {
    extract_tag(str_protocol) != ProtocolTag::Value
}

pub fn join_url(url: &String, base_url: &String) -> Result<String, ParseError> {
    if Regex::new("^https?://").unwrap().is_match(url) {
        return Ok(url.clone().to_string());
    } else {
        let base = Url::parse(base_url).expect("hardcoded URL is known to be valid");
        let joined = base.join(url)?;

        Ok(joined.as_ref().to_string())
    }
}

// ***************************************

#[cfg(test)]
mod tests {
    use super::*;
    // use crate::test_data;
    #[test]
    fn test_clean_content() {
        let str_master = "
    
        #EXTM3U
    
    #EXT-X-STREAM-INF:PROGRAM-ID=1,BANDWIDTH=1064000
        1000kbps.m3u8
        
            #EXT-X-STREAM-INF:PROGRAM-ID=1,BANDWIDTH=564000
        500kbps.m3u8
    #EXT-X-STREAM-INF:PROGRAM-ID=1,BANDWIDTH=282000
        250kbps.m3u8
        #EXT-X-STREAM-INF:PROGRAM-ID=1,BANDWIDTH=2128000
        2000kbps.m3u8"
            .to_string();
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

    #[test]
    fn test_destructure_params_map0() {
        match super::destructure_params(&String::from("#EXT-X-ENDLIST")) {
            Some(params) => match params {
                ProtocolParam::Map(_map) => {}
                ProtocolParam::Array(_arr) => {}
            },
            None => {
                assert_eq!(2, 2);
            }
        };
    }

    #[test]
    fn test_destructure_params_map1() {
        match super::destructure_params(&String::from(
            "#EXT-X-STREAM-INF:PROGRAM-ID=1,BANDWIDTH=1064000",
        )) {
            Some(params) => match params {
                ProtocolParam::Map(map) => {
                    assert_eq!(2, map.len());
                    assert_eq!("1", super::map_val(&map, "program-id"));
                    assert_eq!("1064000", super::map_val(&map, "bandwidth"));
                }
                ProtocolParam::Array(_arr) => {}
            },
            None => {}
        };
    }

    #[test]
    fn test_destructure_params_map2() {
        match super::destructure_params(&String::from(
            "#EXT-X-KEY:METHOD=AES-128,URI=\"https://ts4.chinalincoln.com:9999/20210419/OvroTYry/1000kb/hls/key.key\"",
        )) {
            Some(params) => match params {
                ProtocolParam::Map(map) => {
                    assert_eq!(2, map.len());
                    assert_eq!("AES-128", super::map_val(&map, "method"));
                    assert_eq!("https://ts4.chinalincoln.com:9999/20210419/OvroTYry/1000kb/hls/key.key", super::map_val(&map, "uri"));
                }
                ProtocolParam::Array(_arr) => {}
            },
            None => {}
        };
    }

    #[test]
    fn test_destructure_params_array() {
        let str_pro = String::from(" #EXTINF:,4.128,,");
        super::destructure_params(&str_pro).map(|params| match params {
            ProtocolParam::Map(_map) => {}
            ProtocolParam::Array(_arr) => {
                assert_eq!(1, _arr.len());
                assert_eq!("4.128", _arr[0]);
            }
        });
    }

    #[test]
    fn test_is_hls_tag() {
        let res1 = super::is_hls_tag(&String::from(
            "#EXT-X-STREAM-INF:PROGRAM-ID=1,BANDWIDTH=1064000",
        ));
        assert_eq!(true, res1);
        let res2 = super::is_hls_tag(&String::from("250kbps.m3u8"));
        assert_eq!(false, res2);
    }

    #[test]
    fn test_map_val() {
        let mut protocol_map: HashMap<String, String> = HashMap::new();
        protocol_map.insert("key1".to_string(), " 1 ".to_string());
        protocol_map.insert("key2".to_string(), "1".to_string());
        assert_eq!("1", super::map_val(&protocol_map, "key1"));
        assert_eq!("1", super::map_val(&protocol_map, "key2"));
        assert_eq!("", super::map_val(&protocol_map, "key3"));
    }

    #[test]
    fn test_str_to_int() {
        assert_eq!(1, super::str_to_int(&String::from("1")));
        assert_eq!(10000001, super::str_to_int(&String::from("10000001")));
        assert_eq!(1, super::str_to_int(&String::from("0000001")));
        assert_eq!(0, super::str_to_int(&String::from("fff")));
    }

    #[test]
    fn test_str_to_float() {
        assert_eq!(1.1, super::str_to_float(&String::from("1.1")));
        assert_eq!(0.0221, super::str_to_float(&String::from("0.0221")));
        assert_eq!(0.0, super::str_to_float(&String::from("A")));
    }

    #[test]
    fn test_join_url() {
        let base_url = "http://www.zhisland.com/path1/".to_string();
        let url1 = "/url1/250kbps.m3u8".to_string();
        let url2 = "250kbps.m3u8".to_string();
        let url3 = "http://www.baidu.com/250kbps.m3u8".to_string();
        assert_eq!(
            super::join_url(&url1, &base_url).unwrap(),
            format!("{}{}", "http://www.zhisland.com", url1)
        );

        assert_eq!(
            super::join_url(&url2, &base_url).unwrap(),
            format!("{}{}", base_url, url2)
        );

        assert_eq!(
            super::join_url(&url3, &base_url).unwrap(),
            format!("{}", url3)
        );
    }
}
