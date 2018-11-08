use super::super::ParseError;

#[derive(Debug, Eq, PartialEq)]
pub struct Time {
    hour: u8,
    minute: u8,
    second: u8,
}

impl Time {
    pub fn from_hms(hour: u8, minute: u8, second: u8) -> Time {
        Time { hour,  minute, second }
    }

    pub fn hour(&self) -> u8 { self.hour }
    pub fn minute(&self) -> u8 { self.minute }
    pub fn second(&self) -> u8 { self.second }
}

pub fn parse_time(input: &str) -> Result<Time, ParseError> {
    if input.len() != 6 {
        return Err(ParseError::InvalidTime(input.into()));
    }

    if !input.is_ascii() {
        return Err(ParseError::InvalidTime(input.into()));
    }

    let hour = input[0..2].parse::<u8>()?;
    let minute = input[2..4].parse::<u8>()?;
    let second = input[4..6].parse::<u8>()?;

    if hour >= 24 || minute >= 60 || second >= 60 {
        return Err(ParseError::InvalidTime(input.into()));
    }

    Ok(Time::from_hms(hour, minute, second))
}

#[cfg(test)]
mod tests {
    use super::{parse_time, Time};

    #[test]
    fn test_time() {
        assert_eq!(parse_time("000000").unwrap(), Time::from_hms(0, 0, 0));
        assert_eq!(parse_time("123456").unwrap(), Time::from_hms(12, 34, 56));
        assert_eq!(parse_time("235959").unwrap(), Time::from_hms(23, 59, 59));
        assert!(parse_time("612345").is_err());
    }

    proptest! {
        #[test]
        #[allow(unused_must_use)]
        fn doesnt_crash(s in r"\PC*") {
            parse_time(&s);
        }

        #[test]
        fn parses_all_valid_times(h in 0..24u8, m in 0..60u8, s in 0..60u8) {
            let time = parse_time(&format!("{:02}{:02}{:02}", h, m, s)).unwrap();
            prop_assert_eq!(time, Time::from_hms(h, m, s));
        }
    }
}
