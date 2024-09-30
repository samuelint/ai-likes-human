pub struct CoreConfiguration {
    pub database_url: String,
}

impl Default for CoreConfiguration {
    fn default() -> Self {
        Self {
            database_url: String::from("sqlite::memory:"),
        }
    }
}
