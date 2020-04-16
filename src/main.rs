use std::io::{BufReader, BufRead, Write};
use std::fs::File;
use regex::Regex;

fn main() {

    let inPath = std::env::args().nth(1).expect("Missing input file path");

    let outPath = std::env::args().nth(2).expect("Missing output file path");

    let file = File::open(inPath).unwrap();

    let buffered: BufReader<File> = BufReader::new(file);

    let mut outFile = File::create(outPath).expect("create output file failed");

    let titleRegex:Regex = Regex::new(r"CREATE\s+TABLE\s+\S+.(?P<title>\S+$)").unwrap();
    let colsRegex:Regex = Regex::new(r"^\S{4}(\S+)\s+(\S+),?$").unwrap();
    let endRegex:Regex = Regex::new(r"\)\s+CHARSET\s+=\s+UTF8MB4;").unwrap();

    for line in buffered.lines().map(|x| x.unwrap()) {

        println!("{}", line);

        if titleRegex.is_match(line.as_str()){
            println!("match title:{}",line);
            let title = &(titleRegex.captures(&line).unwrap())["title"];
            outFile.write_all(title.as_bytes());
            outFile.write_all(b"\n");
        }

        if colsRegex.is_match(line.as_str()){
            println!("match colum:{}",line);
            //outFile.write_all( titleRegex.captures_iter(&line)[0].at(0).unwrap_or(""))
        }

        if endRegex.is_match(line.as_str()){
            println!("match end:{}",line);
            let end = endRegex.captures(&line).unwrap().get(0).unwrap().as_str();
            outFile.write_all(end.as_bytes());
            outFile.write_all(b"\n");
        }


    }
}

