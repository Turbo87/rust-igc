use ::utils::additions::AdditionCode;
use ::utils::num::parse_int;

pub trait AdditionSupport {
    fn get_addition(&self, code: &AdditionCode) -> Option<&[u8]>;

    fn get_three_digit_addition(&self, code: &AdditionCode) -> Option<u16> {
        let bytes = self.get_addition(code)?;
        if bytes.len() != 3 { return None }
        parse_int::<u16>(bytes)
    }

    /// Fix accuracy in metres
    fn fix_accuracy(&self) -> Option<u16> {
        self.get_three_digit_addition(&AdditionCode::FXA)
    }

    /// Environmental Noise Level
    fn enl(&self) -> Option<u16> {
        self.get_three_digit_addition(&AdditionCode::ENL)
    }

    /// Heading True
    fn heading(&self) -> Option<u16> {
        let value = self.get_three_digit_addition(&AdditionCode::HDT)?;
        if value < 360 { Some(value) } else { None }
    }

    /// Heading Magnetic
    fn heading_magnetic(&self) -> Option<u16> {
        let value = self.get_three_digit_addition(&AdditionCode::HDM)?;
        if value < 360 { Some(value) } else { None }
    }
}
