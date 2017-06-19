use std::io::BufRead;
use std::iter::Iterator;

use super::Record;
use super::super::ParseError;

pub struct RecordsIter<R> {
    pub reader: R,
}

impl<R: BufRead> Iterator for RecordsIter<R> {
    type Item = Result<Record, ParseError>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let mut buf = Vec::new();

        match self.reader.read_until(b'\n', &mut buf) {
            Ok(0) => None,
            Ok(_n) => {
                if buf.ends_with(b"\n") {
                    buf.pop();
                    if buf.ends_with(b"\r") {
                        buf.pop();
                    }
                }
                Some(Record::parse(&buf))
            }
            Err(e) => Some(Err(ParseError::IoError(e)))
        }
    }
}
