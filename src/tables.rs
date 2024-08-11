use std::fs;
use std::collections::HashMap;
use serde::Deserialize;
use serde_json::Value;

use arguments::Args;

use crate::arguments;

#[derive(Debug, Deserialize, Clone)]
pub struct Override {
    pub name: String,
    pub value: String,
    pub set: HashMap<String, String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TableConfig {
    pub name: String,
    #[serde(default)]
    pub columns: Option<Vec<String>>,
    pub table_rename: Option<String>,
    pub condition: Option<String>,
    pub overrides: Option<Vec<Override>>,
    pub column_rename: Option<HashMap<String, String>>,
}

pub fn get_config(args: &Args) -> Vec<TableConfig> {
    let config: &str = args.table_config
        .as_deref()
        .unwrap_or("table.json");
    let table_json: String = fs::read_to_string(config)
        .expect("Failed to read table.json");

    let table_configs: Vec<TableConfig> = process_table_configuration(table_json);

    table_configs
}

fn process_table_configuration(table_json: String) -> Vec<TableConfig> {
    let tables: Value = serde_json::from_str(&table_json).expect("Failed to parse table.json");
    let mut table_configs: Vec<TableConfig> = Vec::new();
    for table in tables.as_array().expect("Expected an array of tables") {
        let table_config = if table.is_string() {
            let name = table.as_str().unwrap().to_string(); 
            TableConfig {
                name,
                columns: None,
                table_rename: None,
                condition: None,
                overrides: None,
                column_rename: None,
            }
        } else {
            serde_json::from_value(table.clone()).expect("Failed to parse table")
        };
        table_configs.push(table_config);
    }
    table_configs
}
