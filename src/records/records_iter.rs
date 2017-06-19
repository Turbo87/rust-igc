use std::io::{BufRead, Lines};
use std::iter::Iterator;

use super::Record;
use super::super::{parse_line, ParseError};

pub struct RecordsIter<R> {
    pub iter: Lines<R>,
}

impl<R: BufRead> Iterator for RecordsIter<R> {
    type Item = Result<Record, ParseError>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|line| parse_line(&line?))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}
