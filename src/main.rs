use std::io;

#[tokio::main]
async fn main() {
    Volcanis::server::run().await
}
