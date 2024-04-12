use std::env;
use std::io::Result;
use std::io::{Write, Read};
use std::net::TcpStream;

#[derive(Debug, Clone)]
struct Packet<'a> {
    method: &'a [u8],
    emoji: &'a [u8],
    charset: &'a [u8],
    version: &'a [u8],
    headers: &'a [u8],
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let (_, addr, mut method, emoji) = if args.len() >= 4 {
        (
            args[0].clone(),
            args[1].clone(),
            args[2].clone(),
            args[3].clone(),
        )
    } else {
        (String::new(), String::new(), String::new(), String::new())
    };


    // TODO: REFACTOR ALL
    if method == "gb" {
        method.push_str("--");
    }

    // println!("{:?}", emoji.as_bytes().len());
    let emoji_bytes = &mut [0;20];

    //TODO: Refactor
    for i in 0..20 {
        if i < emoji.as_bytes().len() {
            emoji_bytes[i] = emoji.as_bytes()[i];
        } else {
            emoji_bytes[i] = 0;
        }
    }

    // first 4 bytes => method
    // next 20 bytes => emoji
    // next 4 bytes => charset
    // 
    let packet: Vec<u8> = [
        method.as_bytes(),
        emoji_bytes,
        "utf8-".as_bytes(),
        "ETP/0.1".as_bytes(),
        "Sender: carrier/1.0".as_bytes(),
    ]
    .concat();

    println!("{packet:?}");
    let mut stream = TcpStream::connect(addr)?;

    stream.write(&packet)?;


    stream.read

    Ok(())
}
