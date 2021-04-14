mod database;

use std::io::{BufReader, BufRead, Write, Read};
use std::fs::File;
use regex::Regex;
use std::collections::HashMap;
use std::borrow::{Borrow, BorrowMut};
use crate::database::Table;
use std::fmt;

macro_rules! table_title_format {
    ($lang:expr,$args:expr) => {{
        match $lang {
            "GO" => format!("type struct {} \x08",$args),
            "JAVA" => format!("public class {} \x08",$args),
            "RUST" => format!("pub struct {} \x08",$args),
            _ => format!("pub struct {} \x08",$args),
        }
    }}
}

macro_rules! table_row_format {
    ($lang:expr,$args0:expr,$args1:expr) => {{
        match $lang {
            "GO" => format!("{} {}",$args0,$args1),
            "JAVA" => format!("{} {};",$args0,$args1),
            "RUST" => format!("{}:{},",$args0,$args1),
            _ => format!("{}:{},",$args0,$args1),
        }
    }}
}

macro_rules! table_end_format {
    ($lang:expr) => {{
        "}"
    }}
}


fn main() {

    let lang = std::env::args().nth(1).expect("Missing lang").to_uppercase().as_str();

    let out_path = std::env::args().nth(2).expect("Missing out file path");

    let db_url = std::env::args().nth(3).expect("Missing DB URL root:password@host:port");

    let db_dbs = std::env::args().nth(4).expect("Missing DB SCHEMA AA|BB|CC");

    let mut out_file = File::create(out_path).expect("create output file failed");

    let mut tables: Vec<Table> = vec!();

    for db in db_dbs.split("|") {
        let mut ret = database::gen_model_from_database(format!("mysql://{}/{}", db_url, db).as_str());
        tables.append(ret.borrow_mut());
    }

    for table in tables {
        let table_title = table_title_format!(lang, table.name);

        out_file.write(table_title.as_bytes());

        for field in table.fields {
            let table_row = table_row_format!(lang, field.fname, field.ftype);
            out_file.write(table_row.as_bytes());
        }
        out_file.write(table_end_format!(lang).as_bytes());
    }
}


