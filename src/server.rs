use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::path::PathBuf;
use std::str;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = vec![0; 4096];

    loop {

        match stream.read(&mut buffer) {
            Ok(size) => {

                let command = String::from_utf8_lossy(&buffer[0..size]);
                println!("{}", command);
                let words: Vec<&str> = command.trim().split_whitespace().collect();
                println!("{}", &words[0]);

                //write command
                if &words[0] == &"upload" {

                    //get file path
                    let mut path =  PathBuf::from("./publicFiles/");

                    //push filename to path
                    path.push(&words[1]);
                    println!("{}", &words[1]);

                    //create file
                    let mut file = std::fs::File::create(&path).expect("Error creating file");

                    //ask for file size
                    match write!(&stream, "{}", &"send file size\n"){
                        Ok(_) => {
                            println!("File size received");
                            ()
                        },
                        Err(e) => {
                            println!("Error sending message to client when asking for file size: {}", e);
                        }
                    }

                    //BufReader to read filesize, filesizebuf to store filesize
                    let mut reader = BufReader::new(&stream);
                    let mut filesizeBuf = Vec::new();

                    //read file size from client
                    match reader.read_until(b'\n', &mut filesizeBuf){
                        Ok(_) => (),
                        Err(e) => {
                            println!("Error reading file size: {}", e);
                        }
                    }

                    filesizeBuf.pop(); // pop the \n

                    //create buffer for file data
                    let filesize = str::from_utf8(&filesizeBuf).unwrap().parse::<usize>().unwrap(); //parse into usize

                    let mut fileData = vec![0; filesize as usize];


                    //read data from user, put data into fileData vec
                    match reader.read_exact(&mut fileData){
                        Ok(_) => (),
                        Err(e) => {
                            println!("Error reading file data: {}", e);
                        }
                    }

                    //write data into file opened earlier
                    match file.write(&fileData[0..fileData.len()]){
                        Ok(_) => (),
                        Err(e) => {
                            println!("Error writing to file: {}", e);
                        }
                    }

                    println!("File uploaded");

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
