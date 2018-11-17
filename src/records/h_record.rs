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

    fn assert_header(bytes: &[u8], code: HeaderCode, text: &str, source: HeaderSource) {
        assert_eq!(HRecord::parse(bytes).unwrap(), HRecord::new(source, code, text));
    }

    #[test]
    fn test_parse_separators() {
        assert_header(b"HFRFWFlarm-IGC04.07", RFW, "Flarm-IGC04.07", FlightRecorder);
        assert_header(b"HFRFW:Flarm-IGC04.07", RFW, "Flarm-IGC04.07", FlightRecorder);
        assert_header(b"HFRFWFirmwareVersion:Flarm-IGC04.07", RFW, "Flarm-IGC04.07", FlightRecorder);
    }

    #[test]
    fn test_parse_sources() {
        assert_header(b"HFGTYGliderType:A350", GTY, "A350", FlightRecorder);
        assert_header(b"HOGTYGliderType:A350", GTY, "A350", Observer);
        assert_header(b"HPGTYGliderType:A350", GTY, "A350", HeaderSource::Other('P'));
    }

    #[test]
    fn test_parse_encodings() {
        assert_header(b"HFPLT:John Doe", PLT, "John Doe", FlightRecorder);
        assert_header("HFPLT:Jörg Müller".as_bytes(), PLT, "Jörg Müller", FlightRecorder);
        assert_header(b"HFPLT:J\xf6rg M\xfcller", PLT, "Jörg Müller", FlightRecorder);
    }

    #[test]
    fn test_parse_real_world() {
        assert_header(b"HFDTE150510", DTE, "150510", FlightRecorder);
        assert_header(b"HFFXA500", "FXA".parse().unwrap(), "500", FlightRecorder);
        assert_header(b"HFPLTPilotincharge:", PLT, "", FlightRecorder);
        assert_header(b"HPCM2Crew2:", CM2, "", HeaderSource::Other('P'));
        assert_header(b"HFGTYGliderType:", GTY, "", FlightRecorder);
        assert_header(b"HFGIDGliderID:", GID, "", FlightRecorder);
        assert_header(b"HFDTM100GPSDatum:WGS84", DTM, "WGS84", FlightRecorder);
        assert_header(b"HFRFWFirmwareVersion:Flarm-IGC04.07", RFW, "Flarm-IGC04.07", FlightRecorder);
        assert_header(b"HFRHWHardwareVersion:Flarm-IGC06", RHW, "Flarm-IGC06", FlightRecorder);
        assert_header(b"HFFTYFRType:Flarm-IGC", FTY, "Flarm-IGC", FlightRecorder);
        assert_header(b"HFGPSu-blox:LEA-4P,16,8191", GPS, "LEA-4P,16,8191", FlightRecorder);
        assert_header(b"HFPRSPressAltSensor:Intersema MS5534B,8191", PRS, "Intersema MS5534B,8191", FlightRecorder);
        assert_header(b"HFCCLCompetitionClass:", CCL, "", FlightRecorder);
        assert_header(b"HFCIDCompetitionID:", CID, "", FlightRecorder);
        assert_header(b"HFDTE150709", DTE, "150709", FlightRecorder);
        assert_header(b"HFFXA100", "FXA".parse().unwrap(), "100", FlightRecorder);
        assert_header(b"HFPLTPILOT:KEVIN.HOULIHAN", PLT, "KEVIN.HOULIHAN", FlightRecorder);
        assert_header(b"HFGTYGLIDERTYPE:DG800/18", GTY, "DG800/18", FlightRecorder);
        assert_header(b"HFGIDGLIDERID:EI-GMN", GID, "EI-GMN", FlightRecorder);
        assert_header(b"HFDTM100GPSDATUM:WGS-1984", DTM, "WGS-1984", FlightRecorder);
        assert_header(b"HFRFWFIRMWAREVERSION:1.0", RFW, "1.0", FlightRecorder);
        assert_header(b"HFRHWHARDWAREVERSION:2.1", RHW, "2.1", FlightRecorder);
        assert_header(b"HFFTYFRTYPE:LXNAVIGATION,LX7007F", FTY, "LXNAVIGATION,LX7007F", FlightRecorder);
        assert_header(b"HFGPS:uBLOXf_TIM-LP,16,max9000m", GPS, "uBLOXf_TIM-LP,16,max9000m", FlightRecorder);
        assert_header(b"HFPRSPRESSALTSENSOR:INTERSEMA,MS5534A,max8000m", PRS, "INTERSEMA,MS5534A,max8000m", FlightRecorder);
        assert_header(b"HFCIDCOMPETITIONID:MN", CID, "MN", FlightRecorder);
        assert_header(b"HFCCLCOMPETITIONCLASS:STANDARD", CCL, "STANDARD", FlightRecorder);
        assert_header(b"HFDTE040516", DTE, "040516", FlightRecorder);
        assert_header(b"HFFXA500", "FXA".parse().unwrap(), "500", FlightRecorder);
        assert_header(b"HFPLTPilotincharge:", PLT, "", FlightRecorder);
        assert_header(b"HPCM2Crew2:", CM2, "", HeaderSource::Other('P'));
        assert_header(b"HFGTYGliderType:ASG-29E (18m)", GTY, "ASG-29E (18m)", FlightRecorder);
        assert_header(b"HFGIDGliderID:D-KCSS", GID, "D-KCSS", FlightRecorder);
        assert_header(b"HFDTM100GPSDatum:WGS84", DTM, "WGS84", FlightRecorder);
        assert_header(b"HFRFWFirmwareVersion:Flarm-IGC06.01", RFW, "Flarm-IGC06.01", FlightRecorder);
        assert_header(b"HFRHWHardwareVersion:Flarm-IGC06", RHW, "Flarm-IGC06", FlightRecorder);
        assert_header(b"HFFTYFRType:Flarm-IGC", FTY, "Flarm-IGC", FlightRecorder);
        assert_header(b"HFGPSu-blox:LEA-4P,16,8191", GPS, "LEA-4P,16,8191", FlightRecorder);
        assert_header(b"HFPRSPressAltSensor:Intersema MS5534B,8191", PRS, "Intersema MS5534B,8191", FlightRecorder);
        assert_header(b"HFCCLCompetitionClass:Club", CCL, "Club", FlightRecorder);
        assert_header(b"HFCIDCompetitionID:TH", CID, "TH", FlightRecorder);
    }

    proptest! {
        #[test]
        #[allow(unused_must_use)]
        fn parse_doesnt_crash(s in r"\PC*") {
            HRecord::parse(s.as_bytes());
        }
    }
}
