use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

const MIGRATIONS: EmbeddedMigrations = embed_migrations!();
type DB = diesel::sqlite::Sqlite;

pub fn migrate_database(connection: &mut impl MigrationHarness<DB>) {
    connection
        .run_pending_migrations(MIGRATIONS)
        .expect("Could not run migrations");
}
