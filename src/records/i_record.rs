use std::collections::HashMap;

use super::super::{Result, ParseError};

// Examples:
//
// I023638FXA3941ENL
// I013638ENL

#[derive(Debug)]
pub struct IRecord {
    additions: HashMap<String, (u8, u8)>,
}

impl IRecord {
    pub(crate) fn parse(input: &str) -> Result<Self> {
        debug_assert_eq!(&input[0..1], "I");

        if !input.is_ascii() {
            return Err(ParseError::unexpected("ASCII characters", input));
        }

        let input_length = input.len();
        if input_length < 3 {
            return Err(ParseError::unexpected("at least 3 characters", input));
        }

        let number_of_additions = &input[1..3];
        let number_of_additions = number_of_additions.parse()
            .map_err(|_| ParseError::unexpected("digits", number_of_additions))?;

        if number_of_additions * 7 != input_length - 3 {
            return Err(ParseError::unexpected(
                format!("{} additions (= {} characters)", number_of_additions, number_of_additions * 7,),
                format!("{} characters", input_length - 3),
            ));
        }

        let mut additions = HashMap::with_capacity(number_of_additions);

        for i in 0..number_of_additions {
            let start_byte = &input[(3 + i * 7)..(3 + i * 7 + 2)];
            let start_byte = start_byte.parse()
                .map_err(|_| ParseError::unexpected("digits", start_byte))?;

            let end_byte = &input[(3 + i * 7 + 2)..(3 + i * 7 + 4)];
            let end_byte = end_byte.parse()
                .map_err(|_| ParseError::unexpected("digits", end_byte))?;

            let code = &input[(3 + i * 7 + 4)..(3 + i * 7 + 7)];

            additions.insert(code.into(), (start_byte, end_byte));
        }

        Ok(IRecord { additions })
    }
}

#[cfg(test)]
mod tests {
    use galvanic_assert::matchers::collection::*;

    use super::IRecord;

    #[test]
    fn test_example_1() {
        let record = IRecord::parse("I023638FXA3941ENL").unwrap();
        assert_eq!(record.additions.len(), 2);
        assert_that!(&record.additions, has_entry("ENL".into(), (39, 41)));
        assert_that!(&record.additions, has_entry("FXA".into(), (36, 38)));
    }

    #[test]
    fn test_example_2() {
        let record = IRecord::parse("I013638ENL").unwrap();
        assert_eq!(record.additions.len(), 1);
        assert_that!(&record.additions, has_entry("ENL".into(), (36, 38)));
    }

    #[test]
    fn test_errors() {
        assert_eq!(format!("{}", IRecord::parse("I010000ÄÖÜ").unwrap_err()),
                   "Expected: ASCII characters; Found: I010000ÄÖÜ");

        assert_eq!(format!("{}", IRecord::parse("I").unwrap_err()),
                   "Expected: at least 3 characters; Found: I");

        assert_eq!(format!("{}", IRecord::parse("IAB").unwrap_err()),
                   "Expected: digits; Found: AB");

        assert_eq!(format!("{}", IRecord::parse("I01000").unwrap_err()),
                   "Expected: 1 additions (= 7 characters); Found: 3 characters");

        assert_eq!(format!("{}", IRecord::parse("I-1").unwrap_err()),
                   "Expected: digits; Found: -1");

        assert_eq!(format!("{}", IRecord::parse("I010a02ABC").unwrap_err()),
                   "Expected: digits; Found: 0a");

        assert_eq!(format!("{}", IRecord::parse("I0100-1ABC").unwrap_err()),
                   "Expected: digits; Found: -1");
    }

    proptest! {
        #[test]
        #[allow(unused_must_use)]
        fn doesnt_crash(s in r"I\PC*") {
            IRecord::parse(&s);
        }

        #[test]
        fn parses_all_valid_times(additions in "([0-9]{4}[A-Z]{3})+") {
            let record = format!("I{:02}{}", additions.len() / 7, additions);
            prop_assert!(IRecord::parse(&record).is_ok());
        }
    }
}
