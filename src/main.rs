use std::borrow::{BorrowMut, Cow};
use std::fs::{File, OpenOptions};
use std::io::{Write};

use crate::database::Table;

mod database;

macro_rules! table_filename_format {
    ($lang:expr,$args:expr) => {{
        match $lang {
            "GO" => format!("{}.go","database"),
            "PROTOBUF" => format!("{}.proto","protobuf"),
            "JAVA" => format!("{}.java",to_camel_case($args,true)),
            "RUST" => format!("{}.rs","database"),
            _ => format!("{}.java",to_camel_case($args,true)),
        }
    }}
}


macro_rules! table_title_format {
    ($lang:expr,$args:expr) => {{
        match $lang {
            "GO" => format!("type {} struct {{",to_camel_case($args,true)),
            "PROTOBUF" => format!("message {} {{",to_camel_case($args,true)),
            "JAVA" => format!("public class {} {{",to_camel_case($args,true)),
            "RUST" => format!("pub struct {} {{",to_camel_case($args,true)),
            _ => format!("pub struct {} {{",$args),
        }
    }}
}

macro_rules! table_row_format {
    ($lang:expr,$aname:expr,$atype:expr,$idex:expr) => {{
        match $lang {
            "GO" => format!("    {} {}",to_camel_case($aname,true),convert_type($lang,$atype)),
            "PROTOBUF" => format!("    {} {} = {};",convert_type($lang,$atype),to_camel_case($aname,false),$idex),
            "JAVA" => format!("    private {} {};",convert_type($lang,$atype),to_camel_case($aname,false)),
            "RUST" => format!("    pub {}:{},",to_lower_case($aname),convert_type($lang,$atype)),
            _ => format!("    pub {}:{},",$aname,$atype),
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

fn convert_type<'a>(lang: &str, input: String) -> String {
    if input.starts_with("varchar") || input.starts_with("char") {
        let ret = match lang {
            "GO" => "string",
            "PROTOBUF" => "string",
            "RUST" => "String",
            "JAVA" => "String",
            _ => "String"
        };
        return ret.to_string();
    } else if input.starts_with("time") || input.starts_with("date") {
        let ret = match lang {
            "GO" => "time.Time",
            "PROTOBUF" => "string",
            "RUST" => "time",
            "JAVA" => "Date",
            _ => "string"
        };
        return ret.to_string();
    } else if input.starts_with("int") || input.starts_with("bigint") {
        let ret = match lang {
            "GO" => "int",
            "PROTOBUF" => "int32",
            "RUST" => "int",
            "JAVA" => "Integer",
            _ => "int"
        };
        return ret.to_string();
    } else if input.starts_with("decimal") {
        let ret = match lang {
            "GO" => "float64",
            "PROTOBUF" => "double",
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


    let out_path = std::env::args().nth(1).expect("Missing out file path");

    let db_url = std::env::args().nth(2).expect("Missing DB URL root:password@host:port");

    let db_dbs = std::env::args().nth(3).expect("Missing DB SCHEMA AA|BB|CC");

    let lang_in = std::env::args().nth(4).expect("Missing lang").to_uppercase();
    let lang = lang_in.trim();


    let mut tables: Vec<Table> = vec!();

    for db in db_dbs.split("#") {
        let mut ret = database::gen_model_from_database(format!("mysql://{}/{}", db_url, db).as_str());
        tables.append(ret.borrow_mut());
    }

    for table in tables {

        let tname = table.name.get(2..).unwrap().to_string();//remove the table name prefix，eg:T_XXX_XXX

        let mut out_file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .append(true)
            .open(format!("{}/{}", out_path, table_filename_format!(lang, tname.clone()))).unwrap();

        let _ = out_file.write_all(new_line(table_title_format!(lang, tname)).as_bytes());

        for (index, field) in table.fields.iter().enumerate() {
            let table_row = table_row_format!(lang, field.fname.clone(), field.ftype.clone(),index+1);
            let _ = out_file.write_all(new_line(table_row).as_bytes());
        }
        let _ = out_file.write_all(new_line(table_end_format!(lang).to_string()).as_bytes());
    }

    fn new_line(text: String) -> String {
        format!("{}\n", text)
    }
}


