use std::borrow::{BorrowMut, Cow};
use std::fs::File;
use std::io::{Write};

use crate::database::Table;

mod database;

macro_rules! table_title_format {
    ($lang:expr,$args:expr) => {{
        match $lang {
            "GO" => format!("type struct {} {{",to_camel_case($args,true)),
            "JAVA" => format!("public class {} {{",to_camel_case($args,true)),
            "RUST" => format!("pub struct {} {{",to_camel_case($args,true)),
            _ => format!("pub struct {} {{",$args),
        }
    }}
}

macro_rules! table_filename_format {
    ($lang:expr,$args:expr) => {{
        match $lang {
            "GO" => format!("{}.go",to_camel_case($args,true)),
            "JAVA" => format!("{}.java",to_camel_case($args,true)),
            "RUST" => format!("{}.rs",to_lower_case($args)),
            _ => format!("{}.java",to_camel_case($args,true)),
        }
    }}
}


macro_rules! table_row_format {
    ($lang:expr,$args0:expr,$args1:expr) => {{
        match $lang {
            "GO" => format!("{} {}",to_camel_case($args1,true),convert_type($lang,$args1)),
            "JAVA" => format!("{} {};",convert_type($lang,$args0),to_camel_case($args1,false)),
            "RUST" => format!("{}:{},",to_lower_case($args0),convert_type($lang,$args1)),
            _ => format!("{}:{},",$args0,$args1),
        }
    }}
}

macro_rules! table_end_format {
    ($lang:expr) => {{
        "}"
    }}
}

fn to_camel_case(input: String, start_with_upper: bool) -> String {
    let arrays: Vec<&str> = input.split("_").collect();
    let mut result = String::with_capacity(input.len());
    for (index, value) in arrays.iter().enumerate() {
        if index == 0 && !start_with_upper {
            result.push_str(value.to_lowercase().as_str())
        } else {
            result.push_str(upper_first_char(value.to_lowercase()).trim())
        }
    }
    result
}

fn upper_first_char<'a>(input: String) -> Cow<'a, str> {
    let mut ret = String::new();
    for (index, ch) in input.chars().enumerate() {
        if index == 0 {
            ret.push(ch.to_ascii_uppercase());
        } else {
            ret.push(ch.to_ascii_lowercase());
        }
    }
    Cow::Owned(ret)
}

fn to_lower_case(input: String) -> String {
    input.to_lowercase()
}

fn convert_type<'a>(lang: &str, input: String) -> String{
    if input.starts_with("varchar") || input.starts_with("char") {
        let ret = match lang {
            "GO" => "string",
            "RUST" => "String",
            "JAVA" => "String",
            _ => "String"
        };
        return ret.to_string();
    } else if input.starts_with("timestamp") {
       let ret = match lang {
            "GO" => "time.Date",
            "RUST" => "time",
            "JAVA" => "DateTime",
            _ => "string"
        };
        return ret.to_string();
    } else if input.starts_with("int") {
        let ret = match lang {
            "GO" => "int",
            "RUST" => "int",
            "JAVA" => "Int",
            _ => "int"
        };
        return ret.to_string();
    } else if input.starts_with("decimal") {
        let ret = match lang {
            "GO" => "float64",
            "RUST" => "f64",
            "JAVA" => "BigDecimal",
            _ => "string"
        };
        return ret.to_string();
    } else {
        return input.clone();
    }
}


fn main() {
    let lang_in = std::env::args().nth(1).expect("Missing lang").to_uppercase();
    let lang = lang_in.trim();

    let out_path = std::env::args().nth(2).expect("Missing out file path");

    let db_url = std::env::args().nth(3).expect("Missing DB URL root:password@host:port");

    let db_dbs = std::env::args().nth(4).expect("Missing DB SCHEMA AA|BB|CC");


    let mut tables: Vec<Table> = vec!();

    for db in db_dbs.split("|") {
        let mut ret = database::gen_model_from_database(format!("mysql://{}/{}", db_url, db).as_str());
        tables.append(ret.borrow_mut());
    }

    for table in tables {
        let tname = table.name.get(2..).unwrap().to_string();//remove the table name prefix，eg:T_XXX_XXX

        let table_title = table_title_format!(lang, tname.clone());
        let filepath = table_filename_format!(lang, tname);

        let mut out_file = File::create(format!("{}/{}", out_path, filepath)).expect("create output file failed");

        out_file.write_all(new_line(table_title));

        for field in table.fields {
            let table_row = table_row_format!(lang, field.fname, field.ftype.clone());
            out_file.write_all(new_line(table_row));
        }
        out_file.write_all(new_line(table_end_format!(lang).to_string()));
        let _ = out_file.flush();
    }

    fn new_line<'a>(text: String) -> &'a [u8] {
        format!("{}\n", text).as_bytes()
    }
}


