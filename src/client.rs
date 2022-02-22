use std::fs::File;
use std::io::{Error, Read, Result, Write};
use std::net::TcpStream;
use std::path::Path;

fn main() {
    match TcpStream::connect("localhost:7878") {
        Ok(stream) => {
            println!(
                "Connected to the server at {}!",
                stream.peer_addr().unwrap()
            );

            if let Err(e) = send_file(stream, "./client_dir/test.txt") {
                panic!("The file was not able to be sent: {:?}", e);
            }
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    println!("Terminated.");
}

fn send_file(mut stream: TcpStream, path: &str) -> Result<()> {
    let file = File::open(Path::new(path))?;
    let file_size = file.metadata().unwrap().len();

    let buffer = vec![0; file_size as usize];
    let written_amt = stream.write(&buffer)?;
    println!("Bytes written to stream: {}", written_amt);
    Ok(())
}
