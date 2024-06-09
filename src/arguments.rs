use clap::Parser;
/// This struct represents the command-line arguments for the program.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// The connection configuration file
    #[arg(long)]
    pub connection_config: Option<String>,
    /// The table configuration file
    #[arg(long)]
    pub table_config: Option<String>,
    /// Export Path
    #[arg(long)]
    pub export_path: Option<String>,
    /// Use extended insert statements
    #[arg(long)]
    pub extended_insert: bool,
    /// Include column names in insert statements
    #[arg(long)]
    pub complete_insert: bool,
    /// Use INSERT IGNORE instead of INSERT
    #[arg(long)]
    pub insert_ignore: bool,
    /// Run in export only mode
    #[arg(long)]
    pub export_only: bool,
    /// Limit the number of rows in extended insert statements
    #[arg(long, default_value_t = 50)]
    pub extended_insert_limit: usize,
    /// Clean previous exports
    #[arg(long)]
    pub clean: bool,
    /// Export a certain table
    #[arg(long)]
    pub table: Option<String>,
    /// Source Database Host/Hostname/IP Address
    #[arg(long)]
    pub source_host: Option<String>,
    /// Source Database Port
    #[arg(long)]
    pub source_port: Option<i64>,
    // Source Database Name
    #[arg(long)]
    pub source_database: Option<String>,
    // Source Database Username
    #[arg(long)]
    pub source_username: Option<String>,
    /// Source Database Password
    #[arg(long)]
    pub source_password: Option<String>,
    /// Destination Database Host/Hostname/IP Address
    #[arg(long)]
    pub destination_host: Option<String>,
    /// Destination Database Port
    #[arg(long)]
    pub destination_port: Option<i64>,
    // Destiatnion Database Name
    #[arg(long)]
    pub destination_database: Option<String>,
    // Destination Database Username
    #[arg(long)]
    pub destination_username: Option<String>,
    /// Destination Database Password
    #[arg(long)]
    pub destination_password: Option<String>
}
