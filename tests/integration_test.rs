extern crate igc;

use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufRead};

use igc::parse_line;

#[test]
fn it_works() {
    let path = Path::new(file!())
        .parent().unwrap()
        .join("fixtures")
        .join("654g6ng1.igc");

    let file = File::open(path).unwrap();
    let buf_reader = BufReader::new(file);

    for line in buf_reader.lines() {
        let mut s = line.unwrap();
        s.push_str("\r\n");
        parse_line(s.as_ref()).unwrap();
    }
}
