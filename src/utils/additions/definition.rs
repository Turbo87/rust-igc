use ::utils::num::parse_int;
use super::AdditionCode;

#[derive(Debug, PartialEq, Eq)]
pub struct AdditionDef {
    /// Three-Letter-Code describing the addition type
    pub code: AdditionCode,

    /// Index of the first byte of the addition in the record (1-indexed!).
    pub start_byte: u8,

    /// Index of the last byte of the addition in the record (1-indexed!).
    pub end_byte: u8,
}

impl AdditionDef {
    pub fn new(code: AdditionCode, start_byte: u8, end_byte: u8) -> AdditionDef {
        AdditionDef { code, start_byte, end_byte }
    }

    pub unsafe fn parse_unchecked(line: &[u8]) -> AdditionDef {
        debug_assert_eq!(line.len(), 7);
        debug_assert!(line.is_ascii());

        let start_byte = parse_int(&line[0..2]).unwrap();
        let end_byte = parse_int(&line[2..4]).unwrap();
        let code = AdditionCode::from_bytes_unchecked(&line[4..7]);

        AdditionDef::new(code, start_byte, end_byte)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_def_example_1() {
        let def = unsafe { AdditionDef::parse_unchecked(b"3638FXA") };
        assert_eq!(def, AdditionDef::new(AdditionCode::FXA, 36, 38));
    }

    #[test]
    fn parse_def_example_2() {
        let def = unsafe { AdditionDef::parse_unchecked(b"3940SIU") };
        assert_eq!(def, AdditionDef::new(AdditionCode::SIU, 39, 40));
    }

    proptest! {
        #[test]
        #[allow(unused_must_use)]
        fn parse_def_doesnt_crash(s in r"[0-9]{4}[A-Z0-9]{3}") {
            unsafe { AdditionDef::parse_unchecked(s.as_bytes()) }
        }
    }
}
