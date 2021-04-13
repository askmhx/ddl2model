use mysql::Pool;
use mysql::prelude::Queryable;
use std::borrow::Borrow;

pub struct Table {
    pub name: String,
    pub fields: Ver<Field>,
}

pub struct Field {
    pub fname: String,
    pub ftype: String,
    pub fcollection: String,
    pub fnull: String,
    pub fkey: String,
    pub fdefault: String,
    pub fextra: String,
    pub fprivilages: String,
    pub fcomment: String,
}

pub fn gen_model_from_database(db_url: &str) -> Vec<Table> {
    let tables: Vec<Table> = Vec::new();

    let pool = Pool::new(db_url).unwrap();

    let mut conn = pool.get_conn().unwrap();

    let result: Vec<String> = conn.query("SHOW TABLES").unwrap();

    for table_name in result {
        let mut table = Table();
        table.name = table_name;

        let mut fields: Vec<Field> = Vec::new();

        let sql_show_table = format!("SHOW FULL COLUMNS FROM {};", table_name);

        let mut table_conn = pool.get_conn().unwrap();
        table_conn.query_map(sql_show_table, |(field, ftype, fcollection, fnull, fkey, fdefault, fextra, fprivilages, fcomment)| {
            fields.push(Field(field, ftype, fcollection, fnull, fkey, fdefault, fextra, fprivilages, fcomment))
        }).unwrap();
        table.fields = fields;

        tables.borrow().push(table);
    }

    return tables;
}