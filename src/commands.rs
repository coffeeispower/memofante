mod add;
mod list;
mod remove;
use clap::Subcommand;
use rusqlite::Connection;

#[derive(Subcommand)]
pub enum Commands {
    Add { word: String },
    List,
    Remove { word: String },
    Review,
}

pub fn handle_command(command: Commands, connection: &Connection) -> color_eyre::Result<()> {
    match command {
        Commands::Add { word } => add::add(word, connection)?,
        Commands::List => list::list(connection)?,
        Commands::Remove { word } => remove::remove(word, connection)?,
        Commands::Review => todo!(),
    }
    Ok(())
}
