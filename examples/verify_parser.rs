extern crate igc;

use std::env;
use std::fs;
use std::fs::File;
use std::io::BufReader;

use igc::Record;
use igc::utils::lines::ByteLinesExt;

fn main() {
    // collect command line arguments
    let args: Vec<_> = env::args().collect();

    // iterate over files in the folder
    for entry in fs::read_dir(&args[1]).unwrap() {
        let path: std::path::PathBuf = entry.unwrap().path();

        // open file in buffered reader
        let file = File::open(path.clone()).unwrap();
        let reader = BufReader::new(file);

        let errors: Vec<_> = reader.byte_lines()
            .map(Result::unwrap)
            .enumerate()
            .filter(|(_, bytes)| !bytes.is_empty())
            .map(|(line, bytes)| (line, Record::parse(&bytes)))
            .filter(|(_, res)| res.is_err())
            .map(|(line, res)| (line, res.unwrap_err()))
            .collect();

        if !errors.is_empty() {
            // get filename for printing failure message
            let filename = path.file_name().unwrap().to_str().unwrap();
            println!();
            println!("{}", filename);
            println!("--------------------------------");

            errors.iter().for_each(|(line, error)| {
                println!("line {}: {}", line + 1, error);
            });
        };
    }
}
