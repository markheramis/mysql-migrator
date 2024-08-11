use crate::arguments::Args;
use crate::database::Database;
use crate::tables::{Override, TableConfig};
use mysql::prelude::Queryable;
use mysql::Row;
use tokio::task;
use std::io::BufWriter;
use std::path::PathBuf;
use std::fs::File;
use std::io::Write;
use std::collections::HashMap;
use std::sync::Arc;
use crate::mysql_utils::to_vector_string;
pub fn export(
    args: Arc<Args>,
    database: Arc<Database>, 
    table: Arc<TableConfig>, 
    export_path: Arc<PathBuf>,
) -> task::JoinHandle<()> {
    let insert_prefix = if args.insert_ignore {
        "INSERT IGNORE INTO"
    } else {
        "INSERT INTO"
    };
    
    let task: task::JoinHandle<()> = task::spawn({
        // Clone the Arcs to move into the async block
        let args = Arc::clone(&args);
        let database = Arc::clone(&database);
        let table = Arc::clone(&table);
        let export_path = Arc::clone(&export_path);

        async move {
            let columns: Vec<String> = get_columns(&database, &table);
            let file_name = get_file_name(&export_path, &table.name, &table.table_rename);

            let mut writer = BufWriter::new(File::create(&file_name).expect("Unable to create file"));
            
            match query_data(&database, &table.name, &columns, &table.condition) {
                Ok(rows) => {
                    let mut row_count = 0;
                    // Use a for loop to iterate over the rows
                    for row in rows {
                        let mut values = to_vector_string(row);
                        apply_overrides(&mut values, &columns, &table.overrides);
                        rename_columns(&mut columns.clone(), &table.column_rename);
                        if args.extended_insert {
                            handle_extended_insert(
                                &mut row_count,
                                args.extended_insert_limit,
                                &values,
                                &columns,
                                insert_prefix,
                                &table,
                                &mut writer,
                                args.complete_insert
                            );
                        } else {
                            handle_regular_insert(
                                &values,
                                &columns,
                                insert_prefix,
                                &table,
                                &mut writer,
                                args.complete_insert
                            );
                        }
                    }
                }
                Err(err) => eprintln!("Error fetching rows: {:?}", err),
            }

            println!("Exported {}", table.name);
        }
    });
    task
}

fn get_columns(
    database: &Database,
    table: &TableConfig,
) -> Vec<String> {
    match &table.columns {
        Some(cols) if !(cols.len() == 1 && cols[0] == "*") => cols.clone(),
        _ => query_columns(&database, &table.name),
    }
}

fn query_columns(
    database: &Database,
    table: &str
) -> Vec<String> {
    let query = format!(
        "SELECT COLUMN_NAME FROM INFORMATION_SCHEMA.COLUMNS WHERE TABLE_NAME = '{}' AND TABLE_SCHEMA = '{}'",
        table, 
        database.name
    );
    let mut conn = database.pool.get_conn().unwrap();
    conn.query_map(query, |column_name: String| column_name).unwrap()
}

fn query_data(
    database: &Database,
    table: &str,
    columns: &Vec<String>,
    condition: &Option<String>,
) -> Result<Vec<Row>, mysql::Error> {
    let mut query = format!("SELECT {} FROM {}", columns.join(", "), table);
    if let Some(cond) = condition {
        query.push_str(&format!(" WHERE {}", cond));
    }
    
    let mut conn = database.pool.get_conn().unwrap();
    let rows: Vec<Row> = conn.query(query)?;
    Ok(rows)
}

fn get_file_name(
    export_path: &PathBuf,
    table_name: &String,
    table_rename: &Option<String>,
) -> String {
    let path = export_path.display().to_string();
    match table_rename {
        Some(rename) => format!("{}/{}.sql", path, rename),
        None => format!("{}/{}.sql", path, table_name),
    }
}
fn handle_extended_insert(
    row_count: &mut usize,
    extended_insert_limit: usize,
    values: &Vec<String>,
    columns: &Vec<String>,
    insert_prefix: &str,
    table: &TableConfig,
    writer: &mut BufWriter<File>,
    complete_insert: bool
) {
    let mut insert_statement = String::new();
    let column_list = columns_to_str(columns, complete_insert);
    let value_list = values_to_str(values);
    if *row_count == 0 {
        insert_statement.push_str(&format!(
            "{} {} {} VALUES",
            insert_prefix,
            table.table_rename.as_ref().unwrap_or(&table.name),
            column_list
        ));
    } else {
        insert_statement.push(',');
    }
    insert_statement.push_str(&format!(" ({})", value_list));
    *row_count += 1;
    
    if *row_count >= extended_insert_limit {
        insert_statement.push(';');
        writeln!(writer, "{}", insert_statement).expect("Unable to write to file");
        insert_statement.clear();
        *row_count = 0;
    }
}

fn handle_regular_insert(
    values: &Vec<String>,
    columns: &Vec<String>,
    insert_prefix: &str,
    table: &TableConfig,
    writer: &mut BufWriter<File>,
    complete_insert: bool
) {
    let column_list = columns_to_str(columns, complete_insert);
    let value_list = values_to_str(values);
    writeln!(
        writer,
        "{} {} {} VALUES ({});",
        insert_prefix,
        table.table_rename.as_ref().unwrap_or(&table.name),
        column_list,
        value_list
    ).expect("Unable to write to file");
}

fn columns_to_str(columns: &Vec<String>, complete_insert: bool) -> String {
    if complete_insert {
        write_column_query(columns)
    } else {
        String::new()
    }
}

fn values_to_str(values: &Vec<String>) -> String {
    values.iter().map(|value| {
        if value == "NULL" || value == "0000-00-00" || value == "0000-00-00 00:00:00" {
            "NULL".to_string()
        } else {
            format!("\"{}\"", value)
        }
    }).collect::<Vec<String>>().join(", ")
}

fn apply_overrides(
    values: &mut Vec<String>,
    columns: &Vec<String>,
    overrides: &Option<Vec<Override>>,
) {
    if let Some(overrides) = overrides {
        for o in overrides {
            match columns.iter().position(|col| col == &o.name) {
                Some(pos) => {
                    if values[pos] == o.value {
                        o.set.iter().for_each(|(o_column, val)| {
                            if let Some(set_position) = columns.iter().position(|col| col == o_column) {
                                values[set_position] = val.clone();
                            }
                        });
                    }
                }
                _ => (),
            }
        }
    }
}

fn rename_columns(
    columns: &mut Vec<String>,
    column_rename: &Option<HashMap<String, String>>
) {
    if let Some(rename_map) = column_rename {
        for (old_name, new_name) in rename_map {
            if let Some(pos) = columns.iter().position(|col| col == old_name) {
                columns[pos] = new_name.clone();
            }
        }
    }
}

fn write_column_query(columns: &Vec<String>) -> String {
    let mut c = columns.clone();
    c.iter_mut().for_each(|s| {
        s.insert(0, '`');
        s.push('`');
    });
    return format!("({})", c.join(", "));
}