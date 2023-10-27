pub mod commands;
pub mod discovered_word;
mod migrations;
use clap::Parser;
use migrations::MIGRATIONS;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: commands::Commands,
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let db_path = discovered_words_database_path();
    let mut connection = rusqlite::Connection::open(db_path).expect("failed to open database");
    MIGRATIONS.to_latest(&mut connection)?;
    let cli = Cli::parse();
    commands::handle_command(cli.command, &connection)?;
    Ok(())
}

fn discovered_words_database_path() -> std::path::PathBuf {
    let xdg_base_directories = xdg::BaseDirectories::new().unwrap();
    let data_dir = xdg_base_directories
        .create_data_directory("workvault")
        .unwrap();
    
    data_dir.join("db.sqlite")
}
