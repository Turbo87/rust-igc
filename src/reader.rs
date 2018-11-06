use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use error::{Result, ParseError};
use records::*;

#[derive(Debug)]
pub struct Reader<R> {
    /// The underlying reader.
    reader: io::BufReader<R>,
}

impl Reader<Reader<File>> {
    /// Create a new IGC parser for the given file path.
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Reader<File>> {
        Ok(Reader::new(File::open(path)?))
    }
}

impl<R: io::Read> Reader<R> {
    fn new(rdr: R) -> Reader<R> {
        Reader {
            reader: io::BufReader::new(rdr),
        }
    }

    /// Create a new IGC parser for the given reader.
    pub fn from_reader(rdr: R) -> Reader<R> {
        Reader::new(rdr)
    }

    pub fn records(&mut self) -> RecordsIter<R> {
        RecordsIter::new(self)
    }

    fn next_record(&mut self) -> Option<Result<Record>> {
        self.read_line()
            .map(|result| result
                .and_then(|line| self.parse_line(&line)))
    }

    fn read_line(&mut self) -> Option<Result<Vec<u8>>> {
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
                Some(Ok(buf))
            }
            Err(e) => Some(Err(e.into()))
        }
    }

    fn parse_line(&mut self, bytes: &[u8]) -> Result<Record> {
        if bytes.is_empty() {
            return Ok(Record::Empty);
        }

        match bytes[0] {
            b'A' => Ok(Record::A),
            b'B' => BRecord::parse(bytes).map(Record::B),
            b'C' => Ok(Record::C),
            b'D' => Ok(Record::D),
            b'E' => Ok(Record::E),
            b'F' => Ok(Record::F),
            b'G' => Ok(Record::G),
            b'H' => Ok(Record::H),
            b'I' => Ok(Record::I),
            b'J' => Ok(Record::J),
            b'K' => Ok(Record::K),
            b'L' => Ok(Record::L),
            _ => Err(ParseError::UnknownRecordType(bytes[0])),
        }
    }
}

pub struct RecordsIter<'r, R: 'r> {
    reader: &'r mut Reader<R>,
}

impl<'r, R: io::Read> RecordsIter<'r, R> {
    fn new(rdr: &'r mut Reader<R>) -> RecordsIter<'r, R> {
        RecordsIter { reader: rdr }
    }

    /// Return a reference to the underlying IGC reader.
    pub fn reader(&self) -> &Reader<R> {
        &self.reader
    }

    /// Return a mutable reference to the underlying IGC reader.
    pub fn reader_mut(&mut self) -> &mut Reader<R> {
        &mut self.reader
    }
}

impl<'r, R: io::Read> Iterator for RecordsIter<'r, R> {
    type Item = Result<Record>;

    fn next(&mut self) -> Option<Result<Record>> {
        self.reader.next_record()
    }
}