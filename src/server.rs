use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::str::from_utf8;
use std::thread;
use std::fs;
use std::fs::{File, OpenOptions};
use std::path::PathBuf;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = vec![0; 4096];


    loop {

        match stream.read(&mut buffer) {
            Ok(size) => {


                println!(
                    "Buffer currently holds: {:?}",
                    String::from_utf8_lossy(&buffer[0..size])
                );

                let command = String::from_utf8_lossy(&buffer[0..size]);
                println!("{}", command);
                let words: Vec<&str> = command.trim().split_whitespace().collect();

                //write command
                if &words[0] == &"write" {

                    //get file path
                    let mut path =  PathBuf::from("./publicFiles/");
                    path.push(words[1]);


                    //create file
                    let mut file = match OpenOptions::new()
                        .write(true)
                        .create(true)
                        .open("./publicFiles/hello.txt"){
                        Err(e) =>{
                            println!("Error opening file for write operation");
                        },
                        Ok(file) => ()
                    };

                    //ask for file size
                    match write!(&stream, "{}", &"send file size\n"){
                        Ok(_) => {
                            println!("File size received: ");
                            ()
                        },
                        Err(e) => {
                            println!("Error sending message to client when asking for file size");
                        }
                    }

                    //read file size from client
                    let mut reader = BufReader::new(&stream);
                    let mut filesizeBuf: Vec<u8> = Vec::new();


                    //create buffer for file data

                    //add data to file


                } // write
                else if words[0] == "get" {

                } // get


            }// OK
            Err(_) => {
                println!(
                    "An error occurred, terminating connection with {}",
                    stream.peer_addr().unwrap()
                );
                break;
            }// Err
        }// match

    }// loop




}

fn main() {

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    println!("Server listening on: {}", listener.local_addr().unwrap());
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move || handle_client(stream));
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }
}
