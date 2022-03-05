use std::fs::File;
use std::io::{BufRead, BufReader, Read, Result, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::str;
use std::thread;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = vec![0; 4096];

    loop {
        match stream.read(&mut buffer) {
            Ok(size) => {
                let command = String::from_utf8_lossy(&buffer[0..size]);
                let words: Vec<&str> = command.trim().split_whitespace().collect();

                //write command
                if words[0] == "upload" {
                    //get file path
                    let mut path = PathBuf::from("./publicFiles/");

                    //push filename to path
                    path.push(&words[2]);

                    //create file
                    let mut file = std::fs::File::create(&path).expect("Error creating file");
                    //write data into file opened earlier
                    match file.write(words[3..].join(" ").as_bytes()) {
                        Ok(_) => (),
                        Err(e) => {
                            println!("Error writing to file: {}", e);
                        }
                    }

                    // println!("File uploaded");
                }
                // write
                else if words[0] == "download" {
                    if let Err(e) = send_file(&stream, words[1]) {
                        println!("The file was not able to be downloaded: {:?}", e);
                    }
                } // download
            } // OK
            Err(_) => {
                println!(
                    "An error occurred, terminating connection with {}",
                    stream.peer_addr().unwrap()
                );
                break;
            } // Err
        } // match
    } // loop
}

fn main() {
    let listener = TcpListener::bind("localhost:7878").unwrap();

    println!("Server listening on: {}", listener.local_addr().unwrap());
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move || handle_client(stream));
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }
}

fn send_file(mut stream: &TcpStream, file_name: &str) -> Result<()> {
    //currently downloads files only from publicFiles folder
    let mut path = PathBuf::from("./publicFiles/");
    path.push(file_name);

    let file = File::open(Path::new(&path)).expect("Error opening file");
    let file_size = file.metadata().unwrap().len();

    //file length string ending with \n so server knows when to stop reading
    let mut file_length = file_size.to_string();
    file_length.push_str("\n");

    // sending file size
    match stream.write(&file_length.as_bytes()) {
        Ok(_) => {
            println!("File size sent");
            ()
        }
        Err(e) => {
            println!("Error sending file size to server: {}", e);
        }
    }

    // Reading data from file to send to client
    let mut buffer = Vec::new();
    match File::open(&path) {
        Ok(mut file) => {
            match file.read_to_end(&mut buffer) {
                Ok(_) => (),
                Err(e) => {
                    println!("Error reading file to copy data: {}", e);
                }
            };
        }
        Err(e) => {
            println!("Error opening file to copy data: {}", e);
        }
    };

    // sending file data
    match stream.write(&buffer) {
        Ok(_) => {
            println!("File data sent");
            ()
        }
        Err(e) => {
            println!("Error sending file data to server: {}", e);
        }
    }

    println!("File sent successfully!");
    Ok(())
}
