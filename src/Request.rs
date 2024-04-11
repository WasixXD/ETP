
use tokio::net::TcpStream;
use tokio::io::AsyncReadExt;


enum Methods {
    PULL,
    GB
}

enum Charset {
    ASCII,
    UTF8,
}


pub struct Packet {
    method: Methods,
    charset: Charset,

    //will change this
    body: String,
}


pub struct Request {
    packet: Packet
}


impl Request {
    pub async fn parse(mut socket: TcpStream) -> &'static str  {
        let mut buffer = String::new();
        socket.read_to_string(&mut buffer).await;
        let lines = buffer.lines();
        println!("kkk");


        for line in lines {
            println!("{line:?}");
        }

    



        "asdf"

    }
}