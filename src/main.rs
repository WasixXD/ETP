mod Request;

use crate::Request::Request as req;
use std::io::Result;
use tokio::net::{TcpListener, TcpStream};

const ADDR: &str = "127.0.0.1:3019";

async fn handle_client(socket: TcpStream) {
    
    let response = req::parse(socket).await;

    
}


#[tokio::main]
async fn main() -> Result<()> {
    let server = TcpListener::bind(ADDR).await?;
    println!("Server listening on {ADDR}");

    loop {
        match server.accept().await {
            Ok((socket, addr)) => {
                println!("Got new client! {addr}");
                handle_client(socket).await;
            },
    
            Err(e) => println!("Failed"),
        }
            
    }

    Ok(())
}
