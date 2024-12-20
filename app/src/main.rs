use app::server::server::{create_server, start_server};
#[tokio::main]
async fn main() {
    let app = create_server().await;
    start_server(app).await;
}