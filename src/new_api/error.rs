use std::fmt;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "invalid record: {}", line)]
    InvalidRecord { line: Line },
}

impl Error {
    pub fn invalid_record(line: &[u8]) -> Error {
        Error::InvalidRecord { line: line.into() }
    }
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct Line(Vec<u8>);

impl <'a> From<&'a [u8]> for Line {
    fn from(bytes: &'a [u8]) -> Self {
        Line(bytes.to_vec())
    }
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::result::Result<(), fmt::Error> {
        let bytes = self.0.as_ref();

        match std::str::from_utf8(bytes) {
            Ok(s) => f.write_str(s),
            Err(_) => write!(f, "{:?}", bytes),
        }
    }
}
