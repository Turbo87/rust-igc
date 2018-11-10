use ::Result;
use ::parsers::additions::{AdditionsDeclMap, parse_from_record_line};

// Examples:
//
// I023638FXA3941ENL
// I013638ENL

#[derive(Debug)]
pub struct IRecord {
    pub additions: AdditionsDeclMap,
}

impl IRecord {
    pub(crate) fn parse(input: &str) -> Result<Self> {
        debug_assert_eq!(&input[0..1], "I");

        parse_from_record_line(input)
            .map(|additions| IRecord { additions })
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

        assert_eq!(format!("{}", IRecord::parse("I010100ABC").unwrap_err()),
                   "Expected: start byte <= end byte; Found: start=1 end=0");
    }

    proptest! {
        #[test]
        #[allow(unused_must_use)]
        fn doesnt_crash(s in r"I\PC*") {
            IRecord::parse(&s);
        }

        #[test]
        fn parses_valid_records_1(additions in "([0-9]{2}99[A-Z]{3})+") {
            let record = format!("I{:02}{}", additions.len() / 7, additions);
            prop_assert!(IRecord::parse(&record).is_ok());
        }

        #[test]
        fn parses_valid_records_2(additions in "(00[0-9]{2}[A-Z]{3})+") {
            let record = format!("I{:02}{}", additions.len() / 7, additions);
            prop_assert!(IRecord::parse(&record).is_ok());
        }
    }
}