use utils::num::parse_int;

pub fn parse_latitude_unchecked(bytes: &[u8]) -> f64 {
    debug_assert_eq!(bytes.len(), 8);

    let dec: u32 = parse_int(&bytes[0..2]).unwrap();
    let min: u32 = parse_int(&bytes[2..7]).unwrap();

    let abs_value = dec as f64 + min as f64 / 60000.;

    if bytes[7] == b'S' { -abs_value } else { abs_value }
}

pub fn parse_longitude_unchecked(bytes: &[u8]) -> f64 {
    debug_assert_eq!(bytes.len(), 9);

    let dec: u32 = parse_int(&bytes[0..3]).unwrap();
    let min: u32 = parse_int(&bytes[3..8]).unwrap();

    let abs_value = dec as f64 + min as f64 / 60000.;

    if bytes[8] == b'W' { -abs_value } else { abs_value }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn test_parse_latitude_unchecked() {
        assert_relative_eq!(parse_latitude_unchecked(b"5016925N"), 50.28208333333333);
    }

    prop_compose! {
        fn latitude()(value in any::<i32>()) -> f64 {
            (value as f64 / std::i32::MAX as f64) * 90.
        }
    }

    proptest! {
        #[test]
        fn test_parse_latitude_unchecked_with_random_data(value in latitude()) {
            let abs_value = value.abs();
            let dec = abs_value.trunc();
            let min = ((abs_value - dec) * 60000.).round();
            let sign = if value < 0. { 'S' } else { 'N' };
            let input = format!("{:02}{:05}{}", dec, min, sign);

            let result = parse_latitude_unchecked(input.as_bytes());
            assert_abs_diff_eq!(result, value, epsilon = 0.0001);
        }
    }

    #[test]
    fn test_parse_longitude_unchecked() {
        assert_relative_eq!(parse_longitude_unchecked(b"00953112E"), 9.8852);
    }

    prop_compose! {
        fn longitude()(value in any::<i32>()) -> f64 {
            (value as f64 / std::i32::MAX as f64) * 180.
        }
    }

    proptest! {
        #[test]
        fn test_parse_longitude_unchecked_with_random_data(value in longitude()) {
            let abs_value = value.abs();
            let dec = abs_value.trunc();
            let min = ((abs_value - dec) * 60000.).round();
            let sign = if value < 0. { 'W' } else { 'E' };
            let input = format!("{:03}{:05}{}", dec, min, sign);

            let result = parse_longitude_unchecked(input.as_bytes());
            assert_abs_diff_eq!(result, value, epsilon = 0.0001);
        }
    }
}
