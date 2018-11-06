extern crate igc;

use std::env;
use std::fs;

use igc::Reader;

fn main() {
    // collect command line arguments
    let args: Vec<_> = env::args().collect();

    // iterate over files in the folder
    for entry in fs::read_dir(&args[1]).unwrap() {
        let path: std::path::PathBuf = entry.unwrap().path();

        // open file in buffered reader
        let mut reader = Reader::from_path(path.clone()).unwrap();

        // parse file into results vector
        let results = reader.records().collect::<Vec<_>>();

        // check if the results contain errors
        if !results.iter().all(|result| result.is_ok()) {
            // get filename for printing failure message
            let filename = path.file_name().unwrap().to_str().unwrap();

            // print failure message
            println!("{}: parsing failed\n{:?}", filename,
                     results.iter().find(|result| result.is_err()).unwrap());
        };
    }
}
