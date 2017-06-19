use chrono::NaiveTime;

use super::ParseError;

pub fn parse_time(input: &[u8]) -> Result<NaiveTime, ParseError> {
    debug_assert!(input.len() == 6);

    let str = String::from_utf8(input.to_vec())?;

    let h = str[0..2].parse::<u32>()?;
    let m = str[2..4].parse::<u32>()?;
    let s = str[4..6].parse::<u32>()?;

    NaiveTime::from_hms_opt(h, m, s).ok_or_else(|| ParseError::InvalidTime(str))
}

#[cfg(test)]
mod tests {
    use chrono::NaiveTime;
    use super::parse_time;

    #[test]
    fn test_time() {
        assert_eq!(parse_time(b"000000").unwrap(), NaiveTime::from_hms(0, 0, 0));
        assert_eq!(parse_time(b"123456").unwrap(), NaiveTime::from_hms(12, 34, 56));
        assert_eq!(parse_time(b"235959").unwrap(), NaiveTime::from_hms(23, 59, 59));
        assert!(parse_time(b"612345").is_err());
    }
}
