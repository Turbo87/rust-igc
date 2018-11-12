use std::collections::HashMap;

use ::{Error, Result};
use super::{AdditionCode, AdditionDef};

pub type AdditionsMap = HashMap<AdditionCode, Vec<u8>>;

pub trait ParseAdditions {
    fn parse(&self, bytes: &[u8]) -> Result<AdditionsMap>;
}

impl ParseAdditions for Vec<AdditionDef> {
    fn parse(&self, bytes: &[u8]) -> Result<AdditionsMap> {
        let input_length = bytes.len();

        let mut additions = AdditionsMap::with_capacity(self.len());

        for AdditionDef { start_byte, end_byte, code } in self {
            debug_assert!(*start_byte >= 1);
            debug_assert!(*start_byte <= *end_byte);

            if *end_byte as usize > input_length || *start_byte as usize > input_length {
                return Err(Error::invalid_record(bytes));
            }

            let value = &bytes[(*start_byte as usize - 1)..(*end_byte as usize)];

            additions.insert(code.clone(), value.into());
        }

        Ok(additions)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_example_1() {
        let additions_defs = vec![AdditionDef::new(AdditionCode::HDT, 8, 12)];
        let additions = additions_defs.parse(b"K12345600090").unwrap();
        assert_eq!(additions.len(), 1);
        assert_eq!(additions.get(&AdditionCode::HDT), Some(&b"00090".to_vec()));
    }

    #[test]
    fn parse_example_2() {
        let additions_defs = vec![
            AdditionDef::new(AdditionCode::FXA, 36, 38),
            AdditionDef::new(AdditionCode::SIU, 39, 40),
        ];
        let additions = additions_defs.parse(b"B0818265049456N00610940EA011730132000308").unwrap();
        assert_eq!(additions.len(), 2);
        assert_eq!(additions.get(&AdditionCode::FXA), Some(&b"003".to_vec()));
        assert_eq!(additions.get(&AdditionCode::SIU), Some(&b"08".to_vec()));
    }

    #[test]
    fn parse_errors() {
        let additions_defs = vec![AdditionDef::new(AdditionCode::HDT, 8, 12)];
        let result = additions_defs.parse(b"K1234560009");
        assert_eq!(format!("{}", result.unwrap_err()), "invalid record: K1234560009");
    }

    proptest! {
        #[test]
        #[allow(unused_must_use)]
        fn parse_doesnt_crash(s in r"[0-9]{4}[A-Z0-9]{3}", t in r"\PC*") {
            let addition_def = unsafe { AdditionDef::parse_unchecked(s.as_bytes()) };
            if addition_def.start_byte >= 1 && addition_def.start_byte <= addition_def.end_byte {
                let additions_defs = vec![addition_def];
                additions_defs.parse(t.as_bytes());
            }
        }
    }

}
