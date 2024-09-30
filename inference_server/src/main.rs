use inference_server::{serve, ServeParameters};

#[tokio::main]
async fn main() {
    serve(ServeParameters {
        port: 1234,
        ..ServeParameters::default()
    })
    .await;
}
