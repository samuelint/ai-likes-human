use std::error::Error;

use app::configuration::AppConfiguration;
use inference_server::{self, serve, ServeParameters};

#[derive(Clone)]
pub struct InferenceServer {
    port: u16,
    database_url: String,
}

impl InferenceServer {
    pub fn from_configuration(app_configuration: &AppConfiguration) -> Self {
        InferenceServer::new(
            app_configuration.local_server_port,
            &app_configuration.local_database_url,
        )
    }

    pub fn new(port: u16, database_url: &str) -> Self {
        InferenceServer {
            port,
            database_url: database_url.to_string(),
        }
    }

    pub fn get_url(&self) -> String {
        format!("http://localhost:{}", self.port)
    }

    pub async fn serve(&self) {
        serve(ServeParameters {
            port: self.port,
            use_trace: false,
            database_url: self.database_url.clone(),
            ..ServeParameters::default()
        })
        .await
    }

    pub fn is_up(&self) -> Result<bool, Box<dyn Error>> {
        let url = self.get_url();
        let response = ureq::get(&url).call()?;
        let status = response.status();

        return Ok(status == 200);
    }
}
