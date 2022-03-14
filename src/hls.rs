use crate::types::*;

#[derive(Debug, Clone)]
pub struct HLS {
    /** 协议根元素，在协议文件的第一行 */
    pub ext_m3u: String,
    /** 协议类型 */
    pub ext_playlist_type: PlayListType
}

impl HLS {
    pub fn new () -> HLS {
        let hls = HLS {
            ext_m3u: String::from("1"),
            ext_playlist_type: PlayListType::Master
        };

        return hls;
    }

    pub fn parse (self: &mut Self) {
        self.ext_m3u = String::from("2");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hls_parse() {
        let mut protocol1 = HLS::new();
        assert_eq!("1", protocol1.ext_m3u);
        protocol1.parse();
        assert_eq!("2", protocol1.ext_m3u);
    }
}