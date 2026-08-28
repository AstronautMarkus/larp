mod cli;
mod commands;
mod scripts;
mod state;
mod theatrics;

use clap::Parser;
use cli::{Cli, Commands};
use state::AppState;

fn main() {
    let cli = Cli::parse();
    let mut state = AppState::load();

    match cli.command {
        Commands::Install { package } => commands::install::run(&mut state, &package),
        Commands::Remove { package } => commands::remove::run(&mut state, &package),
        Commands::Upgrade { package } => commands::upgrade::run(&mut state, package.as_deref()),
        Commands::Doctor => commands::doctor::run(&state),
    }

    state.save();
}
