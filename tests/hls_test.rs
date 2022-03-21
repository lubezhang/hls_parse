use hls_parse::hls::*;
use hls_parse::types::*;
mod common;

#[test]
fn hls_parse_master() {
    let str_master = common::get_data_master();

    let mut protocol1: HLS = HLS::new();
    assert_eq!("#EXTM3U", protocol1.ext_m3u);
    assert_eq!(PlayListType::Master, protocol1.ext_playlist_type);

    protocol1.parse(&str_master);
    assert_eq!(PlayListType::Master, protocol1.ext_playlist_type);
    assert_eq!(4, protocol1.ext_stream_inf.len());
    assert_eq!(1064000, protocol1.ext_stream_inf[0].bandwidth);
    assert_eq!(564000, protocol1.ext_stream_inf[1].bandwidth);
    assert_eq!("1000kbps.m3u8", protocol1.ext_stream_inf[0].url);
}

#[test]
fn hls_parse_vod() {
    let str_vod = common::get_data_vod();

    let mut protocol1: HLS = HLS::new();
    assert_eq!("#EXTM3U", protocol1.ext_m3u);
    assert_eq!(PlayListType::Master, protocol1.ext_playlist_type);

    protocol1.parse(&str_vod);
    assert_eq!(PlayListType::Vod, protocol1.ext_playlist_type);

    // println!("hsl: {:#?}", protocol1);
}
