use hls_parse::hls::*;

#[test]
fn hls_parse_for_string() {
    let mut protocol1 = HLS::new();
    assert_eq!("1", protocol1.ext_m3u);
    protocol1.parse();
    // assert_eq!("1", protocol1.ext_m3u);
    assert_eq!("2", protocol1.ext_m3u);
}