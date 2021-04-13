mod database;

use std::io::{BufReader, BufRead, Write, Read};
use std::fs::File;
use regex::Regex;
use std::collections::HashMap;
use std::borrow::{Borrow, BorrowMut};
use crate::database::Table;

const CONST_LANG_GO: &str = "GO";
const CONST_LANG_JAVA: &str = "JAVA";
const CONST_LANG_RUST: &str = "RUST";

const CONST_TEMPLATE_STAT: HashMap<&str, &str> = HashMap::new();
const CONST_TEMPLATE_END: HashMap<&str, &str> = HashMap::new();
const CONST_TEMPLATE_ROW: HashMap<&str, &str> = HashMap::new();


fn main() {
    CONST_TEMPLATE_STAT.insert(CONST_LANG_GO, "type struct {} {");
    CONST_TEMPLATE_STAT.insert(CONST_LANG_JAVA, "public class {} {");
    CONST_TEMPLATE_STAT.insert(CONST_LANG_RUST, "pub struct {} {");

    CONST_TEMPLATE_ROW.insert(CONST_LANG_GO, "{} {}");
    CONST_TEMPLATE_ROW.insert(CONST_LANG_JAVA, "{} {};");
    CONST_TEMPLATE_ROW.insert(CONST_LANG_RUST, "{}:{},");


    CONST_TEMPLATE_END.insert(CONST_LANG_GO, "}");
    CONST_TEMPLATE_END.insert(CONST_LANG_JAVA, "}");
    CONST_TEMPLATE_END.insert(CONST_LANG_RUST, "}");

    let lang = std::env::args().nth(1).expect("lang").to_uppercase();

    let out_path = std::env::args().nth(2).expect("Missing out file path");

    let db_url = std::env::args().nth(3).expect("Missing DB URL root:password@host:port");

    let db_dbs = std::env::args().nth(4).expect("Missing dB SCHEMA AA|BB|CC");

    let mut out_file = File::create(out_path).expect("create output file failed");

    let mut tables: Vec<Table> = Vec::new();

    for db in db_dbs.split("|") {
        let mut ret = database::gen_model_from_database(format!("mysql://{}/{}", db_url, db).as_str());
        tables.append(ret.borrow_mut());
    }

    for table in tables {
        let table_title = format!(CONST_TEMPLATE_STAT.get(lang.borrow()), table.name);

        out_file.write(table_title.as_bytes());

        for field in table.fields {
            let table_row = format!(CONST_TEMPLATE_ROW.get(lang.borrow()), field.fname, field.ftype);
            out_file.write(table_row.as_bytes());
        }
        out_file.write(CONST_TEMPLATE_END.get(lang.borrow()).as_bytes());
    }
}


