use hls_parse::hls::*;
use hls_parse::types::*;
mod common;

#[test]
fn hls_parse_for_string() {
    let str_master = common::get_data_master();

    let mut protocol1: HLS = HLS::new();
    assert_eq!("#EXTM3U", protocol1.ext_m3u);
    assert_eq!(PlayListType::Master, protocol1.ext_playlist_type);

    protocol1.parse(&str_master);
    assert_eq!(PlayListType::Master, protocol1.ext_playlist_type);
    assert_eq!(4, protocol1.ext_stream_inf.len());
}
