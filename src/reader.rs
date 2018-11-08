use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::result::{Result as StdResult};

use encoding::{Encoding, DecoderTrap};
use encoding::all::{ASCII, ISO_8859_1, UTF_8};

use error::{Result, ParseError};
use records::*;

pub struct Reader<R> {
    /// The underlying reader.
    reader: io::BufReader<R>,

    encoding: BufEncoding,
}

#[derive(Debug, Eq, PartialEq)]
enum BufEncoding {
    Auto,
    UTF8,
    Latin1,
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
            encoding: BufEncoding::Auto,
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
        loop {
            // 1. read until line break
            let bytes = match self.read_line() {
                None => return None,
                Some(Err(error)) => return Some(Err(ParseError::IoError(error))),
                Some(Ok(bytes)) => bytes,
            };

            // 2. if empty -> goto 1.
            if bytes.is_empty() {
                continue;
            }

            // 3. decode bytes to string
            let line = match self.decode_bytes(&bytes) {
                Err(error) => return Some(Err(ParseError::Encoding(error))),
                Ok(line) => line,
            };

            // 4. parse string
            return Some(self.parse_line(&line));
        }
    }

    fn read_line(&mut self) -> Option<StdResult<Vec<u8>, io::Error>> {
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

    fn decode_bytes(&mut self, bytes: &[u8]) -> StdResult<String, std::borrow::Cow<'static, str>> {
        return match self.encoding {
            BufEncoding::UTF8 => UTF_8.decode(bytes, DecoderTrap::Strict),
            BufEncoding::Latin1 => ISO_8859_1.decode(bytes, DecoderTrap::Strict),
            BufEncoding::Auto => {
                if let Ok(result) = ASCII.decode(bytes, DecoderTrap::Strict) {
                    return Ok(result);
                }

                if let Ok(result) = UTF_8.decode(bytes, DecoderTrap::Strict) {
                    self.encoding = BufEncoding::UTF8;
                    return Ok(result);
                }

                return match ISO_8859_1.decode(bytes, DecoderTrap::Strict) {
                    Ok(result) => {
                        self.encoding = BufEncoding::Latin1;
                        Ok(result)
                    },
                    Err(error) => Err(error)
                }
            },
        }
    }

    fn parse_line(&mut self, line: &str) -> Result<Record> {
        match line.as_bytes()[0] {
            b'A' => Ok(Record::A),
            b'B' => BRecord::parse(line).map(Record::B),
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
            _ => Err(ParseError::UnknownRecordType(line.chars().next().unwrap())),
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

#[cfg(test)]
mod tests {
    use super::Reader;

    #[test]
    fn test_ascii() {
        use std::io::Cursor;
        use super::BufEncoding;

        let input = Cursor::new(b"HFPLTPILOT:John Doe\n");
        let mut reader = Reader::from_reader(input);

        let _: Vec<_> = reader.records().collect();

        assert_eq!(reader.encoding, BufEncoding::Auto);
    }

    #[test]
    fn test_utf8() {
        use std::io::Cursor;
        use super::BufEncoding;

        let input = Cursor::new("HFPLTPILOT:Jörg Müller\n".as_bytes());
        let mut reader = Reader::from_reader(input);

        let _: Vec<_> = reader.records().collect();

        assert_eq!(reader.encoding, BufEncoding::UTF8);
    }

    #[test]
    fn test_latin1() {
        use std::io::Cursor;
        use super::BufEncoding;

        let input = Cursor::new(b"HFPLTPILOT:J\xf6rg M\xfcller\n");
        let mut reader = Reader::from_reader(input);

        let _: Vec<_> = reader.records().collect();

        assert_eq!(reader.encoding, BufEncoding::Latin1);
    }
}
