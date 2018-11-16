use regex::bytes::Regex;

use ::{Error, Result};
use ::utils::headers::{HeaderSource, HeaderCode};
use ::utils::text::as_text;

#[derive(Debug, PartialEq, Eq)]
pub struct HRecord {
    pub source: HeaderSource,
    pub code: HeaderCode,
    pub text: String,
}

impl HRecord {
    pub fn new<T: Into<String>>(source: HeaderSource, code: HeaderCode, text: T) -> HRecord {
        HRecord { source, code, text: text.into() }
    }

    pub fn parse(line: &[u8]) -> Result<HRecord> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(?x-u)
                ^H
                ([FOP])                # data source
                ([A-Z\d]{3})           # record subtype
                (?:[^:]+:|:)?          # optional record subtype long name and colon separator
                (.*)                   # text
            ").unwrap();
        }

        let cap = RE.captures(line).ok_or_else(|| Error::invalid_record(line))?;

        let source = HeaderSource::from_byte_unchecked(cap[1][0]);
        let code = unsafe { HeaderCode::from_bytes_unchecked(&cap[2]) };

        let text = as_text(&cap[3])
            .ok_or_else(||Error::invalid_record(line))?;

        Ok(HRecord { source, code, text })
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use super::HeaderSource::*;
    use super::HeaderCode::*;

    #[test]
    fn test_parse_separators() {
        assert_eq!(HRecord::parse(b"HFRFWFlarm-IGC04.07").unwrap(),
                   HRecord::new(FlightRecorder, RFW, "Flarm-IGC04.07"));
        assert_eq!(HRecord::parse(b"HFRFW:Flarm-IGC04.07").unwrap(),
                   HRecord::new(FlightRecorder, RFW, "Flarm-IGC04.07"));
        assert_eq!(HRecord::parse(b"HFRFWFirmwareVersion:Flarm-IGC04.07").unwrap(),
                   HRecord::new(FlightRecorder, RFW, "Flarm-IGC04.07"));
    }

    #[test]
    fn test_parse_sources() {
        assert_eq!(HRecord::parse(b"HFGTYGliderType:A350").unwrap(),
                   HRecord::new(FlightRecorder, GTY, "A350"));
        assert_eq!(HRecord::parse(b"HOGTYGliderType:A350").unwrap(),
                   HRecord::new(Observer, GTY, "A350"));
        assert_eq!(HRecord::parse(b"HPGTYGliderType:A350").unwrap(),
                   HRecord::new(HeaderSource::Other('P'), GTY, "A350"));
    }

    #[test]
    fn test_parse_encodings() {
        assert_eq!(HRecord::parse(b"HFPLT:John Doe").unwrap(),
                   HRecord::new(FlightRecorder, PLT, "John Doe"));
        assert_eq!(HRecord::parse("HFPLT:Jörg Müller".as_bytes()).unwrap(),
                   HRecord::new(FlightRecorder, PLT, "Jörg Müller"));
        assert_eq!(HRecord::parse(b"HFPLT:J\xf6rg M\xfcller").unwrap(),
                   HRecord::new(FlightRecorder, PLT, "Jörg Müller"));
    }

    #[test]
    fn test_parse_real_world() {
        assert_eq!(HRecord::parse(b"HFDTE150510").unwrap(),
                   HRecord::new(FlightRecorder, DTE, "150510"));
        assert_eq!(HRecord::parse(b"HFFXA500").unwrap(),
                   HRecord::new(FlightRecorder, "FXA".parse().unwrap(), "500"));
        assert_eq!(HRecord::parse(b"HFPLTPilotincharge:").unwrap(),
                   HRecord::new(FlightRecorder, PLT, ""));
        assert_eq!(HRecord::parse(b"HPCM2Crew2:").unwrap(),
                   HRecord::new(HeaderSource::Other('P'), CM2, ""));
        assert_eq!(HRecord::parse(b"HFGTYGliderType:").unwrap(),
                   HRecord::new(FlightRecorder, GTY, ""));
        assert_eq!(HRecord::parse(b"HFGIDGliderID:").unwrap(),
                   HRecord::new(FlightRecorder, GID, ""));
        assert_eq!(HRecord::parse(b"HFDTM100GPSDatum:WGS84").unwrap(),
                   HRecord::new(FlightRecorder, DTM, "WGS84"));
        assert_eq!(HRecord::parse(b"HFRFWFirmwareVersion:Flarm-IGC04.07").unwrap(),
                   HRecord::new(FlightRecorder, RFW, "Flarm-IGC04.07"));
        assert_eq!(HRecord::parse(b"HFRHWHardwareVersion:Flarm-IGC06").unwrap(),
                   HRecord::new(FlightRecorder, RHW, "Flarm-IGC06"));
        assert_eq!(HRecord::parse(b"HFFTYFRType:Flarm-IGC").unwrap(),
                   HRecord::new(FlightRecorder, FTY, "Flarm-IGC"));
        assert_eq!(HRecord::parse(b"HFGPSu-blox:LEA-4P,16,8191").unwrap(),
                   HRecord::new(FlightRecorder, GPS, "LEA-4P,16,8191"));
        assert_eq!(HRecord::parse(b"HFPRSPressAltSensor:Intersema MS5534B,8191").unwrap(),
                   HRecord::new(FlightRecorder, PRS, "Intersema MS5534B,8191"));
        assert_eq!(HRecord::parse(b"HFCCLCompetitionClass:").unwrap(),
                   HRecord::new(FlightRecorder, CCL, ""));
        assert_eq!(HRecord::parse(b"HFCIDCompetitionID:").unwrap(),
                   HRecord::new(FlightRecorder, CID, ""));
        assert_eq!(HRecord::parse(b"HFDTE150709").unwrap(),
                   HRecord::new(FlightRecorder, DTE, "150709"));
        assert_eq!(HRecord::parse(b"HFFXA100").unwrap(),
                   HRecord::new(FlightRecorder, "FXA".parse().unwrap(), "100"));
        assert_eq!(HRecord::parse(b"HFPLTPILOT:KEVIN.HOULIHAN").unwrap(),
                   HRecord::new(FlightRecorder, PLT, "KEVIN.HOULIHAN"));
        assert_eq!(HRecord::parse(b"HFGTYGLIDERTYPE:DG800/18").unwrap(),
                   HRecord::new(FlightRecorder, GTY, "DG800/18"));
        assert_eq!(HRecord::parse(b"HFGIDGLIDERID:EI-GMN").unwrap(),
                   HRecord::new(FlightRecorder, GID, "EI-GMN"));
        assert_eq!(HRecord::parse(b"HFDTM100GPSDATUM:WGS-1984").unwrap(),
                   HRecord::new(FlightRecorder, DTM, "WGS-1984"));
        assert_eq!(HRecord::parse(b"HFRFWFIRMWAREVERSION:1.0").unwrap(),
                   HRecord::new(FlightRecorder, RFW, "1.0"));
        assert_eq!(HRecord::parse(b"HFRHWHARDWAREVERSION:2.1").unwrap(),
                   HRecord::new(FlightRecorder, RHW, "2.1"));
        assert_eq!(HRecord::parse(b"HFFTYFRTYPE:LXNAVIGATION,LX7007F").unwrap(),
                   HRecord::new(FlightRecorder, FTY, "LXNAVIGATION,LX7007F"));
        assert_eq!(HRecord::parse(b"HFGPS:uBLOXf_TIM-LP,16,max9000m").unwrap(),
                   HRecord::new(FlightRecorder, GPS, "uBLOXf_TIM-LP,16,max9000m"));
        assert_eq!(HRecord::parse(b"HFPRSPRESSALTSENSOR:INTERSEMA,MS5534A,max8000m").unwrap(),
                   HRecord::new(FlightRecorder, PRS, "INTERSEMA,MS5534A,max8000m"));
        assert_eq!(HRecord::parse(b"HFCIDCOMPETITIONID:MN").unwrap(),
                   HRecord::new(FlightRecorder, CID, "MN"));
        assert_eq!(HRecord::parse(b"HFCCLCOMPETITIONCLASS:STANDARD").unwrap(),
                   HRecord::new(FlightRecorder, CCL, "STANDARD"));
        assert_eq!(HRecord::parse(b"HFDTE040516").unwrap(),
                   HRecord::new(FlightRecorder, DTE, "040516"));
        assert_eq!(HRecord::parse(b"HFFXA500").unwrap(),
                   HRecord::new(FlightRecorder, "FXA".parse().unwrap(), "500"));
        assert_eq!(HRecord::parse(b"HFPLTPilotincharge:").unwrap(),
                   HRecord::new(FlightRecorder, PLT, ""));
        assert_eq!(HRecord::parse(b"HPCM2Crew2:").unwrap(),
                   HRecord::new(HeaderSource::Other('P'), CM2, ""));
        assert_eq!(HRecord::parse(b"HFGTYGliderType:ASG-29E (18m)").unwrap(),
                   HRecord::new(FlightRecorder, GTY, "ASG-29E (18m)"));
        assert_eq!(HRecord::parse(b"HFGIDGliderID:D-KCSS").unwrap(),
                   HRecord::new(FlightRecorder, GID, "D-KCSS"));
        assert_eq!(HRecord::parse(b"HFDTM100GPSDatum:WGS84").unwrap(),
                   HRecord::new(FlightRecorder, DTM, "WGS84"));
        assert_eq!(HRecord::parse(b"HFRFWFirmwareVersion:Flarm-IGC06.01").unwrap(),
                   HRecord::new(FlightRecorder, RFW, "Flarm-IGC06.01"));
        assert_eq!(HRecord::parse(b"HFRHWHardwareVersion:Flarm-IGC06").unwrap(),
                   HRecord::new(FlightRecorder, RHW, "Flarm-IGC06"));
        assert_eq!(HRecord::parse(b"HFFTYFRType:Flarm-IGC").unwrap(),
                   HRecord::new(FlightRecorder, FTY, "Flarm-IGC"));
        assert_eq!(HRecord::parse(b"HFGPSu-blox:LEA-4P,16,8191").unwrap(),
                   HRecord::new(FlightRecorder, GPS, "LEA-4P,16,8191"));
        assert_eq!(HRecord::parse(b"HFPRSPressAltSensor:Intersema MS5534B,8191").unwrap(),
                   HRecord::new(FlightRecorder, PRS, "Intersema MS5534B,8191"));
        assert_eq!(HRecord::parse(b"HFCCLCompetitionClass:Club").unwrap(),
                   HRecord::new(FlightRecorder, CCL, "Club"));
        assert_eq!(HRecord::parse(b"HFCIDCompetitionID:TH").unwrap(),
                   HRecord::new(FlightRecorder, CID, "TH"));
    }

    proptest! {
        #[test]
        #[allow(unused_must_use)]
        fn parse_doesnt_crash(s in r"\PC*") {
            HRecord::parse(s.as_bytes());
        }
    }
}
