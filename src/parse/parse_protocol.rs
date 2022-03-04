#[derive(Debug)]
pub struct ParseProtocol {
    // name: String,
    // age: i32
}

impl ParseProtocol {
    pub fn parse() -> i32 {
        println!("parse_protocol*********************************************");
        return 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_protocol_parse() {
        assert_eq!(1, ParseProtocol::parse());
    }
}