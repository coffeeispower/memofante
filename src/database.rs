mod migrations;
use migrations::MIGRATIONS;

pub fn path() -> std::path::PathBuf {
    let xdg_base_directories = xdg::BaseDirectories::new().expect("Failed to get XDG base directories");
    let data_dir = xdg_base_directories
        .create_data_directory("workvault")
        .unwrap();
    
    data_dir.join("db.sqlite")
}
pub fn setup() -> color_eyre::Result<rusqlite::Connection> {
    let db_path = path();
    let mut connection = rusqlite::Connection::open(db_path)?;
    MIGRATIONS.to_latest(&mut connection)?;
    Ok(connection)
}