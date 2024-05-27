// Import necessary modules
mod arguments;
mod database;
mod table_exporter;
mod table_importer;

use arguments::Args;
use clap::Parser;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use serde_json::Value;
#[derive(Debug, Deserialize)]
struct Config {
    source: DatabaseConfig,
    destination: DatabaseConfig,
}
#[derive(Debug, Deserialize)]
struct DatabaseConfig {
    host: String,
    port: u16,
    name: String,
    user: String,
    pass: String,
    tables: Vec<TableConfig>,
}
#[derive(Debug, Deserialize)]
struct TableConfig {
    name: String,
    #[serde(default)]
    columns: Option<Vec<String>>,
    table_rename: Option<String>,
    condition: Option<String>,
    overrides: Option<Vec<Override>>,
    column_rename: Option<HashMap<String, String>>,
}
#[derive(Debug, Deserialize)]
struct Override {
    name: String,
    value: Value,
    set: HashMap<String, Value>,
}
fn main() {
    let args: Args = Args::parse();
    let config_file: &str = args.config.as_deref().unwrap_or("config.json");
    let config: Config = match fs::read_to_string(config_file) {
        Ok(content) => serde_json::from_str(&content).expect("Invalid config file format"),
        Err(_) => {
            eprintln!("Config file {} does not exist", config_file);
            return;
        }
    };
    let data_path: &Path = Path::new("data");
    if !data_path.exists() {
        fs::create_dir_all(data_path).expect("Failed to create data directory");
    }
    let mut source_db: database::Database = database::Database::new(&config.source);
    for table in &config.source.tables {
        println!("exporting {}: in progress", &table.name);
        table_exporter::export_table(&mut source_db, table);
        println!("exporting {}: completed", &table.name);
    }
    let mut destination_db: database::Database = database::Database::new(&config.destination);
    for table in &config.destination.tables {
        println!("importing {}: in progress", &table.name);
        let file_name: String = format!("{}.sql", table.name);
        table_importer::import_table(&mut destination_db, &file_name);
        println!("importing {}: completed", &table.name);
    }
}
