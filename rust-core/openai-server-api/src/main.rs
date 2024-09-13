use openai_server_api::{serve, ServeParameters};

#[tokio::main]
async fn main() {
    serve(ServeParameters { port: 3000 }).await;
}
