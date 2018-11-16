use regex::bytes::Regex;

use ::{Error, Result, Time};
use ::utils::num::parse_int;
use ::utils::additions::*;

#[derive(Debug)]
pub struct KRecord {
    pub time: Time,
    pub additions: AdditionsMap,
}

impl KRecord {
    pub fn parse(line: &[u8]) -> Result<KRecord> {
        Self::parse_with_additions(line, &vec![])
    }

    pub fn parse_with_additions(line: &[u8], addition_defs: &Vec<AdditionDef>) -> Result<KRecord> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(?x-u)
                ^K
                (\d{6})                # UTC time
                (.*)                   # additions
            ").unwrap();
        }

        let cap = RE.captures(line).ok_or_else(|| Error::invalid_record(line))?;

        let time = Time::parse_unchecked(&cap[1]);
        let additions = addition_defs.parse(&line)?;

        Ok(KRecord { time, additions })
    }

    /// Fix accuracy in metres
    pub fn fix_accuracy(&self) -> Option<u16> {
        self.get_three_digit_addition(&AdditionCode::FXA)
    }

    /// Environmental Noise Level
    pub fn enl(&self) -> Option<u16> {
        self.get_three_digit_addition(&AdditionCode::ENL)
    }

    /// Heading True
    pub fn heading(&self) -> Option<u16> {
        let value = self.get_three_digit_addition(&AdditionCode::HDT)?;
        if value < 360 { Some(value) } else { None }
    }

    /// Heading Magnetic
    pub fn heading_magnetic(&self) -> Option<u16> {
        let value = self.get_three_digit_addition(&AdditionCode::HDM)?;
        if value < 360 { Some(value) } else { None }
    }

    fn get_three_digit_addition(&self, code: &AdditionCode) -> Option<u16> {
        let bytes = self.additions.get(code)?;
        if bytes.len() != 3 { return None }
        parse_int::<u16>(bytes)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let addition_defs = vec![
            AdditionDef::new(AdditionCode::HDT, 8, 10),
        ];
        let record = KRecord::parse_with_additions(b"K160310090", &addition_defs).unwrap();
        assert_eq!(record.time, Time::from_hms(16, 3, 10));
        assert_eq!(record.additions.len(), 1);
        assert_eq!(record.additions.get(&AdditionCode::HDT).unwrap(), b"090");
    }

    #[test]
    fn test_headings() {
        let addition_defs = vec![
            AdditionDef::new(AdditionCode::HDT, 8, 10),
            AdditionDef::new(AdditionCode::HDM, 11, 13),
        ];

        let record = KRecord::parse_with_additions(b"K160310090299", &addition_defs).unwrap();
        assert_eq!(record.heading(), Some(90));
        assert_eq!(record.heading_magnetic(), Some(299));

        let record = KRecord::parse_with_additions(b"K160310090???", &addition_defs).unwrap();
        assert_eq!(record.heading(), Some(90));
        assert_eq!(record.heading_magnetic(), None);

        assert!(KRecord::parse_with_additions(b"K160310090", &addition_defs).is_err());
        assert!(KRecord::parse_with_additions(b"K1603100901", &addition_defs).is_err());
    }

    proptest! {
        #[test]
        #[allow(unused_must_use)]
        fn parse_doesnt_crash(s in r"\PC*") {
            KRecord::parse(s.as_bytes());
        }
    }
}
