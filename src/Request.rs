use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;

#[derive(Debug, Clone)]
enum Methods {
    PULL,
    GB,
}

#[derive(Debug, Clone)]
enum Charset {
    ASCII,
    UTF8,
}

pub enum PErr {
    PacketError,
    PacketWithoutMethod
}

#[derive(Debug, Clone)]
pub struct Packet {
    method: Methods,
    charset: Charset,

    //will change this
    body: [u8; 128],
}

#[derive(Debug, Clone)]
pub struct Request {
    packet: Packet,
}


const PULL_SIGN: &[u8] = b"pull";
const GB_SIGN: &[u8] = b"gb00";

pub fn check_for_methods(a: &[u8]) -> Option<Methods> {
    println!("{a:?}, {GB_SIGN:?}");
    if a == PULL_SIGN {
        
        return Some(Methods::PULL);
    } else if a == GB_SIGN {
        return Some(Methods::GB);
    }

    None
}

impl Request {

    pub async fn parse(mut socket: TcpStream) -> Result<Self, PErr> {
        let mut r = Request {
            packet: Packet {
                method: Methods::GB,
                charset: Charset::UTF8,
                body: [0; 128],
            }
        };
        let bytes = socket.read(&mut r.packet.body).await.unwrap();

        //check for method
        match check_for_methods(&r.packet.body[0..4]) {
            Some(Methods::PULL) => {
                println!("pull");
            },
            Some(Methods::GB) => {
                println!("gb");
            },
            None => return Err(PErr::PacketWithoutMethod),
        }
        

        todo!()
    }
}
