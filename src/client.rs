use std::io::{Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;

fn main() {
    match TcpStream::connect("localhost:7878") {
        Ok(mut stream) => {
            println!(
                "Connected to the server at {}!",
                stream.peer_addr().unwrap()
            );

            let msg = b"hello server";

            stream.write(msg).unwrap();
            println!("Sent, awaiting reply");

            let mut data = [0 as u8; 12];
            match stream.read_exact(&mut data) {
                Ok(_) => {
                    if &data == msg {
                        println!("Reply is ok! Reply: {}", from_utf8(&data).unwrap());
                    } else {
                        let text = from_utf8(&data).unwrap();
                        println!("Unexpected reply: {}", text);
                    }
                }
                Err(e) => {
                    println!("Failed to receive data: {}", e);
                }
            }
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    println!("Terminated.");
}
