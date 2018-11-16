use regex::bytes::Regex;

use ::{Error, Result};
use ::utils::num::parse_int;
use ::utils::additions::AdditionDef;


#[derive(Debug, PartialEq, Eq)]
pub struct JRecord {
    /// Number of additions contained in `additions_def`.
    ///
    /// Note that this is what the original record claimed,
    /// not necessarily what is actually available.
    pub num_additions: u8,

    /// B-Record addition definitions
    pub addition_defs: Vec<AdditionDef>,
}

impl JRecord {
    pub fn parse(line: &[u8]) -> Result<JRecord> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(?x-u)
                ^J
                (\d{2})                     # number of additions
                ((?:\d{4}[[:alnum:]]{3})+)  # additions
            ").unwrap();
        }

        let cap = RE.captures(line).ok_or_else(|| Error::invalid_record(line))?;

        let num_additions = parse_int(&cap[1]).unwrap();

        let addition_defs: Vec<_> = cap[2].chunks(7)
            .map(|bytes| unsafe { AdditionDef::parse_unchecked(bytes) })
            .collect();

        Ok(JRecord { num_additions, addition_defs })
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use ::utils::additions::AdditionCode;

    #[test]
    fn test_parse() {
        let record = JRecord::parse(b"J010810HDT").unwrap();
        assert_eq!(record.num_additions, 1);
        assert_eq!(record.addition_defs, vec![
            AdditionDef::new(AdditionCode::HDT, 8, 10),
        ]);
    }

    proptest! {
        #[test]
        #[allow(unused_must_use)]
        fn parse_doesnt_crash(s in r"\PC*") {
            JRecord::parse(s.as_bytes());
        }
    }
}
