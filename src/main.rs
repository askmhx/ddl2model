
mod ddl;

use std::io::{BufReader, BufRead, Write};
use std::fs::File;
use regex::Regex;
use std::collections::HashMap;

const CONST_LANG_GO: &str = "GO";
const CONST_LANG_JAVA: &str = "JAVA";
const CONST_LANG_RUST: &str = "RUST";

const CONST_TEMPLATE_STAT: HashMap<&str, &str> = HashMap::new();
const CONST_TEMPLATE_END: HashMap<&str, &str> = HashMap::new();
const CONST_TEMPLATE_ROW: HashMap<&str, &str> = HashMap::new();

const CONST_REGEX_START: Regex = Regex::new(r"CREATE\s+TABLE\s+\S+.(?P<title>\S+$)").unwrap();
const CONST_REGEX_END: Regex = Regex::new(r"\)\s+CHARSET\s+=\s+UTF8MB4(\s+)?;(\s+)?").unwrap();
const CONST_REGEX_ROW: Regex = Regex::new(r"^\S{4}(\S+)\s+(\S+),?$").unwrap();


fn main() {

    CONST_TEMPLATE_STAT.insert(CONST_LANG_GO, "");
    CONST_TEMPLATE_STAT.insert(CONST_LANG_JAVA, "");
    CONST_TEMPLATE_STAT.insert(CONST_LANG_RUST, "");

    CONST_TEMPLATE_ROW.insert(CONST_LANG_GO, "");
    CONST_TEMPLATE_ROW.insert(CONST_LANG_JAVA, "");
    CONST_TEMPLATE_ROW.insert(CONST_LANG_RUST, "");


    CONST_TEMPLATE_END.insert(CONST_LANG_GO, "");
    CONST_TEMPLATE_END.insert(CONST_LANG_JAVA, "");
    CONST_TEMPLATE_END.insert(CONST_LANG_RUST, "");


    let lang = std::env::args().nth(1).expect("Missing input file path").to_uppercase();

    let in_path = std::env::args().nth(2).expect("Missing input file path");

    let out_path = std::env::args().nth(3).expect("Missing output file path");

    let in_file = File::open(in_path).unwrap();

    let mut out_file = File::create(out_path).expect("create output file failed");

    ddl::gen_model(lang.as_str(), in_file, out_file)
}


