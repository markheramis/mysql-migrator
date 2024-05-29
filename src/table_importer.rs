use crate::TableConfig;
use crate::database::Database;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use mysql::prelude::Queryable;

pub fn import_table(
    db: &mut Database,
    table: &TableConfig,
    file_name: &str
) {
    let path: String = format!("data/{}", file_name);
    let file: File = File::open(path).expect("Unable to open file");
    let reader: BufReader<File> = BufReader::new(file);
    for line in reader.lines() {
        let query: String = line.expect("Unable to read line");
        db.conn.query_drop(query).expect("Query execution failed");
    }
    println!("import {}: completed", &table.name);
}
