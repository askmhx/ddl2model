use mysql::{Pool, Value};
use mysql::prelude::{Queryable};

pub struct Table {
    pub name: String,
    pub fields: Vec<Field>,
}

pub struct Field {
    pub fname: String,
    pub ftype: String,
    pub fcollection: Value,
    pub fnull: Value,
    pub fkey: Value,
    pub fdefault: Value,
    pub fextra: Value,
    pub fprivilages: Value,
    pub fcomment: Value,
}



pub fn gen_model_from_database(db_url: &str) -> Vec<Table> {
    let mut tables: Vec<Table> = vec!();

    let pool = Pool::new(db_url).unwrap();

    let mut conn = pool.get_conn().unwrap();

    let result: Vec<String> = conn.query("SHOW TABLES").unwrap();

    for table_name in result {
        let mut table = Table { name: table_name.clone(), fields: vec!() };

        let mut table_conn = pool.get_conn().unwrap();


        let mut ret_fields: Vec<Field> = table_conn.query_map(format!("SHOW FULL COLUMNS FROM {};", table_name), |(fname, ftype, fcollection, fnull, fkey, fdefault, fextra, fprivilages, fcomment)| {
            println!("{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?}", fname, ftype, fcollection, fnull, fkey, fdefault, fextra, fprivilages, fcomment);

            Field { fname: fname, ftype: ftype, fcollection: fcollection, fnull: fnull, fkey: fkey, fdefault: fdefault, fextra: fextra, fprivilages: fprivilages, fcomment: fcomment }
        }).unwrap();

        table.fields.append(&mut ret_fields);

        tables.push(table);
    }

    return tables;
}