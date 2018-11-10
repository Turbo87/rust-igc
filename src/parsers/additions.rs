use std::collections::HashMap;

use ::{Result, ParseError};

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
