use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::sync::{Arc, Mutex};
use crate::database::Database;
use crate::tables::TableConfig;
use std::path::{Path, PathBuf};
use mysql::prelude::Queryable;
use tokio::task;
use futures::future;
use std::io::Error;

pub async fn import(
    database: Arc<Database>,
    table: Arc<TableConfig>,
    export_path: Arc<PathBuf>,
) {
    let success_counter = Arc::new(Mutex::new(0));
    let error_counter = Arc::new(Mutex::new(0));
    let query_errors = Arc::new(Mutex::new(String::new()));
    let file_path: String = get_file_path(&export_path, &table);
    let error_path: String = get_error_file_name(&export_path, &table);
    let file: File = File::open(&file_path).expect("Unable to open file");
    let reader: BufReader<File> = BufReader::new(file);
    let mut tasks = vec![];
    for line in reader.lines() {
        let query = line.expect("Unable to read line");
        let database_clone = Arc::clone(&database);
        let success_counter_clone = Arc::clone(&success_counter);
        let error_counter_clone = Arc::clone(&error_counter);
        let query_errors_clone = Arc::clone(&query_errors);
        let task = task::spawn(async move {
            let mut conn = database_clone.pool.get_conn().unwrap();
            match conn.query_drop(&query) {
                Ok(_) => {
                    let mut success_counter = success_counter_clone.lock().unwrap();
                    *success_counter += 1;
                }
                Err(err) => {
                    // eprintln!("Error executing query: {}\nError details: {}", query, err);
                    let mut error_counter = error_counter_clone.lock().unwrap();
                    *error_counter += 1;
                    let mut query_errors = query_errors_clone.lock().unwrap();
                    let error = err.to_string();
                    query_errors.push_str(&query);
                    query_errors.push('\n');
                    query_errors.push_str(error.as_str());
                    query_errors.push('\n');
                }
            }
        });
        tasks.push(task);
    }
    future::join_all(tasks).await;
    let success_count = *success_counter.lock().unwrap();
    let error_count = *error_counter.lock().unwrap();
    println!("Completed importing {} with {} success and {} errors", file_path, success_count, error_count);
    if error_count > 0 {
        let error_file = get_error_file(&error_path)
            .expect(&format!("Unable to create error file {}", error_path));
        let mut writer = BufWriter::new(error_file);
        let query_errors = query_errors.lock().unwrap();
        match writer.write_all(query_errors.as_bytes()) {
            Ok(_) => (),
            Err(error) => eprintln!("Error writing to error file: {}\nError details: {}", error_path, error)
        }
    }
}

fn get_file_path(
    export_path: &Arc<PathBuf>,
    table: &Arc<TableConfig>
) -> String {
    let path = export_path.display().to_string();
    match &table.table_rename {
        Some(rename) => format!("{}\\{}.sql", path, rename),
        None => format!("{}\\{}.sql", path, table.name),
    }
}

fn get_error_file_name(export_path: &PathBuf, table: &TableConfig) -> String {
    let path = export_path.display().to_string();
    match &table.table_rename {
        Some(rename) => format!("{}\\err-{}.sql",path, rename),
        None => format!("{}\\{}-error.sql", path, table.name),
    }
}
fn get_error_file(error_path: &String) -> Result<File, Error> {
    let error_path: &Path = Path::new(&error_path);
    let error_file: Result<File, Error>;
    error_file = match error_path.exists() {
        true => File::open(&error_path),
        false => File::create(&error_path),
    };
    return error_file;
}