use std::str::{FromStr, from_utf8_unchecked};

#[derive(Debug, PartialEq, Eq, Hash, Clone, EnumString)]
pub enum EventCode {
    /// Altimeter pressure setting in hectoPascals (the same as Millibars) as
    /// a 6 digit number PPPPpp including 2 decimal places. For instance,
    /// ICAO ISA Sea Level (1013.25 mb) has an PPPPpp code of 101325, and
    /// 980.75 mb has a code of 098075.
    ATS,

    /// Blind Flying Instrument. Recorded as ON or OFF in the format `BFION`
    /// or `BFIOFF`, followed by a space and then AH (Artificial Horizon) for
    /// an instrument displaying the horizon, or TI (Turn Indicator) for one
    /// giving rate of turn, change of heading, or similar. If the ON/OFF
    /// status is uncertain, use the format BFIUN (for Status Unknown). A
    /// Text String (optional) may follow to give more detail of the
    /// instrument and its status. The initial state shall be reported in an
    /// E record at the time of the first B record in the IGC file with the
    /// Fix Validity byte set to A (3D Fix).
    BFI,

    /// Camera Connect
    CCN,
    /// Camera Disconnect
    CDC,

    /// Change of geodetic datum
    CGD,

    /// Engine on.
    ///
    /// Note: In some legacy recorders where ENL (now mandatory) and MOP
    /// (where required) are not used, the EON/EOF or EUP/EDN codes were used
    /// instead. EON/EOF was based on functions such as ignition ON/OFF,
    /// generator output, etc. EUP/EDN was used for a microswitch sensor
    /// for engine bay doors open/closed or pylon up/down.
    EON,
    /// Engine off. See note on line for EON
    EOF,
    /// Engine down. See note on line for EON
    EDN,
    /// Engine up. See note on line for EON
    EUP,

    /// Finish
    FIN,

    /// Flap position, three characters such as FLP060 for 60 degrees of
    /// positive flap. If negative, use a negative sign before the numbers,
    /// such as FLP-20 for minus 20 degrees flap.
    FLP,

    /// GNSS (Separate module) Connect
    GCN,
    /// GNSS (Separate module) Disconnect
    GDC,

    /// Low voltage. Must be set for each FR at the lowest voltage at which
    /// the FR will operate without the possibility of recorded data being
    /// degraded by the voltage level. Not to be used to invalidate a flight
    /// if the flight data appears correct when checked in the normal way,
    /// but a warning to check fix data particularly carefully.
    LOV,

    /// MacCready setting for rate of climb/speed-to-fly (m/sec)
    MAC,

    /// On Task – attempting task
    ONT,

    /// Pilot EVent - Pilot initiated action such as pressing a button. A
    /// sequence of fast fixes follows.
    PEV,

    /// Photo taken (shutter-press)
    PHO,

    /// Start event
    STA,

    /// Turn point confirmation - Equipment generated event (not valid for
    /// flight validation which requires independent checking of fixes and
    /// relevant Observation Zones)
    TPC,

    /// Undercarriage (landing gear), recorded as UP or DN, in the format
    /// `UNDUP` or `UNDDN`.
    UND,

    #[strum(default="true")]
    Other(String),
}

impl EventCode {
    pub unsafe fn from_bytes_unchecked(bytes: &[u8]) -> EventCode {
        debug_assert!(bytes.is_ascii());
        let code = from_utf8_unchecked(bytes);
        EventCode::from_str(code).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use super::*;

    #[test]
    fn test_from_str() {
        assert_eq!(EventCode::from_str("PEV").unwrap(), EventCode::PEV);
        assert_eq!(EventCode::from_str("MAC").unwrap(), EventCode::MAC);
        assert_eq!(EventCode::from_str("pev").unwrap(), EventCode::Other("pev".into()));
        assert_eq!(EventCode::from_str("???").unwrap(), EventCode::Other("???".into()));
        assert_eq!(EventCode::from_str("").unwrap(), EventCode::Other("".into()));
        assert_eq!(EventCode::from_str("foobar").unwrap(), EventCode::Other("foobar".into()));
    }

    #[test]
    fn test_from_bytes() {
        assert_eq!(unsafe { EventCode::from_bytes_unchecked(b"PEV") }, EventCode::PEV);
        assert_eq!(unsafe { EventCode::from_bytes_unchecked(b"MAC") }, EventCode::MAC);
        assert_eq!(unsafe { EventCode::from_bytes_unchecked(b"pev") }, EventCode::Other("pev".into()));
        assert_eq!(unsafe { EventCode::from_bytes_unchecked(b"???") }, EventCode::Other("???".into()));
        assert_eq!(unsafe { EventCode::from_bytes_unchecked(b"") }, EventCode::Other("".into()));
        assert_eq!(unsafe { EventCode::from_bytes_unchecked(b"foobar") }, EventCode::Other("foobar".into()));
    }

    proptest! {
        #[test]
        #[allow(unused_must_use)]
        fn from_str_doesnt_crash(s in r"\PC*") {
            prop_assert!(EventCode::from_str(&s).is_ok());
        }
    }
}
