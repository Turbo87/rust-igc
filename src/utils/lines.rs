use std::io::{Read, BufRead, BufReader, Cursor, Result};

pub trait ByteLinesExt: BufRead {
    fn byte_lines(self) -> ByteLines<Self> where Self: Sized {
        ByteLines { buf: self }
    }
}

impl<R: Read> ByteLinesExt for BufReader<R> {}
impl<T> ByteLinesExt for Cursor<T> where T: AsRef<[u8]> {}

#[derive(Debug)]
pub struct ByteLines<B> {
    buf: B,
}

impl<B: BufRead> Iterator for ByteLines<B> {
    type Item = Result<Vec<u8>>;

    fn next(&mut self) -> Option<Result<Vec<u8>>> {
        let mut buf = Vec::new();
        match self.buf.read_until(b'\n', &mut buf) {
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
            Err(e) => Some(Err(e))
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn byte_lines() {
        use std::io::Cursor;

        let buf = Cursor::new(b"12\r");
        let mut s = buf.byte_lines();
        assert_eq!(s.next().unwrap().unwrap(), b"12\r");
        assert!(s.next().is_none());

        let buf = Cursor::new(b"12\r\n\n");
        let mut s = buf.byte_lines();
        assert_eq!(s.next().unwrap().unwrap(), b"12");
        assert_eq!(s.next().unwrap().unwrap(), b"");
        assert!(s.next().is_none());

        let buf = Cursor::new(b"abc\ndef");
        let mut s = buf.byte_lines();
        assert_eq!(s.next().unwrap().unwrap(), b"abc");
        assert_eq!(s.next().unwrap().unwrap(), b"def");
        assert!(s.next().is_none());

        let buf = Cursor::new(b"abc\r\ndef");
        let mut s = buf.byte_lines();
        assert_eq!(s.next().unwrap().unwrap(), b"abc");
        assert_eq!(s.next().unwrap().unwrap(), b"def");
        assert!(s.next().is_none());
    }
}
