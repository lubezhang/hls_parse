use hls_parse;
use hls_parse::parse_protocol::ParseProtocol;

#[test]
fn hls_parse_add() {
    assert_eq!(1, ParseProtocol::parse());
}