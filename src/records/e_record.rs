use regex::bytes::Regex;

use ::{Error, Result};
use ::utils::datetime::Time;
use ::utils::events::EventCode;
use ::utils::text::as_text;

#[derive(Debug, PartialEq, Eq)]
pub struct ERecord {
    pub time: Time,
    pub code: EventCode,
    pub text: Option<String>,
}

impl ERecord {
    pub fn parse(line: &[u8]) -> Result<ERecord> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(?x-u)
                ^E
                (\d{6})           # UTC time
                ([A-Z0-9]{3})     # code
                (.+)?             # text
            ").unwrap();
        }

        let cap = RE.captures(line).ok_or_else(|| Error::invalid_record(line))?;

        let time = Time::parse_unchecked(&cap[1]);
        let code = unsafe { EventCode::from_bytes_unchecked(&cap[2]) };
        let text = match cap.get(3) {
            None => None,
            Some(m) => Some(as_text(m.as_bytes()).ok_or_else(|| Error::invalid_record(line))?),
        };

        Ok(ERecord { time, code, text })
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let record = ERecord::parse(b"E104533PEV").unwrap();
        assert_eq!(record.time, Time::from_hms(10, 45, 33));
        assert_eq!(record.code, EventCode::PEV);
        assert_eq!(record.text, None);

        let record = ERecord::parse(b"E104544ATS102312").unwrap();
        assert_eq!(record.time, Time::from_hms(10, 45, 44));
        assert_eq!(record.code, EventCode::ATS);
        assert_eq!(record.text, Some("102312".into()));
    }

    proptest! {
        #[test]
        #[allow(unused_must_use)]
        fn parse_doesnt_crash(s in r"\PC*") {
            ERecord::parse(s.as_bytes());
        }
    }
}
