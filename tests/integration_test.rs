extern crate igc;

use std::path::Path;
use std::fs::File;
use std::io::BufReader;

use igc::Record;
use igc::utils::lines::ByteLinesExt;

#[test]
fn it_works() {
    let path = Path::new(file!())
        .parent().unwrap()
        .join("fixtures")
        .join("654g6ng1.igc");

    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut result_count = 0;
    let mut error_count = 0;
    let mut record_count = 0;
    let mut a_record_count = 0;
    let mut b_record_count = 0;
    let mut c_record_count = 0;
    let mut d_record_count = 0;
    let mut e_record_count = 0;
    let mut f_record_count = 0;
    let mut g_record_count = 0;
    let mut h_record_count = 0;
    let mut i_record_count = 0;
    let mut j_record_count = 0;
    let mut k_record_count = 0;
    let mut l_record_count = 0;

    for result in reader.byte_lines() {
        result_count += 1;

        let bytes = match result {
            Err(_) => {
                error_count += 1;
                continue;
            },
            Ok(bytes) => bytes,
        };

        if bytes.is_empty() { continue }

        match Record::parse(&bytes) {
            Err(_) => error_count += 1,
            Ok(record) => {
                record_count += 1;

                match record {
                    Record::A => a_record_count += 1,
                    Record::B(_) => b_record_count += 1,
                    Record::C => c_record_count += 1,
                    Record::D => d_record_count += 1,
                    Record::E => e_record_count += 1,
                    Record::F => f_record_count += 1,
                    Record::G => g_record_count += 1,
                    Record::H => h_record_count += 1,
                    Record::I(_) => i_record_count += 1,
                    Record::J => j_record_count += 1,
                    Record::K => k_record_count += 1,
                    Record::L => l_record_count += 1,
                }
            },
        }
    }

    assert_eq!(result_count, 13533);

    assert_eq!(error_count, 0);
    assert_eq!(record_count, 13533);

    assert_eq!(a_record_count, 1);
    assert_eq!(b_record_count, 9762);
    assert_eq!(c_record_count, 7);
    assert_eq!(d_record_count, 0);
    assert_eq!(e_record_count, 0);
    assert_eq!(f_record_count, 130);
    assert_eq!(g_record_count, 2);
    assert_eq!(h_record_count, 14);
    assert_eq!(i_record_count, 1);
    assert_eq!(j_record_count, 0);
    assert_eq!(k_record_count, 0);
    assert_eq!(l_record_count, 3616);
}
