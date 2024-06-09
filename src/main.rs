use serde::{Deserialize, Serialize};
use serde_json::{json, Map};
use std::{collections::HashMap, fs, path::PathBuf};
use arguments::Args;
use clap::Parser;
use config_table::process_table_configuration;
mod arguments;
mod database;
mod table_exporter;
mod table_importer;
mod config_conn;
mod config_table;
#[derive(Debug, Deserialize)]
struct ConnectionConfig {
    source: ConnectionDatabaseConfig,
    destination: ConnectionDatabaseConfig,
}
#[derive(Debug, Serialize, Deserialize)]
struct ConnectionDatabaseConfig {
    hostname: String,
    port: u16,
    database: String,
    username: String,
    password: String,
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
    value: serde_json::Value,
    set: HashMap<String, serde_json::Value>,
}
fn main() {
    let args: Args = Args::parse();
    let connection_config: &str = args.connection_config.as_deref().unwrap_or("connection.json");
    let mut conn: serde_json::Value = match fs::read_to_string(connection_config) {
        Ok(content) => serde_json::from_str(&content).unwrap(),
        Err(_) => serde_json::from_str("{}").unwrap()
    };
    let conn_map: &mut Map<String, serde_json::Value> = conn.as_object_mut().unwrap();
    config_conn::process_connection_configuration(&args, conn_map);
    let conn: ConnectionConfig = serde_json::from_value(json!(conn_map)).expect("Failed to deserialize ConnectionConfig");
    let table_config: &str = args.table_config.as_deref().unwrap_or("table.json");
    let table_json: String = fs::read_to_string(table_config).expect("Failed to read table.json");
    let table_configs: Vec<TableConfig>;
    table_configs = process_table_configuration(table_json);
    
    let default_path: PathBuf = std::env::current_dir().unwrap().join("mysql-migrator");
    let export_path: &PathBuf = &args.export_path.map(PathBuf::from).unwrap_or(default_path);
    if !export_path.exists() {
        fs::create_dir_all(export_path).expect("Failed to create data directory");
    } else {
        if args.clean {
            fs::remove_dir_all(export_path).expect("Failed to clear data directory");
            fs::create_dir_all(export_path).expect("Failed to create data directory");
        }
    }
    let mut source_db: database::Database = database::Database::new(&conn.source);
    for table in &table_configs {
        table_exporter::export_table(
            &mut source_db,
            export_path,
            table,
            args.extended_insert,
            args.extended_insert_limit,
            args.complete_insert,
            args.insert_ignore,
        );
    }
    if !args.export_only {
        let mut destination_db: database::Database = database::Database::new(&conn.destination);
        for table in &table_configs {
            table_importer::import_table(&mut destination_db, table,export_path);
        }
    } else {
        println!("Export-only mode: Skipping import process");
    }
}
