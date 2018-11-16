use utils::num::parse_int;

#[derive(Debug, Eq, PartialEq)]
pub struct Date {
    /// The IGC format only requires the last two digits of the year...
    pub year: u8,
    pub month: u8,
    pub day: u8,
}

impl Date {
    pub fn new(year: u8, month: u8, day: u8) -> Date {
        Date { year, month, day }
    }

    pub fn parse_unchecked(bytes: &[u8]) -> Date {
        let day = parse_int(&bytes[0..2]).unwrap();
        let month = parse_int(&bytes[2..4]).unwrap();
        let year = parse_int(&bytes[4..6]).unwrap();

        Date::new(year, month, day)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Time {
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
}

impl Time {
    pub fn from_hms(hour: u8, minute: u8, second: u8) -> Time {
        Time { hour,  minute, second }
    }

    pub fn parse_unchecked(bytes: &[u8]) -> Time {
        let hour = parse_int(&bytes[0..2]).unwrap();
        let minute = parse_int(&bytes[2..4]).unwrap();
        let second = parse_int(&bytes[4..6]).unwrap();

        Time::from_hms(hour, minute, second)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_parse_unchecked() {
        assert_eq!(Time::parse_unchecked(b"000000"), Time::from_hms(0, 0, 0));
        assert_eq!(Time::parse_unchecked(b"012345"), Time::from_hms(1, 23, 45));
        assert_eq!(Time::parse_unchecked(b"123456"), Time::from_hms(12, 34, 56));
        assert_eq!(Time::parse_unchecked(b"235959"), Time::from_hms(23, 59, 59));
        assert_eq!(Time::parse_unchecked(b"999999"), Time::from_hms(99, 99, 99));
    }

    proptest! {
        #[test]
        fn test_time_parse_unchecked_with_random_input(input in r"[0-9]{6}") {
            let result = Time::parse_unchecked(input.as_bytes());
            let formatted_result = format!("{:02}{:02}{:02}", result.hour, result.minute, result.second);
            prop_assert_eq!(formatted_result, input);
        }
    }

    #[test]
    fn test_date_parse_unchecked() {
        assert_eq!(Date::parse_unchecked(b"000000"), Date::new(0, 0, 0));
        assert_eq!(Date::parse_unchecked(b"012345"), Date::new(45, 23, 1));
        assert_eq!(Date::parse_unchecked(b"123456"), Date::new(56, 34, 12));
        assert_eq!(Date::parse_unchecked(b"235959"), Date::new(59, 59, 23));
        assert_eq!(Date::parse_unchecked(b"999999"), Date::new(99, 99, 99));
    }

    proptest! {
        #[test]
        fn test_date_parse_unchecked_with_random_input(input in r"[0-9]{6}") {
            let result = Date::parse_unchecked(input.as_bytes());
            let formatted_result = format!("{:02}{:02}{:02}", result.day, result.month, result.year);
            prop_assert_eq!(formatted_result, input);
        }
    }
}
