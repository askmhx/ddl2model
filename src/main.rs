
mod ddl;

use std::fs::File;

fn main() {

    let lang = std::env::args().nth(1).expect("Missing input file path").to_uppercase();

    let in_path = std::env::args().nth(2).expect("Missing input file path");

    let out_path = std::env::args().nth(3).expect("Missing output file path");

    let in_file = File::open(in_path).unwrap();

    let out_file = File::create(out_path).expect("create output file failed");

    ddl::gen_model(lang.as_str(), in_file, out_file)
}


