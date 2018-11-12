use regex::bytes::Regex;

use ::utils::num::buf_to_uint;
use ::utils::additions::AdditionDef;
use ::new_api::{Error, Result};


// Examples:
//
// I023638FXA3941ENL
// I013638ENL

#[derive(Debug, PartialEq, Eq)]
pub struct IRecord {
    /// Number of additions contained in `additions_def`.
    ///
    /// Note that this is what the original record claimed,
    /// not necessarily what is actually available.
    pub num_additions: u8,

    /// B-Record addition definitions
    pub addition_defs: Vec<AdditionDef>,
}

impl IRecord {
    pub fn parse(line: &[u8]) -> Result<IRecord> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(?x-u)
                ^I                          # record typ
                (\d{2})                     # number of additions
                ((?:\d{4}[[:alnum:]]{3})+)  # additions
            ").unwrap();
        }

        let cap = RE.captures(line).ok_or_else(|| Error::invalid_record(line))?;

        let num_additions: u8 = buf_to_uint(&cap[1]);

        let addition_defs: Vec<_> = cap[2].chunks(7)
            .map(|bytes| unsafe { AdditionDef::parse_unchecked(bytes) })
            .collect();

        Ok(IRecord { num_additions, addition_defs })
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use ::utils::addition_code::AdditionCode;

    #[test]
    fn test_example_1() {
        let record = IRecord::parse(b"I023638FXA3941ENL").unwrap();
        assert_eq!(record.num_additions, 2);
        assert_eq!(record.addition_defs, vec![
            AdditionDef::new(AdditionCode::FXA, 36, 38),
            AdditionDef::new(AdditionCode::ENL, 39, 41),
        ]);
    }

    #[test]
    fn test_example_2() {
        let record = IRecord::parse(b"I013638ENL").unwrap();
        assert_eq!(record.num_additions, 1);
        assert_eq!(record.addition_defs, vec![
            AdditionDef::new(AdditionCode::ENL, 36, 38),
        ]);
    }

    proptest! {
        #[test]
        #[allow(unused_must_use)]
        fn parse_doesnt_crash(s in r"\PC*") {
            IRecord::parse(s.as_bytes());
        }
    }
}
