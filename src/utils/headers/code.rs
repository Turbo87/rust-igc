use std::str::{FromStr, from_utf8_unchecked};

#[derive(Debug, PartialEq, Eq, Hash, Clone, EnumString)]
pub enum HeaderCode {
    /// Date, expressed as DDMMYY
    DTE,

    /// Pilot-in-charge (aircraft commander),
    /// family name first then given name(s) as required
    PLT,
    /// Second Crew Member's Name, family name first then given name(s) as
    /// required (same format as PLT for pilot-in-charge). For aircraft with
    /// more than two crew, use CM3 and so forth if required.
    CM2,

    /// Obsolete code, now use DB1. Was Date of Birth of the pilot in the
    /// previous line of the H record (DDMMYY)
    DOB,
    /// Date of Birth of the pilot-in-charge (aircraft commander) in th
    /// previous line of the H record (DDMMYY)
    DB1,
    /// Date of Birth of second crew member in format DDMMYY. For aircraft
    /// with more than two crew, use DB3, DB4 etc.
    DB2,

    /// Glider type, manufacturer, model
    GTY,
    /// Glider ID (registration)
    GID,
    /// Competition ID
    CID,
    /// Competition class
    CCL,

    /// Geodetic Datum in use for lat/long records (for IGC purposes this must
    /// be set to WGS84)
    DTM,

    /// FR Type (Manufacturer's name, FR Model Number)
    FTY,
    /// Firmware Revision Version of FR
    RFW,
    /// Hardware Revision Version of FR
    RHW,
    /// GPS (US GNS System), followed by receiver maker, type & version
    /// letter/number.
    GPS,
    /// Pressure Altitude Sensor, manufacturer, model, etc (in the H record
    /// line this is followed by the maximum altitude processed by the FR)
    PRS,
    /// Flight Recorder Security. To be used where a security fault has been
    /// detected such as the recorder internal security system (microswitch)
    /// having operated.
    FRS,

    /// Means of Propulsion.
    /// (Maker, ON/OFF, Acoustic / ECurrent / Other, Sensor Model)
    MOP,

    #[strum(default="true")]
    Other(String),
}

impl HeaderCode {
    pub unsafe fn from_bytes_unchecked(bytes: &[u8]) -> HeaderCode {
        debug_assert!(bytes.is_ascii());
        let code = from_utf8_unchecked(bytes);
        HeaderCode::from_str(code).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use super::*;

    #[test]
    fn test_from_str() {
        assert_eq!(HeaderCode::from_str("PLT").unwrap(), HeaderCode::PLT);
        assert_eq!(HeaderCode::from_str("CM2").unwrap(), HeaderCode::CM2);
        assert_eq!(HeaderCode::from_str("cm2").unwrap(), HeaderCode::Other("cm2".into()));
        assert_eq!(HeaderCode::from_str("???").unwrap(), HeaderCode::Other("???".into()));
        assert_eq!(HeaderCode::from_str("").unwrap(), HeaderCode::Other("".into()));
        assert_eq!(HeaderCode::from_str("foobar").unwrap(), HeaderCode::Other("foobar".into()));
    }

    #[test]
    fn test_from_bytes() {
        assert_eq!(unsafe { HeaderCode::from_bytes_unchecked(b"PLT") }, HeaderCode::PLT);
        assert_eq!(unsafe { HeaderCode::from_bytes_unchecked(b"CM2") }, HeaderCode::CM2);
        assert_eq!(unsafe { HeaderCode::from_bytes_unchecked(b"cm2") }, HeaderCode::Other("cm2".into()));
        assert_eq!(unsafe { HeaderCode::from_bytes_unchecked(b"???") }, HeaderCode::Other("???".into()));
        assert_eq!(unsafe { HeaderCode::from_bytes_unchecked(b"") }, HeaderCode::Other("".into()));
        assert_eq!(unsafe { HeaderCode::from_bytes_unchecked(b"foobar") }, HeaderCode::Other("foobar".into()));
    }

    proptest! {
        #[test]
        #[allow(unused_must_use)]
        fn from_str_doesnt_crash(s in r"\PC*") {
            prop_assert!(HeaderCode::from_str(&s).is_ok());
        }
    }
}
