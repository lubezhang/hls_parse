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
