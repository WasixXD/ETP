use std::env;
use std::io::Result;
use std::io::Write;
use std::mem;
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

    // TODO: THIS HAS TO BE REFACTORED
    // let packet = Packet {
    //     method: method.as_bytes(),
    //     emoji: emoji.as_bytes(),
    //     charset: "utf8".as_bytes(),
    //     version: "ETP/0.1".as_bytes(),
    //     headers: "Sender: carrier/1.0".as_bytes()
    // };

    // let sender: [u8; mem::size_of::<Packet>()] = unsafe {
    //       mem::transmute(packet.clone())
    // };

    // println!("{packet:?}");
    // println!("{sender:?}");

    if method == "gb" {
        method.push_str("00");
    }
    let packet: Vec<u8> = [
        method.as_bytes(),
        emoji.as_bytes(),
        "utf8".as_bytes(),
        "ETP/0.1".as_bytes(),
        "Sender: carrier/1.0".as_bytes(),
    ]
    .concat();

    println!("{packet:?}");
    let mut stream = TcpStream::connect(addr)?;

    stream.write(&packet)?;

    Ok(())
}
