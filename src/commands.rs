mod add;
mod list;
mod remove;
use clap::Subcommand;
use rusqlite::Connection;

#[derive(Subcommand)]
#[clap(about = "Manage ✨discovered word✨")]
pub enum Commands {
    #[clap(about = "Add a new ✨discovered word✨")]
    Add { word: String },
    #[clap(
        about = "List your ✨discovered words✨ with the successes, failures and success rate in reviews"
    )]
    List,
    #[clap(about = "Remove a ✨discovered word✨")]
    Remove { word: String },
    #[clap(about = "Start a new review")]
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
