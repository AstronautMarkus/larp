use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "larp",
    version,
    about = "A deeply serious system management tool.",
    long_about = None
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Install a package
    Install { package: String },
    /// Upgrade one package, or everything installed
    Upgrade { package: Option<String> },
    /// Remove a package
    Remove { package: String },
    /// Run system diagnostics
    Doctor,
}
