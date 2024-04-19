mod request;
mod response;

use crate::request::Request as req;
use crate::request::{Methods, PErr};
use crate::response::Response;
use std::io::Result;
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpStream};

use std::str;

const ADDR: &str = "127.0.0.1:1999";

fn process_emoji(e: &Vec<u8>) -> String {
    let raw: Vec<String> = e.iter().map(|b| char::from(*b).to_string()).collect();
    raw.join("")
}

async fn handle_client(mut socket: TcpStream) {
    match req::parse(&mut socket).await {
        Ok(reqw) => {
            // Just send back the emoji
            if reqw.get_method() == Methods::GB {
                let my_response = Response::sucess(reqw.packet.emoji);

                let _ = socket.write_all(&my_response.smash()).await;
            } else if reqw.get_method() == Methods::PULL {
                let processed_emoji = process_emoji(&reqw.packet.emoji);
                let my_response = Response::sucess(processed_emoji.as_bytes().to_vec());

                let _ = socket.write_all(&my_response.smash()).await;
            }
        }
        Err(err) => {
            let error_body = match err {
                PErr::WithoutCharset => "Please Provide a valid charset",
                PErr::WithoutMethod => "Please Provide a valid method",
                PErr::Error => "A Error has occured",
            };
            let p = Response::error(error_body.as_bytes().to_vec());
            let _ = socket.write_all(&p.smash()).await;
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
