use cgmath::Deg;

use super::numbers::*;

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Point {
    pub longitude: Deg<f64>,
    pub latitude: Deg<f64>,
}

impl Point {
    pub fn new(longitude: Deg<f64>, latitude: Deg<f64>) -> Point {
        Point { longitude, latitude }
    }
}

named!(lat_invert <bool>, alt!(
    tag!("N") => { |_| false } |
    tag!("S") => { |_| true }
));

named!(latitude <Deg<f64>>, do_parse!(
    deg: below_90 >>
    min: below_60 >>
    min_dec: below_1000 >>
    invert: lat_invert >>
    (dms_to_deg(deg as f64, min as f64, min_dec as f64, invert))
));

named!(lng_invert <bool>, alt!(
    tag!("E") => { |_| false } |
    tag!("W") => { |_| true }
));

named!(longitude <Deg<f64>>, do_parse!(
    deg: below_180 >>
    min: below_60 >>
    min_dec: below_1000 >>
    invert: lng_invert >>
    (dms_to_deg(deg as f64, min as f64, min_dec as f64, invert))
));

fn dms_to_deg(deg: f64, min: f64, min_dec: f64, invert: bool) -> Deg<f64> {
    let value = Deg(deg + min / 60. + min_dec / 60000.);
    if invert { -value } else { value }
}

named!(pub coordinate <Point>, do_parse!(
    lat: latitude >>
    lng: longitude >>
    (Point::new(lng, lat))
));

#[cfg(test)]
mod tests {
    use cgmath::Deg;
    use super::{latitude, longitude, coordinate};

    #[test]
    fn test_latitude() {
        assert!(latitude(b"0").is_incomplete());
        assert!(latitude(b"-1").is_err());
        assert_relative_eq!(latitude(b"0000000N").unwrap().1, Deg(0.));
        assert_relative_eq!(latitude(b"0000000S").unwrap().1, Deg(0.));
        assert_relative_eq!(latitude(b"5016925N").unwrap().1, Deg(50.28208333333333));
        assert_relative_eq!(latitude(b"5016925S").unwrap().1, Deg(-50.28208333333333));
        assert_relative_eq!(latitude(b"8959999N").unwrap().1, Deg(89.99998333333333));
        assert_eq!(latitude(b"8959999N-").unwrap().0, &b"-"[..]);
        assert!(latitude(b"9000000N").is_err());
    }

    #[test]
    fn test_longitude() {
        assert!(longitude(b"0").is_incomplete());
        assert!(longitude(b"-1").is_err());
        assert_relative_eq!(longitude(b"00000000E").unwrap().1, Deg(0.));
        assert_relative_eq!(longitude(b"00000000W").unwrap().1, Deg(0.));
        assert_relative_eq!(longitude(b"05016925E").unwrap().1, Deg(50.28208333333333));
        assert_relative_eq!(longitude(b"05016925W").unwrap().1, Deg(-50.28208333333333));
        assert_relative_eq!(longitude(b"00953112E").unwrap().1, Deg(9.8852));
        assert_relative_eq!(longitude(b"17959999E").unwrap().1, Deg(179.99998333333333));
        assert_eq!(longitude(b"17959999E-").unwrap().0, &b"-"[..]);
        assert!(longitude(b"18000000E").is_err());
    }

    #[test]
    fn test_coordinate() {
        let result = coordinate(b"5016925N00953112E");
        assert!(result.is_done());
        let point = result.unwrap().1;
        assert_relative_eq!(point.longitude, Deg(9.8852));
        assert_relative_eq!(point.latitude, Deg(50.28208333333333));
    }
}
