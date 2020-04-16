use std::io::{BufReader, BufRead, Write};
use std::fs::File;
use regex::Regex;

fn main() {
    let filepath = std::env::args().nth(1).expect("Missing configuration file path");
    let file = std::fs::File::open(filepath).unwrap();
    let buffered: BufReader<File> = BufReader::new(file);

    let mut outFile = std::fs::File::create("./out.txt").expect("create failed");

    let titleRegex:Regex = Regex::new(r"CREATE\s+TABLE\s+\S+.(\S+)").unwrap();
    let colsRegex:Regex = Regex::new(r"^\S{4}(\S+)\s+(\S+)\,?$").unwrap();
    let endRegex:Regex = Regex::new(r"\)\s+CHARSET\s+=\s+UTF8MB4;").unwrap();

    for line in buffered.lines().map(|x| x.unwrap()) {
        println!("{}", line);

        if titleRegex.is_match(line.as_str()){
            println!("--------{}-------",line)
            // outFile.write_all( titleRegex.captures_iter(&line)[0].at(0).unwrap_or(""))
        }

    }
}
