use app_core::CoreConfiguration;
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct AppConfiguration {
    pub local_database_url: String,
    pub local_server_port: u16,
    pub app_data_directory_path: String,
}

impl Default for AppConfiguration {
    fn default() -> Self {
        AppConfiguration {
            local_database_url: String::from("sqlite::memory:"),
            local_server_port: 1234,
            app_data_directory_path: String::from(""),
        }
    }
}

pub struct AppConfigurationBuilder {
    app_configuration: AppConfiguration,
}

impl AppConfigurationBuilder {
    pub fn new() -> Self {
        Self {
            app_configuration: AppConfiguration::default(),
        }
    }

    pub fn with_app_data_directory_path(mut self, path: &PathBuf) -> Self {
        self.app_configuration.app_data_directory_path = path.to_str().unwrap().to_string();

        self
    }

    pub fn with_local_database(mut self) -> Self {
        let mut database_file_path =
            PathBuf::from(self.app_configuration.app_data_directory_path.as_str());
        database_file_path.push("data_v1.db");
        let database_file_path = database_file_path.to_str().unwrap();

        self.app_configuration.local_database_url =
            format!("sqlite://{}?mode=rwc", database_file_path);

        self
    }

    pub fn with_local_server_port(mut self, port: u16) -> Self {
        self.app_configuration.local_server_port = port;

        self
    }

    pub fn create(self) -> AppConfiguration {
        self.app_configuration
    }
}

impl From<&AppConfiguration> for CoreConfiguration {
    fn from(config: &AppConfiguration) -> Self {
        CoreConfiguration {
            database_url: config.local_database_url.clone(),
            ..CoreConfiguration::default()
        }
    }
}
