use mysql::Pool;
use mysql::prelude::Queryable;
use std::io::Bytes;

pub struct Table {
    pub name: String,
    pub fields: Vec<Field>,
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
    let mut tables: Vec<Table> = vec!();

    let pool = Pool::new(db_url).unwrap();

    let mut conn = pool.get_conn().unwrap();

    let result: Vec<String> = conn.query("SHOW TABLES").unwrap();

    for table_name in result {
        let mut table = Table { name: table_name.clone(), fields: vec!() };

        let mut table_conn = pool.get_conn().unwrap();

        for row in table_conn.query_iter(format!("SHOW FULL COLUMNS FROM {};", table_name)).unwrap() {
            let rfname:String = row.clone().unwrap().get("Field").unwrap();
            let rftype:String = row.clone().unwrap().get("Type").unwrap();
            let rfcollection:String = row.clone().unwrap().get("Collation").unwrap();
            let rfnull:String = row.clone().unwrap().get("Null").unwrap();
            let rfkey:String = row.clone().unwrap().get("Key").unwrap();
            let rfdefault:String = row.clone().unwrap().get("Default").unwrap();
            let rfextra:String = row.clone().unwrap().get("Extra").unwrap();
            let rfprivilages:String = row.clone().unwrap().get("Privileges").unwrap();
            let rfcomment:String = row.clone().unwrap().get("Comment").unwrap();
            println!("{:?}", row.clone());
                println!("{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?}",rfname, rftype, rfcollection, rfnull, rfkey, rfdefault, rfextra, rfprivilages, rfcomment);

        }

        // let mut ret_fields: Vec<Field> = table_conn.query_map(format!("SHOW FULL COLUMNS FROM {};", table_name), |row:(String)| {
        //     // let rfname = String::from_utf8(fname)?;
        //     // let rftype = String::from_utf8(ftype)?;
        //     // let rfcollection = String::from_utf8(fcollection)?;
        //     // let rfnull = String::from_utf8(fnull)?;
        //     // let rfkey = String::from_utf8(fkey)?;
        //     // let rfdefault = String::from_utf8(fdefault)?;
        //     // let rfextra = String::from_utf8(fextra)?;
        //     // let rfprivilages = String::from_utf8(fprivilages)?;
        //     // let rfcomment = String::from_utf8(fcomment)?;
        //     // println!("{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?}",fname, ftype, fcollection, fnull, fkey, fdefault, fextra, fprivilages, fcomment);
        //     println!("{:?}",row);
        //     // Field { fname: rfname, ftype: rftype, fcollection: rfcollection, fnull: rfnull, fkey: rfkey, fdefault: rfdefault, fextra: rfextra, fprivilages: rfprivilages, fcomment: rfcomment }
        //     Field{
        //         fname: "".to_string(),
        //         ftype: "".to_string(),
        //         fcollection: "".to_string(),
        //         fnull: "".to_string(),
        //         fkey: "".to_string(),
        //         fdefault: "".to_string(),
        //         fextra: "".to_string(),
        //         fprivilages: "".to_string(),
        //         fcomment: "".to_string()
        //     }
        // }).unwrap();

        // table.fields.append(&mut ret_fields);

        tables.push(table);
    }

    return tables;
}