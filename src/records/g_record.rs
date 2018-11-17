use regex::bytes::Regex;

use ::{Error, Result};
use ::utils::text::as_text;

#[derive(Debug, PartialEq, Eq)]
pub struct GRecord {
    pub text: String,
}

impl GRecord {
    pub fn parse(line: &[u8]) -> Result<GRecord> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(?x-u)
                ^G
                (.+)              # text
            ").unwrap();
        }

        let cap = RE.captures(line).ok_or_else(|| Error::invalid_record(line))?;

        let text = as_text(&cap[1]).ok_or_else(|| Error::invalid_record(line))?;

        Ok(GRecord { text })
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let record = GRecord::parse(b"G0AD873C2B7B20B7BECBFF52F1581601F7AAE3769").unwrap();
        assert_eq!(record.text, "0AD873C2B7B20B7BECBFF52F1581601F7AAE3769");

        assert!(GRecord::parse(b"G").is_err());
    }

    proptest! {
        #[test]
        #[allow(unused_must_use)]
        fn parse_doesnt_crash(s in r"\PC*") {
            GRecord::parse(s.as_bytes());
        }
    }
}
