use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author = "hirontan", version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize a new Archflow project configuration
    Init,
    /// Plan the architecture based on contracts and schemas
    Plan,
    /// Scaffold code structure and artifact sidecars
    Scaffold,
}
