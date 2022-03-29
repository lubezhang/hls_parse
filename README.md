# hls_parse

![crates.io](https://img.shields.io/crates/v/hls_parse?style=plastic)

A Rust library for parsing m3u8 playlists (HTTP Live Streaming) [link](https://tools.ietf.org/html/draft-pantos-http-live-streaming-19).

# Installation
To use this library, add the following dependency to `Cargo.toml`:

```toml
[dependencies]
hls_parse = "0.1.2"
```

Also available on [crates.io](https://crates.io/crates/hls_parse)

# Examples
```rust
use hls_parse::protocol::HLS;
let m3u8 = "#EXTM3U
#EXT-X-TARGETDURATION:10
#EXT-X-VERSION:3
#EXTINF:9.009,
http://media.example.com/first.ts
#EXTINF:9.009,
http://media.example.com/second.ts
#EXTINF:3.003,
http://media.example.com/third.ts
#EXT-X-ENDLIST";
let base_url = "http://media.example.com".to_string();
let mut hls = HLS::new();
hls.set_base_url(&base_url);
hls.parse(&m3u8);
```