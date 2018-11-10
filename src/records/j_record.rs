use ::Result;
use ::parsers::additions::{AdditionsDeclMap, parse_from_record_line};

// Examples:
//
// I023638FXA3941ENL
// I013638ENL

#[derive(Debug)]
pub struct JRecord {
    pub additions: AdditionsDeclMap,
}

impl JRecord {
    pub(crate) fn parse(input: &str) -> Result<Self> {
        debug_assert_eq!(&input[0..1], "J");

        parse_from_record_line(input)
            .map(|additions| JRecord { additions })
    }
}

#[cfg(test)]
mod tests {
    use galvanic_assert::matchers::collection::*;

    use super::JRecord;

    #[test]
    fn test_example_1() {
        let record = JRecord::parse("J023638FXA3941ENL").unwrap();
        assert_eq!(record.additions.len(), 2);
        assert_that!(&record.additions, has_entry("ENL".into(), (39, 41)));
        assert_that!(&record.additions, has_entry("FXA".into(), (36, 38)));
    }

    #[test]
    fn test_example_2() {
        let record = JRecord::parse("J010812HDT").unwrap();
        assert_eq!(record.additions.len(), 1);
        assert_that!(&record.additions, has_entry("HDT".into(), (8, 12)));
    }

    #[test]
    fn test_errors() {
        assert_eq!(format!("{}", JRecord::parse("J010000ÄÖÜ").unwrap_err()),
                   "Expected: ASCII characters; Found: J010000ÄÖÜ");

        assert_eq!(format!("{}", JRecord::parse("J").unwrap_err()),
                   "Expected: at least 3 characters; Found: J");

        assert_eq!(format!("{}", JRecord::parse("JAB").unwrap_err()),
                   "Expected: digits; Found: AB");

        assert_eq!(format!("{}", JRecord::parse("J01000").unwrap_err()),
                   "Expected: 1 additions (= 7 characters); Found: 3 characters");

        assert_eq!(format!("{}", JRecord::parse("J-1").unwrap_err()),
                   "Expected: digits; Found: -1");

        assert_eq!(format!("{}", JRecord::parse("J010a02ABC").unwrap_err()),
                   "Expected: digits; Found: 0a");

        assert_eq!(format!("{}", JRecord::parse("J0100-1ABC").unwrap_err()),
                   "Expected: digits; Found: -1");
    }

    proptest! {
        #[test]
        #[allow(unused_must_use)]
        fn doesnt_crash(s in r"J\PC*") {
            JRecord::parse(&s);
        }

        #[test]
        fn parses_all_valid_times(additions in "([0-9]{4}[A-Z]{3})+") {
            let record = format!("J{:02}{}", additions.len() / 7, additions);
            prop_assert!(JRecord::parse(&record).is_ok());
        }
    }
}
