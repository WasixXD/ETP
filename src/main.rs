mod request;

use crate::request::Request as req;
use std::io::Result;
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpStream};

const ADDR: &str = "127.0.0.1:1999";

async fn handle_client(mut socket: TcpStream) {
    match req::parse(&mut socket).await {
        Ok(a) => a,
        Err(_) => {
            socket.write_all("erro".as_bytes()).await.unwrap();
            return;
        }
    };
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
            }
            Err(e) => println!("Failed"),
        }
    }
}
