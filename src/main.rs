use saidl_cli as cli;

#[tokio::main]
async fn main() {
    cli::run().await;
}
