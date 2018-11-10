use std::collections::HashMap;

use ::{Result, ParseError};

pub type AdditionsMap = HashMap<String, String>;
pub type AdditionsDeclMap = HashMap<String, (u8, u8)>;

pub fn parse_from_record_line(input: &str) -> Result<AdditionsDeclMap> {
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

    let mut additions = AdditionsDeclMap::with_capacity(number_of_additions);

    for i in 0..number_of_additions {
        let start_byte = &input[(3 + i * 7)..(3 + i * 7 + 2)];
        let start_byte = start_byte.parse()
            .map_err(|_| ParseError::unexpected("digits", start_byte))?;

        let end_byte = &input[(3 + i * 7 + 2)..(3 + i * 7 + 4)];
        let end_byte = end_byte.parse()
            .map_err(|_| ParseError::unexpected("digits", end_byte))?;

        if end_byte < start_byte {
            return Err(ParseError::unexpected(
                "start byte <= end byte",
                format!("start={} end={}", start_byte, end_byte),
            ));
        }

        let code = &input[(3 + i * 7 + 4)..(3 + i * 7 + 7)];

        additions.insert(code.into(), (start_byte, end_byte));
    }

    Ok(additions)
}

pub fn parse_additions(input: &str, additions_decl: &AdditionsDeclMap) -> Result<AdditionsMap> {
    debug_assert!(input.is_ascii());

    let input_length = input.len();

    let mut additions = AdditionsMap::with_capacity(additions_decl.len());

    for (code, &(start_byte, end_byte)) in additions_decl {
        if input_length < end_byte as usize {
            return Err(ParseError::unexpected(
                format!("at least {} characters", end_byte),
                format!("{} characters", input_length)));
        }

        let value = &input[(start_byte as usize - 1)..(end_byte as usize)];

        additions.insert(code.clone(), value.into());
    }

    Ok(additions)
}

#[cfg(test)]
mod tests {
    use galvanic_assert::matchers::collection::*;

    use super::*;

    #[test]
    fn test_example_1() {
        let additions_decl = parse_from_record_line("J010812HDT").unwrap();
        let additions = parse_additions("K12345600090", &additions_decl).unwrap();
        assert_eq!(additions.len(), 1);
        assert_that!(&additions, has_entry("HDT".into(), "00090".into()));
    }

    #[test]
    fn test_example_2() {
        let additions_decl = parse_from_record_line("I023638FXA3940SIU").unwrap();
        let additions = parse_additions("B0818265049456N00610940EA011730132000308", &additions_decl).unwrap();
        assert_eq!(additions.len(), 2);
        assert_that!(&additions, has_entry("FXA".into(), "003".into()));
        assert_that!(&additions, has_entry("SIU".into(), "08".into()));
    }

    #[test]
    fn test_errors() {
        let additions_decl = parse_from_record_line("J010812HDT").unwrap();
        let result = parse_additions("K1234560009", &additions_decl);
        assert_eq!(format!("{}", result.unwrap_err()),
                   "Expected: at least 12 characters; Found: 11 characters");
    }
}
