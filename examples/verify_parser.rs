extern crate igc;

use std::env;
use std::fs;
use std::io::BufReader;

use igc::parse;

fn main() {
    // collect command line arguments
    let args: Vec<_> = env::args().collect();

    // iterate over files in the folder
    for entry in fs::read_dir(&args[1]).unwrap() {
        let path: std::path::PathBuf = entry.unwrap().path();

        // open file in buffered reader
        let file = fs::File::open(path.clone()).unwrap();
        let buf_reader = BufReader::new(file);

        // parse file into results vector
        let results = parse(buf_reader).collect::<Vec<_>>();

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
