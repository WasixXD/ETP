use std::io::Result;
use tokio::net::TcpListener;

const ADDR: &str = "127.0.0.1:3019";

#[tokio::main]
async fn main() -> Result<()> {
    let server = TcpListener::bind(ADDR).await?;
    println!("Server listening on {ADDR}");

    match server.accept().await {
        Ok((_socket, addr)) => println!("New client: {addr}"),
        Err(e) => eprintln!("{e}"),
    }

    Ok(())
}
