use futures::future;
use std::io;
use std::sync::Arc;
use arguments::Args;
use clap::Parser;
use database::Database;

mod arguments;
mod connection;
mod database;
mod tables;
mod table_export;
mod table_import;
mod export_path;
mod mysql_utils;

#[tokio::main]
async fn main() -> io::Result<()> {
    let args: Args = Args::parse();
    let conn_config: connection::ConnectionConfig = connection::get_config(&args);
    let table_config: Vec<tables::TableConfig> = tables::get_config(&args);

    let export_path: Arc<std::path::PathBuf> = export_path::get_export_path(&args);
    export_path::create_export_dir(&args, &export_path);
    let args = &Arc::new(args);
    export(args, &conn_config, &table_config, export_path.clone()).await;

    if !args.export_only {
        import(&conn_config, &table_config, export_path).await;
    }
    Ok(())
}

async fn export(
    args: &Arc<Args>,  // Changed to Arc<Args>
    conn_config: &connection::ConnectionConfig,
    table_config: &Vec<tables::TableConfig>,
    export_path: Arc<std::path::PathBuf>
) {
    let source_db: Arc<Database> = Arc::new(Database::new(&conn_config.source));
    let mut tasks = vec![];

    for tbl in table_config.clone() {
        let database = source_db.clone();
        let table = Arc::new(tbl);

        let task = table_export::export(
            Arc::clone(args),  // Pass cloned Arc<Args> 
            database,
            table,
            export_path.clone()
        );

        tasks.push(task);
    }

    future::join_all(tasks).await;

    println!("----------------------------------------------");
    println!("EXPORT COMPLETE");
    println!("----------------------------------------------");
}

async fn import(
    conn_config: &connection::ConnectionConfig,
    table_config: &Vec<tables::TableConfig>,
    export_path: Arc<std::path::PathBuf>
) {
    let destination_db = Arc::new(Database::new(&conn_config.destination));
    for tbl in table_config.clone() {
        let database = destination_db.clone();
        let table = Arc::new(tbl);
        table_import::import(
            database,
            table,
            export_path.clone()
        ).await;
    }
    
    println!("----------------------------------------------");
    println!("IMPORT COMPLETE");
    println!("----------------------------------------------");
}
