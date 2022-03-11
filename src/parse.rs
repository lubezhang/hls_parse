use super::types::*;

/**
 * 函数功能说明
 */
pub fn parse_protocol () -> HLSProtocol {
    let protocol = HLSProtocol {
        ext_m3u: String::from("1"),
        ext_playlist_type: PlayListType::Master
    };
    // str;

    return protocol;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_protocol_parse() {
        let mut p = parse_protocol();
        assert_eq!("1", p.ext_m3u);
        p.ext_m3u = String::from("2");
        assert_eq!("2", p.ext_m3u);
        // assert_eq!(PlayListType::Master, p.ext_playlist_type);
        // println!("{:?}", p);
    }
}