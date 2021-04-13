use std::fs::File;
// use crate::{CONST_REGEX_ROW, CONST_REGEX_END, CONST_REGEX_START};
use mysql::Pool;
use mysql::prelude::Queryable;

pub struct Table {
     pub name :String,
     pub fields:Ver<Field>,
}

pub struct Field {
   pub fname:String,
   pub ftype:String,
   pub fcollection:String,
   pub fnull:String,
   pub fkey:String,
   pub fdefault:String,
   pub fextra:String,
   pub fprivilages:String,
   pub fcomment:String,
}

pub fn gen_model_from_database(db_url: &str) -> Vec<Table> {

    let tables:Vec<Table> = Vec::new();

    let pool = Pool::new(db_url).unwrap();

    let mut conn = pool.get_conn().unwrap();

    let result: Vec<String> = conn.query("show tables").unwrap();

    for table in result {
        let sql_show_table = format!("SHOW FULL COLUMNS FROM {};", table);
        println!("{}", table.as_str());
        let mut table_conn = pool.get_conn().unwrap();
        table_conn.query_map(sql_show_table, |(field, ftype, fcollection, fnull, fkey, fdefault, fextra, fprivilages, fcomment)| {
            println!("{},{},{},{},{},{},{},{},{}", field, ftype, fcollection, fnull, fkey, fdefault, fextra, fprivilages, fcomment);
        }).unwrap()
    }

    return tables
}