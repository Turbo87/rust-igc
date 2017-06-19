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

    let mut records = Vec::new();

    for line in buf_reader.lines() {
        let record = parse_line(line.unwrap().as_ref()).unwrap();
        records.push(record);
    }

    assert_eq!(records.len(), 13533);
}
