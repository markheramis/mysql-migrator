use clap::Parser;

/// This struct represents the command-line arguments for the program.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// The configuration file of the migration
    #[arg(long)]
    pub config: Option<String>,

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
    pub clean: bool
}
