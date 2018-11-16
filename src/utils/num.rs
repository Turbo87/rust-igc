use std::str::{FromStr, from_utf8_unchecked};

pub fn parse_int<T: FromStr>(bytes: &[u8]) -> Option<T> {
    // `unsafe` here should be okay because `from_str()` converts back
    // to `&[u8]` and only cares about ASCII digits
    let chars = unsafe { from_utf8_unchecked(bytes) };
    <(T)>::from_str(chars).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_int() {
        assert_eq!(parse_int::<u16>(b"0"), Some(0));
        assert_eq!(parse_int::<u8>(b"12"), Some(12));
        assert_eq!(parse_int::<u16>(b"2018"), Some(2018));
        assert_eq!(parse_int::<u32>(b"2018"), Some(2018));
        assert_eq!(parse_int::<u32>(b"01d8"), None);
        assert_eq!(parse_int::<u32>(b"-018"), None);
        assert_eq!(parse_int::<i16>(b"0"), Some(0));
        assert_eq!(parse_int::<i16>(b"-12"), Some(-12));
        assert_eq!(parse_int::<i16>(b"2018"), Some(2018));
        assert_eq!(parse_int::<i32>(b"-018"), Some(-18));
        assert_eq!(parse_int::<i32>(b"-0d18"), None);
    }

    proptest! {
        #[test]
        fn test_parse_int_with_u8(v: u8) {
            let input = format!("{}", v);
            prop_assert_eq!(parse_int::<u8>(input.as_bytes()), Some(v));
        }

        #[test]
        fn test_parse_int_with_u16(v: u16) {
            let input = format!("{}", v);
            prop_assert_eq!(parse_int::<u16>(input.as_bytes()), Some(v));
        }

        #[test]
        fn test_parse_int_with_u32(v: u32) {
            let input = format!("{}", v);
            prop_assert_eq!(parse_int::<u32>(input.as_bytes()), Some(v));
        }

        #[test]
        fn test_parse_int_with_u64(v: u64) {
            let input = format!("{}", v);
            prop_assert_eq!(parse_int::<u64>(input.as_bytes()), Some(v));
        }

        #[test]
        fn test_parse_int_with_i8(v: i8) {
            let input = format!("{}", v);
            prop_assert_eq!(parse_int::<i8>(input.as_bytes()), Some(v));
        }

        #[test]
        fn test_parse_int_with_i16(v: i16) {
            let input = format!("{}", v);
            prop_assert_eq!(parse_int::<i16>(input.as_bytes()), Some(v));
        }

        #[test]
        fn test_parse_int_with_i32(v: i32) {
            let input = format!("{}", v);
            prop_assert_eq!(parse_int::<i32>(input.as_bytes()), Some(v));
        }

        #[test]
        fn test_parse_int_with_i64(v: i64) {
            let input = format!("{}", v);
            prop_assert_eq!(parse_int::<i64>(input.as_bytes()), Some(v));
        }
    }
}
