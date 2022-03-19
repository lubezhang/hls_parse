#[allow(dead_code)]
pub fn setup() {
    // setup code specific to your library's tests would go here
}

/// 返回主文件内容的测试数据
#[allow(dead_code)]
pub fn get_data_master() -> String {
    let str_master = "
    
        #EXTM3U
    
        #EXT-X-STREAM-INF:PROGRAM-ID=1,BANDWIDTH=1064000
        1000kbps.m3u8
        
        #EXT-X-STREAM-INF:PROGRAM-ID=1,BANDWIDTH=564000
        500kbps.m3u8
        #EXT-X-STREAM-INF:PROGRAM-ID=1,BANDWIDTH=282000
        250kbps.m3u8
        #EXT-X-STREAM-INF:PROGRAM-ID=1,BANDWIDTH=2128000
        2000kbps.m3u8";
    return str_master.to_string();
}

#[allow(dead_code)]
pub fn get_data_vod() -> String {
    let str_vod = "
    
    #EXTM3U
    #EXT-X-VERSION:3
    #EXT-X-TARGETDURATION:5
    #EXT-X-PLAYLIST-TYPE:VOD
    #EXT-X-MEDIA-SEQUENCE:0
    #EXTINF:4.128,
    https://ts4.chinalincoln.com:9999/20210419/OvroTYry/1000kb/hls/YMgVK9tU.ts
    #EXTINF:3.127,
    https://ts4.chinalincoln.com:9999/20210419/OvroTYry/1000kb/hls/3e9Ux5sa.ts
    #EXT-X-KEY:METHOD=AES-128,URI=\"https://ts4.chinalincoln.com:9999/20210419/OvroTYry/1000kb/hls/key.key\"
    #EXTINF:3.461,
    https://ts4.chinalincoln.com:9999/20210419/OvroTYry/1000kb/hls/ZXyddo0d.ts
    #EXTINF:2.043,
    https://ts4.chinalincoln.com:9999/20210419/OvroTYry/1000kb/hls/FsOLD1kG.ts
    #EXTINF:3.127,
    https://ts4.chinalincoln.com:9999/20210419/OvroTYry/1000kb/hls/J1Xo6bvk.ts
    #EXTINF:4.253,
    https://ts4.chinalincoln.com:9999/20210419/OvroTYry/1000kb/hls/70zdcWHN.ts
    #EXTINF:3.336,
    https://ts4.chinalincoln.com:9999/20210419/OvroTYry/1000kb/hls/hZO2SoIF.ts
    #EXTINF:0.917,
    https://ts4.chinalincoln.com:9999/20210419/OvroTYry/1000kb/hls/NtSoA2hU.ts
    #EXT-X-KEY:METHOD=AES-128,URI=\"https://ts4.chinalincoln.com:9999/20210419/OvroTYry/1000kb/hls/key.key\"
    #EXTINF:3.127,
    https://ts4.chinalincoln.com:9999/20210419/OvroTYry/1000kb/hls/E3jKvOa0.ts
    #EXTINF:3.044,
    https://ts4.chinalincoln.com:9999/20210419/OvroTYry/1000kb/hls/NeK9QXha.ts
    #EXTINF:3.002,
    https://ts4.chinalincoln.com:9999/20210419/OvroTYry/1000kb/hls/q51WSnXk.ts
    #EXT-X-ENDLIST";
    return str_vod.to_string();
}
