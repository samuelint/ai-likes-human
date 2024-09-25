use std::error::Error;

use inference_server::{self, serve, ServeParameters};

#[derive(Clone)]
pub struct InferenceServer {
    port: u16,
}

impl InferenceServer {
    pub fn new(port: u16) -> Self {
        InferenceServer { port }
    }

    pub fn get_url(&self) -> String {
        format!("http://localhost:{}", self.port)
    }

    pub async fn serve(&self) {
        serve(ServeParameters {
            port: self.port,
            use_trace: false,
            ..ServeParameters::default()
        })
        .await
    }

    pub fn is_core_server_up(&self) -> Result<bool, Box<dyn Error>> {
        let url = self.get_url();
        let response = ureq::get(&url).call()?;
        let status = response.status();

        return Ok(status == 200);
    }
}
