#[derive(Debug, Eq, PartialEq)]
pub struct Time {
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
}

impl Time {
    pub fn from_hms(hour: u8, minute: u8, second: u8) -> Time {
        Time { hour,  minute, second }
    }
}
