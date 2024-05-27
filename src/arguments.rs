use clap::Parser;

/// This struct represents the command-line arguments for the program.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// The configuration file of the migration
    #[arg(long)]
    pub config: Option<String>,
}
