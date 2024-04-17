mod request;
mod response;

use crate::request::Methods;
use crate::request::Request as req;
use crate::response::Response;
use std::io::Result;
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpStream};

const ADDR: &str = "127.0.0.1:1999";

async fn handle_client(mut socket: TcpStream) {
    match req::parse(&mut socket).await {
        Ok(reqw) => {
            // Just send back the emoji
            if reqw.get_method() == Methods::GB {
                let my_response = Response::sucess(reqw.packet.emoji);

                let _ = socket.write_all(&my_response.smash()).await;
            } else if reqw.get_method() == Methods::PULL {
                let mut string_holder = String::new();

                for b in reqw.packet.emoji.iter() {
                    string_holder.push(*b as char);
                }
                println!("{string_holder:?}, {:?}", reqw.packet.emoji);
                let my_response = Response::sucess(string_holder.as_bytes().to_vec());

                let _ = socket.write_all(&my_response.smash()).await;
            }
        }
        Err(err) => {
            println!("{err:?}");
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
