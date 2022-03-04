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
                println!("{}", command);
                let words: Vec<&str> = command.trim().split_whitespace().collect();
                println!("{:?}", &words);

                //write command
                if words[0] == "upload" {
                    //get file path
                    let mut path = PathBuf::from("./publicFiles/");

                    //push filename to path
                    path.push(&words[1]);
                    println!("{}", &words[1]);

                    //create file
                    let mut file = std::fs::File::create(&path).expect("Error creating file");

                    //ask for file size
                    match write!(&stream, "{}", &"file size request\n") {
                        Ok(_) => {
                            println!("File size received");
                            ()
                        }
                        Err(e) => {
                            println!(
                                "Error sending message to client when asking for file size: {}",
                                e
                            );
                        }
                    }

                    //BufReader to read filesize, filesizebuf to store filesize
                    let mut reader = BufReader::new(&stream);
                    let mut filesize_buf = Vec::new();

                    //read file size from client
                    match reader.read_until(b'\n', &mut filesize_buf) {
                        Ok(_) => (),
                        Err(e) => {
                            println!("Error reading file size: {}", e);
                        }
                    }

                    filesize_buf.pop(); // pop the \n

                    //create buffer for file data
                    let filesize = str::from_utf8(&filesize_buf)
                        .unwrap()
                        .parse::<usize>()
                        .unwrap(); //parse into usize

                    let mut file_data = vec![0; filesize as usize];

                    //read data from user, put data into fileData vec
                    match reader.read_exact(&mut file_data) {
                        Ok(_) => (),
                        Err(e) => {
                            println!("Error reading file data: {}", e);
                        }
                    }

                    //write data into file opened earlier
                    match file.write(&file_data[0..file_data.len()]) {
                        Ok(_) => (),
                        Err(e) => {
                            println!("Error writing to file: {}", e);
                        }
                    }

                    println!("File uploaded");
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
