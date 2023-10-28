mod add;
mod list;
mod remove;
mod review;
use clap::Subcommand;
use rusqlite::Connection;
#[derive(Subcommand, Default)]
#[clap(about = "Start a new review")]
pub enum ReviewMode {
    #[clap(about = "Memofante asks you the meaning of the words")]
    #[default]
    Meaning,
    #[clap(about = "Memofante asks you to read the words")]
    Reading,
}
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
    Review {
        #[clap(subcommand)]
        mode: Option<ReviewMode>,
    },
}

pub fn handle_command(command: Commands, connection: &Connection) -> color_eyre::Result<()> {
    match command {
        Commands::Add { word } => add::add(word, connection)?,
        Commands::List => list::list(connection)?,
        Commands::Remove { word } => remove::remove(word, connection)?,
        Commands::Review { mode } => review::review(connection, mode.unwrap_or_default())?,
    }
    Ok(())
}
