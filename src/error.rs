use std::error;
use std::fmt;
use std::num;
use std::string;

#[derive(Debug)]
pub enum ParseError {
    InvalidCharacters(string::FromUtf8Error),
    InvalidIntNumber(num::ParseIntError),
    LineTooShort,
    UnknownRecordType(u8),
    InvalidValidity(u8),
    InvalidTime(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ParseError::InvalidCharacters(ref err) => err.fmt(f),
            ParseError::InvalidIntNumber(ref err) => err.fmt(f),
            ParseError::LineTooShort => write!(f, "Line too short"),
            ParseError::UnknownRecordType(t) => write!(f, "Unknown record type: {}", t as char),
            ParseError::InvalidValidity(v) => write!(f, "Invalid validity: {}", v as char),
            ParseError::InvalidTime(ref str) => write!(f, "Invalid time: {}", str),
        }
    }
}

impl error::Error for ParseError {
    fn description(&self) -> &str {
        match *self {
            ParseError::InvalidCharacters(ref err) => err.description(),
            ParseError::InvalidIntNumber(ref err) => err.description(),
            ParseError::LineTooShort => "Line too short",
            ParseError::UnknownRecordType(..) => "Unknown record type",
            ParseError::InvalidValidity(..) => "Invalid validity",
            ParseError::InvalidTime(..) => "Invalid time",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            ParseError::InvalidCharacters(ref err) => Some(err),
            ParseError::InvalidIntNumber(ref err) => Some(err),
            _ => None,
        }
    }
}

impl From<string::FromUtf8Error> for ParseError {
    fn from(err: string::FromUtf8Error) -> ParseError {
        ParseError::InvalidCharacters(err)
    }
}

impl From<num::ParseIntError> for ParseError {
    fn from(err: num::ParseIntError) -> ParseError {
        ParseError::InvalidIntNumber(err)
    }
}
