use network::server::Server;
use std::io;

pub mod game;
pub mod network;
pub mod shared;

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() -> io::Result<()> {
    let server = Server::new("0.0.0.0:8090").await;

    tokio::signal::ctrl_c()
        .await
        .expect("Failed to listen for the shutdown signal");
    println!("Shutting down....");

    Ok(())
}
