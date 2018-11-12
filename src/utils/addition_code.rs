use std::str::{FromStr, from_utf8_unchecked};

#[derive(Debug, PartialEq, Eq, Hash, Clone, EnumString)]
pub enum AdditionCode {
    /// Linear accelerations in X axes (longitudinal), for aerobatic aircraft
    /// equipped with appropriate sensors feeding to the recorder and IGC file. (so-called "G")
    ACX,
    /// Linear accelerations in Y axes (lateral), for aerobatic aircraft
    /// equipped with appropriate sensors feeding to the recorder and IGC file. (so-called "G")
    ACY,
    /// Linear accelerations in Z axes (vertical), for aerobatic aircraft
    /// equipped with appropriate sensors feeding to the recorder and IGC file. (so-called "G")
    ACZ,

    /// Angular accelerations in X axes (pitch), for aerobatic aircraft
    /// equipped with appropriate sensors feeding to the recorder and IGC file.
    ANX,
    /// Angular accelerations in Y axes (roll), for aerobatic aircraft
    /// equipped with appropriate sensors feeding to the recorder and IGC file.
    ANY,
    /// Angular accelerations in Z axes (yaw), for aerobatic aircraft
    /// equipped with appropriate sensors feeding to the recorder and IGC file.
    ABZ,

    /// Displacement east, metres. For West use negative sign
    DAE,
    /// Displacement north, metres. For South use negative sign
    DAN,

    /// Environmental Noise Level. The ENL system is inside the FR and is
    /// intended to record when an engine is running in three numbers between
    /// 000 and 999 in the fix records of the IGC file.
    ENL,

    /// Fix accuracy. When used in the B (fix) record, this is the EPE
    /// (Estimated Position Error) figure in metres (MMM) for the individual
    /// fix concerned, to a 2-Sigma (95.45%) probability
    FXA,

    /// Heading Magnetic, three numbers based on degrees clockwise from 000 for north
    HDM,
    /// Heading True, three numbers based on degrees clockwise from 000 for north
    HDT,

    /// Airspeed, three numbers in kilometres per hour
    IAS,

    /// The last places of decimal minutes of latitude, where latitude is
    /// recorded to a greater precision than the three decimal minutes that are
    /// in the main body of the B-record. The fourth and any further decimal
    /// places of minutes are recorded as an addition to the B-record, their
    /// position in each B-record line being specified in the I-record.
    LAD,
    /// The last places of decimal minutes of longitude, where longitude is
    /// recorded to a greater precision than the three decimal minutes that are
    /// in the main body of the B-record. The fourth and any further decimal
    /// places of minutes are recorded as an addition to the B-record, their
    /// position in each B-record line being specified in the I-record.
    LOD,

    /// Means of Propulsion. A signal from an engine-related function from a
    /// sensor connected by cable to the FR and placed close to the engine
    /// and/or propeller, giving three numbers between 000 and 999 in the
    /// fix records of the IGC file.
    MOP,

    /// Outside air temperature (Celsius). If negative, use negative sign
    /// before the numbers.
    OAT,

    /// RAIM - GPS Parameter, see Glossary
    RAI,

    /// Record addition - Manufacturer defined data defined in the I or J
    /// record as appropriate, normally in the form of a TLC (which, if a new
    /// variable is agreed, may be a new TLC allocated by GFAC at the time).
    /// Any use must be approved by GFAC, and published so that there will be
    /// no doubt on how it is being used.
    REX,

    /// Satellites in use. A two-character field from the NMEA GGA or GNS
    /// sentences, as appropriate, or equivalent data agreed by GFAC.
    SIU,

    /// Airspeed True, give units (kt, kph, etc.)
    TAS,

    /// Decimal seconds of UTC time, for use with systems recording time to
    /// this accuracy. Time in seconds is recorded in the main body of the
    /// B-record and decimal seconds are recorded as an addition to the
    /// B-record, their position in each B-record line being specified in
    /// the I-record. Similarly with the K and J-records.
    TDS,

    /// Total Energy Altitude in metres
    TEN,

    /// Uncompensated variometer (non-total energy) vertical speed in metres
    /// per second and tenths of metres per second with leading zero and no
    /// dot (".") separator between metres and tenths. Valid characters 0-9
    /// and negative sign "-". Negative values to have negative sign instead
    /// of leading zero
    VAR,
    /// Compensated variometer (total energy/NETTO) vertical speed in metres
    /// per second and tenths of metres per second with leading zero and no
    /// dot (".") separator between metres and tenths. Valid characters 0-9
    /// and negative sign "-". Negative values to have negative sign instead
    /// of leading zero
    VAT,

    /// Vertical Fix Accuracy, Three characters in metres from the VDOP part
    /// of the NMEA GSA sentence, or equivalent data agreed by GFAC.
    VXA,

    /// Wind Direction (the direction the wind is coming from). Three numbers
    /// based on degrees clockwise from 000 for north
    WDI,
    /// Wind speed, three numbers in kilometres per hour
    WSP,

    #[strum(default="true")]
    Other(String),
}

impl AdditionCode {
    pub unsafe fn from_bytes_unchecked(bytes: &[u8]) -> AdditionCode {
        debug_assert!(bytes.is_ascii());
        let code = from_utf8_unchecked(bytes);
        AdditionCode::from_str(code).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use super::*;

    #[test]
    fn test_from_str() {
        assert_eq!(AdditionCode::from_str("FXA").unwrap(), AdditionCode::FXA);
        assert_eq!(AdditionCode::from_str("SIU").unwrap(), AdditionCode::SIU);
        assert_eq!(AdditionCode::from_str("siu").unwrap(), AdditionCode::Other("siu".into()));
        assert_eq!(AdditionCode::from_str("???").unwrap(), AdditionCode::Other("???".into()));
        assert_eq!(AdditionCode::from_str("").unwrap(), AdditionCode::Other("".into()));
        assert_eq!(AdditionCode::from_str("foobar").unwrap(), AdditionCode::Other("foobar".into()));
    }

    #[test]
    fn test_from_bytes() {
        assert_eq!(unsafe { AdditionCode::from_bytes_unchecked(b"FXA") }, AdditionCode::FXA);
        assert_eq!(unsafe { AdditionCode::from_bytes_unchecked(b"SIU") }, AdditionCode::SIU);
        assert_eq!(unsafe { AdditionCode::from_bytes_unchecked(b"siu") }, AdditionCode::Other("siu".into()));
        assert_eq!(unsafe { AdditionCode::from_bytes_unchecked(b"???") }, AdditionCode::Other("???".into()));
        assert_eq!(unsafe { AdditionCode::from_bytes_unchecked(b"") }, AdditionCode::Other("".into()));
        assert_eq!(unsafe { AdditionCode::from_bytes_unchecked(b"foobar") }, AdditionCode::Other("foobar".into()));
    }

    proptest! {
        #[test]
        #[allow(unused_must_use)]
        fn from_str_doesnt_crash(s in r"\PC*") {
            prop_assert!(AdditionCode::from_str(&s).is_ok());
        }
    }
}
