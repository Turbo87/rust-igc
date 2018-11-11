use std::ops::{AddAssign, MulAssign};

pub fn buf_to_uint<T>(s: &[u8]) -> T
    where
        T: AddAssign + MulAssign + From<u8>,
{
    let mut sum = T::from(0);
    for digit in s {
        debug_assert!(*digit >= b'0' && *digit <= b'9', "`{}` is not a digit", *digit);

        sum *= T::from(10);
        sum += T::from(*digit - b'0');
    }
    sum
}

pub fn buf_to_int<T>(s: &[u8]) -> T
    where
        T: AddAssign + MulAssign + From<u8> + From<i8>,
{
    let negative = s[0] == b'-';
    if negative {
        let mut value = buf_to_uint::<T>(&s[1..]);
        value *= T::from(-1i8);
        value
    } else {
        buf_to_uint::<T>(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buf_to_uint() {
        assert_eq!(buf_to_uint::<u16>(b"0"), 0);
        assert_eq!(buf_to_uint::<u8>(b"12"), 12);
        assert_eq!(buf_to_uint::<u16>(b"2018"), 2018);
        assert_eq!(buf_to_uint::<u32>(b"2018"), 2018);
    }

    #[test]
    #[should_panic]
    fn test_buf_to_uint_invalid_digit() {
        buf_to_uint::<u32>(b"01d8");
    }

    #[test]
    #[should_panic]
    fn test_buf_to_uint_minus_sign() {
        buf_to_uint::<u32>(b"-018");
    }

    proptest! {
        #[test]
        fn test_buf_to_uint_with_u8(v: u8) {
            let input = format!("{}", v);
            prop_assert_eq!(buf_to_uint::<u8>(input.as_bytes()), v);
        }

        #[test]
        fn test_buf_to_uint_with_u16(v: u16) {
            let input = format!("{}", v);
            prop_assert_eq!(buf_to_uint::<u16>(input.as_bytes()), v);
        }

        #[test]
        fn test_buf_to_uint_with_u32(v: u32) {
            let input = format!("{}", v);
            prop_assert_eq!(buf_to_uint::<u32>(input.as_bytes()), v);
        }

        #[test]
        fn test_buf_to_uint_with_u64(v: u64) {
            let input = format!("{}", v);
            prop_assert_eq!(buf_to_uint::<u64>(input.as_bytes()), v);
        }
    }

    #[test]
    fn test_buf_to_int() {
        assert_eq!(buf_to_int::<i16>(b"0"), 0);
        assert_eq!(buf_to_int::<i16>(b"-12"), -12);
        assert_eq!(buf_to_int::<i16>(b"2018"), 2018);
        assert_eq!(buf_to_int::<i32>(b"-018"), -18);
    }

    #[test]
    #[should_panic]
    fn test_buf_to_int_invalid_digit() {
        buf_to_int::<i32>(b"-0d18");
    }

    proptest! {
        #[test]
        fn test_buf_to_int_with_i16(v: i16) {
            let input = format!("{}", v);
            prop_assert_eq!(buf_to_int::<i16>(input.as_bytes()), v);
        }

        #[test]
        fn test_buf_to_int_with_i32(v: i32) {
            let input = format!("{}", v);
            prop_assert_eq!(buf_to_int::<i32>(input.as_bytes()), v);
        }

        #[test]
        fn test_buf_to_int_with_i64(v: i64) {
            let input = format!("{}", v);
            prop_assert_eq!(buf_to_int::<i64>(input.as_bytes()), v);
        }
    }
}
