use lazy_static::lazy_static;
use rusqlite_migration::{Migrations, M};

lazy_static! {
    pub static ref MIGRATIONS: Migrations<'static> = Migrations::new(vec![M::up(
        r#"
            CREATE TABLE discovered_words(
                word text NOT NULL PRIMARY KEY,
                success_reviews int NOT NULL DEFAULT 0,
                failed_reviews int NOT NULL DEFAULT 0,
                total_reviews int GENERATED ALWAYS AS (success_reviews + failed_reviews) VIRTUAL
            );
            "#,
    ),]);
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn migrations_test() {
        MIGRATIONS.validate().expect("Migrations are invalid");
    }
}
