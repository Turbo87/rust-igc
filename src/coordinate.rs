use super::ParseError;

fn parse_latitude_invert(input: u8) -> Option<bool> {
    match input {
        b'N' => Some(false),
        b'S' => Some(true),
        _ => None,
    }
}

pub fn parse_latitude(input: &[u8]) -> Result<f64, ParseError> {
    debug_assert_eq!(input.len(), 8);

    let str = String::from_utf8(input.to_vec())?;

    let deg = str[0..2].parse::<u32>().map_err(|_| ParseError::InvalidLatitude(str.to_string()))?;
    if deg >= 90 {
        return Err(ParseError::InvalidLatitude(str.to_string()));
    }

    let min = str[2..4].parse::<u32>().map_err(|_| ParseError::InvalidLatitude(str.to_string()))?;
    if min >= 60 {
        return Err(ParseError::InvalidLatitude(str.to_string()));
    }

    let min_dec = str[4..7].parse::<u32>().map_err(|_| ParseError::InvalidLatitude(str.to_string()))?;

    let invert = parse_latitude_invert(input[7]).ok_or_else(|| ParseError::InvalidLatitude(str.to_string()))?;

    Ok(dms_to_deg(deg as f64, min as f64, min_dec as f64, invert))
}

fn parse_longitude_invert(input: u8) -> Option<bool> {
    match input {
        b'E' => Some(false),
        b'W' => Some(true),
        _ => None,
    }
}

pub fn parse_longitude(input: &[u8]) -> Result<f64, ParseError> {
    debug_assert_eq!(input.len(), 9);

    let str = String::from_utf8(input.to_vec())?;

    let deg = str[0..3].parse::<u32>().map_err(|_| ParseError::InvalidLongitude(str.to_string()))?;
    if deg >= 180 {
        return Err(ParseError::InvalidLongitude(str.to_string()));
    }

    let min = str[3..5].parse::<u32>().map_err(|_| ParseError::InvalidLongitude(str.to_string()))?;
    if min >= 60 {
        return Err(ParseError::InvalidLongitude(str.to_string()));
    }

    let min_dec = str[5..8].parse::<u32>().map_err(|_| ParseError::InvalidLongitude(str.to_string()))?;

    let invert = parse_longitude_invert(input[8]).ok_or_else(|| ParseError::InvalidLongitude(str.to_string()))?;

    Ok(dms_to_deg(deg as f64, min as f64, min_dec as f64, invert))
}

fn dms_to_deg(deg: f64, min: f64, min_dec: f64, invert: bool) -> f64 {
    let value = deg + min / 60. + min_dec / 60000.;
    if invert { -value } else { value }
}

#[cfg(test)]
mod tests {
    use super::{parse_latitude, parse_longitude};

    #[test]
    fn test_latitude() {
        assert_relative_eq!(parse_latitude(b"0000000N").unwrap(), 0.);
        assert_relative_eq!(parse_latitude(b"0000000S").unwrap(), 0.);
        assert_relative_eq!(parse_latitude(b"5016925N").unwrap(), 50.28208333333333);
        assert_relative_eq!(parse_latitude(b"5016925S").unwrap(), -50.28208333333333);
        assert_relative_eq!(parse_latitude(b"8959999N").unwrap(), 89.99998333333333);
        assert!(parse_latitude(b"9000000N").is_err());
        assert!(parse_latitude(b"0060000N").is_err());
        assert!(parse_latitude(b"0000000X").is_err());
        assert!(parse_latitude(b"00000x0N").is_err());
    }

    #[test]
    fn test_longitude() {
        assert_relative_eq!(parse_longitude(b"00000000E").unwrap(), 0.);
        assert_relative_eq!(parse_longitude(b"00000000W").unwrap(), 0.);
        assert_relative_eq!(parse_longitude(b"05016925E").unwrap(), 50.28208333333333);
        assert_relative_eq!(parse_longitude(b"05016925W").unwrap(), -50.28208333333333);
        assert_relative_eq!(parse_longitude(b"00953112E").unwrap(), 9.8852);
        assert_relative_eq!(parse_longitude(b"17959999E").unwrap(), 179.99998333333333);
        assert!(parse_longitude(b"18000000E").is_err());
        assert!(parse_longitude(b"00060000E").is_err());
        assert!(parse_longitude(b"00000000X").is_err());
        assert!(parse_longitude(b"00000x00E").is_err());
    }
}
