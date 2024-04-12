use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;

#[derive(Debug, Clone)]
pub enum Methods {
    PULL,
    GB,
}

#[derive(Debug, Clone)]
pub enum Charset {
    ASCII,
    UTF8,
}

pub enum PErr {
    Error,
    WithoutMethod,
    WithoutCharset,
}

#[derive(Debug, Clone)]
pub struct Packet {
    method: Methods,
    charset: Charset,
    emoji: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct Request {
    packet: Packet,
}

const ASCII_SIGN: &[u8] = b"ascii";
const UTF8_SIGN: &[u8] = b"utf8-";

const PULL_SIGN: &[u8] = b"pull";
const GB_SIGN: &[u8] = b"gb--";

pub fn check_for_methods(a: &[u8]) -> Option<Methods> {
    //Maybe iterating over the array we can be fast by checking if
    //the sign appers on the firsts bytes
    //TODO: Test this outcome
    if a == PULL_SIGN {
        return Some(Methods::PULL);
    } else if a == GB_SIGN {
        return Some(Methods::GB);
    }

    None
}

pub fn check_for_charset(a: &[u8]) -> Option<Charset> {
    if a == ASCII_SIGN {
        return Some(Charset::ASCII);
    } else if a == UTF8_SIGN {
        return Some(Charset::UTF8);
    }
    None
}

pub fn get_emoji(a: &[u8]) -> Vec<u8> {
    let b: Vec<_> = a
        .iter()
        .copied()
        .filter(|value| value != &(0_u8))
        .collect();
    b
}

impl Request {
    pub async fn parse(socket: &mut tokio::net::TcpStream) -> Result<Self, PErr> {
        let mut r = Request {
            packet: Packet {
                method: Methods::GB,
                charset: Charset::UTF8,
                emoji: vec![0],
            },
        };
        let mut data = [0; 64];
        let _ = socket.read(&mut data).await.unwrap();

        //check for method
        match check_for_methods(&data[0..4]) {
            Some(Methods::PULL) => {
                r.packet.method = Methods::PULL;
            }
            Some(Methods::GB) => {
                r.packet.method = Methods::GB;
            }
            None => return Err(PErr::WithoutMethod),
        }

        //emoji
        r.packet.emoji = get_emoji(&data[4..24]);

        //check for charset
        match check_for_charset(&data[24..29]) {
            Some(Charset::ASCII) => {
                r.packet.charset = Charset::ASCII;
            }
            Some(Charset::UTF8) => {
                r.packet.charset = Charset::UTF8;
            }
            None => return Err(PErr::WithoutCharset),
        }

        let rest = String::from_utf8_lossy(&data[29..data.len() - 1]);

        println!(
            "{:?} {:?} {:?}\n{:?}",
            r.packet.method,
            String::from_utf8_lossy(&r.packet.emoji),
            r.packet.charset,
            rest
        );

        Ok(r)
    }
}
