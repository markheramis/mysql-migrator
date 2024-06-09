use serde_json::Value;
use crate::TableConfig;
pub fn process_table_configuration(table_json: String) -> Vec<TableConfig> {
    let tables: Value = serde_json::from_str(&table_json).expect("Failed to parse table.json");
    let mut table_configs: Vec<TableConfig> = Vec::new();
    for table in tables.as_array().unwrap() {
        let table_config = if table.is_string() {
            TableConfig {
                name: table.as_str().unwrap().to_string(),
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
    return table_configs;
}