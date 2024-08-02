use crate::database::Database;
use crate::Override;
use crate::TableConfig;
use mysql::Value;
use std::fs::File;
use std::io::Write;
use std::io::BufWriter;
use std::path::PathBuf;
fn escape_string(bytes: &[u8]) -> String {
    let mut escaped_string = String::from_utf8_lossy(bytes).to_string();
    escaped_string = escaped_string.replace("\\", "\\\\");
    escaped_string = escaped_string.replace("\"", "\\\"");
    escaped_string = escaped_string.replace("\n", "\\n");
    escaped_string = escaped_string.replace("\r", "\\r");
    escaped_string = escaped_string.replace("\t", "\\t");
    if escaped_string.is_empty() {
        "NULL".to_string()
    } else {
        format!("\"{}\"", escaped_string)
    }
}
fn value_to_string(value: Value) -> String {
    match value {
        Value::NULL => "NULL".to_string(),
        Value::Bytes(bytes) => escape_string(&bytes),
        Value::Int(int) => int.to_string(),
        Value::UInt(uint) => uint.to_string(),
        Value::Float(float) => float.to_string(),
        Value::Double(double) => double.to_string(),
        Value::Date(year, month, day, hour, minute, second, micro) => {
            if year == 0 && month == 0 && day == 0 && hour == 0 && minute == 0 && second == 0 && micro == 0 {
                "NULL".to_string()
            } else {
                format!("\"{:04}-{:02}-{:02} {:02}:{:02}:{:02}.{:06}\"", year, month, day, hour, minute, second, micro)
            }
        },
        _ => "NULL".to_string(),
    }
}
fn to_vector_string(row: mysql::Row) -> Vec<String> {
    row.unwrap().into_iter().map(value_to_string).collect()
}
fn get_columns(db: &mut Database, table: &TableConfig) -> Vec<String> {
    match &table.columns {
        Some(column_list) if column_list.len() == 1 && column_list[0] == "*" => db.query_columns(&table.name),
        Some(column_list) => column_list.clone(),
        None => db.query_columns(&table.name),
    }
}
fn get_file_name(export_path: &PathBuf, table: &TableConfig) -> String {
    let path = export_path.display().to_string();
    match &table.table_rename {
        Some(rename) => format!("{}\\{}.sql",path, rename),
        None => format!("{}\\{}.sql", path, table.name),
    }
}
fn serde_value_to_mysql_value(value: serde_json::Value) -> mysql::Value {
    match value {
        serde_json::Value::Null => mysql::Value::NULL,
        serde_json::Value::Bool(b) => mysql::Value::from(b),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                mysql::Value::Int(i)
            } else if let Some(u) = n.as_u64() {
                mysql::Value::UInt(u)
            } else if let Some(f) = n.as_f64() {
                mysql::Value::Double(f)
            } else {
                mysql::Value::NULL
            }
        }
        serde_json::Value::String(s) => mysql::Value::Bytes(s.into_bytes()),
        serde_json::Value::Array(_) | serde_json::Value::Object(_) => mysql::Value::NULL, // handle complex types as needed
    }
}
fn apply_overrides(values: &mut Vec<String>, columns: &Vec<String>, overrides: &Option<Vec<Override>>) {
    if let Some(overrides) = overrides {
        for override_conf in overrides {
            let pos = columns.iter().position(|col| col == &override_conf.name);
            if let Some(pos) = pos {
                let override_value = value_to_string(serde_value_to_mysql_value(override_conf.value.clone()));
                if values[pos] == override_value {
                    for (col, val) in &override_conf.set {
                        if let Some(set_pos) = columns.iter().position(|c| c == col) {
                            values[set_pos] = value_to_string(serde_value_to_mysql_value(val.clone()));
                        }
                    }
                }
            }
        }
    }
}
fn rename_columns(columns: &mut Vec<String>, column_rename: &Option<std::collections::HashMap<String, String>>) {
    if let Some(rename_map) = column_rename {
        for (old_name, new_name) in rename_map {
            if let Some(pos) = columns.iter().position(|col| col == old_name) {
                columns[pos] = new_name.clone();
            }
        }
    }
}
pub fn export_table(
    db: &mut Database,
    export_path: &PathBuf,
    table: &TableConfig,
    extended_insert: bool,
    extended_insert_limit: usize,
    complete_insert: bool,
    insert_ignore: bool,
) {
    let mut columns: Vec<String> = get_columns(db, table);
    columns.iter_mut().for_each(|s| {
        s.insert(0, '`');
        s.push('`');
    });
    let result = db.query_table_unbuffered(&table.name, &columns.join(", "), &table.condition);
    let file_name: String = get_file_name(export_path, table);
    let file: File = File::create(&file_name).expect("Unable to create file");
    let mut writer: BufWriter<File> = BufWriter::new(file);
    if let Ok(mut query_result) = result {
        let insert_prefix = if insert_ignore {
            "INSERT IGNORE INTO"
        } else {
            "INSERT INTO"
        };
        let mut insert_statement = String::new();
        let mut row_count = 0;
        while let Some(row) = query_result.next() {
            let row: mysql::Row = row.unwrap();
            let mut values: Vec<String> = to_vector_string(row);
            apply_overrides(&mut values, &columns, &table.overrides);
            // Rename columns after applying overrides
            rename_columns(&mut columns, &table.column_rename);
            // Create column_names after renaming columns
            let column_names = if complete_insert {
                format!(" ({})", columns.join(", "))
            } else {
                String::new()
            };
            if extended_insert {
                if row_count == 0 {
                    insert_statement.push_str(&format!(
                        "{} {}{} VALUES",
                        insert_prefix,
                        table.table_rename.as_ref().unwrap_or(&table.name),
                        column_names
                    ));
                } else {
                    insert_statement.push_str(",");
                }
                insert_statement.push_str(&format!(" ({})", values.join(", ")));
                row_count += 1;
                if row_count >= extended_insert_limit {
                    insert_statement.push_str(";");
                    writeln!(writer, "{}", insert_statement).expect("Unable to write to file");
                    insert_statement.clear();
                    row_count = 0;
                }
            } else {
                // Use renamed columns in the INSERT statements
                insert_statement = format!(
                    "{} {}{} VALUES ({});",
                    insert_prefix,
                    table.table_rename.as_ref().unwrap_or(&table.name),
                    column_names,
                    values.join(", ")
                );
                writeln!(writer, "{}", insert_statement).expect("Unable to write to file");
                insert_statement.clear();
            }
        }
        if extended_insert && !insert_statement.is_empty() {
            insert_statement.push_str(";");
            writeln!(writer, "{}", insert_statement).expect("Unable to write to file");
        }
        println!("export {}: completed", &table.name);
    }
    writer.flush().expect("Failed to flush the buffer");
}
