use crate::TableConfig;
use crate::database::Database;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::path::PathBuf;
use mysql::prelude::Queryable;

pub fn import_table(
    db: &mut Database,
    table: &TableConfig,
    export_path: &PathBuf
) {
    let export_file: String = get_file_name(export_path, table);
    let file: File = File::open(export_file).expect("Unable to open file");
    let reader: BufReader<File> = BufReader::new(file);
    for line in reader.lines() {
        let query: String = line.expect("Unable to read line");
        db.conn.query_drop(query).expect("Query execution failed");
    }
    println!("import {}: completed", &table.name);
}

fn get_file_name(export_path: &PathBuf, table: &TableConfig) -> String {
    let path = export_path.display().to_string();
    match &table.table_rename {
        Some(rename) => format!("{}\\{}.sql",path, rename),
        None => format!("{}\\{}.sql", path, table.name),
    }
}