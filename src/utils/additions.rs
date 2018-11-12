use ::utils::num::buf_to_uint;


#[derive(Debug, PartialEq, Eq)]
pub struct AdditionDef {
    /// Three-Letter-Code describing the addition type
    pub code: String,

    /// Index of the first byte of the addition in the record (1-indexed!).
    pub start_byte: u8,

    /// Index of the last byte of the addition in the record (1-indexed!).
    pub end_byte: u8,
}

impl AdditionDef {
    pub fn new<T: Into<String>>(code: T, start_byte: u8, end_byte: u8) -> AdditionDef {
        AdditionDef { code: code.into(), start_byte, end_byte }
    }

    pub unsafe fn parse_unchecked(line: &[u8]) -> AdditionDef {
        debug_assert_eq!(line.len(), 7);
        debug_assert!(line.is_ascii());

        let start_byte = buf_to_uint(&line[0..2]);
        let end_byte = buf_to_uint(&line[2..4]);
        let code = std::str::from_utf8_unchecked(&line[4..7]);

        AdditionDef::new(code, start_byte, end_byte)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let def = unsafe { AdditionDef::parse_unchecked(b"3638FXA") };
        assert_eq!(def, AdditionDef::new("FXA", 36, 38));
    }

    #[test]
    fn test_example_2() {
        let def = unsafe { AdditionDef::parse_unchecked(b"3940SIU") };
        assert_eq!(def, AdditionDef::new("SIU", 39, 40));
    }

    proptest! {
        #[test]
        #[allow(unused_must_use)]
        fn parse_unchecked_doesnt_crash(s in r"[0-9]{4}[A-Z0-9]{3}") {
            unsafe { AdditionDef::parse_unchecked(s.as_bytes()) }
        }
    }
}
