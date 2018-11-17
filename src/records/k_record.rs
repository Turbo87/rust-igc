use regex::bytes::Regex;

use ::{Error, Result, Time};
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
}

impl AdditionSupport for KRecord {
    fn get_addition(&self, code: &AdditionCode) -> Option<&[u8]> {
        self.additions.get(code).map(Vec::as_ref)
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
