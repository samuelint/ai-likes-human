pub struct AppConfiguration {
    pub database_url: String,
}

impl Default for AppConfiguration {
    fn default() -> Self {
        Self {
            database_url: String::from("sqlite::memory:"),
        }
    }
}
