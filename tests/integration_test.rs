extern crate igc;

use std::path::Path;
use std::fs::File;
use std::io::BufReader;

use igc::{parse, Record};

#[test]
fn it_works() {
    let path = Path::new(file!())
        .parent().unwrap()
        .join("fixtures")
        .join("654g6ng1.igc");

    let file = File::open(path).unwrap();
    let buf_reader = BufReader::new(file);

    let records: Vec<Record> = parse(buf_reader)
        .filter_map(Result::ok)
        .collect();

    assert_eq!(records.len(), 13533);
}
