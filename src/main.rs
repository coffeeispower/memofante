pub mod commands;
pub mod database;
pub mod discovered_word;
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: commands::Commands,
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let connection = database::setup()?;
    let cli = Cli::parse();
    commands::handle_command(cli.command, &connection)?;
    Ok(())
}
