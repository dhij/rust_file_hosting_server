use std::fs::File;
use std::io::{self, Result, Write};
use std::net::TcpStream;
use std::path::Path;

fn main() {
    println!("\nThis is a filehosting server written in Rust.\n");

    println!("\n Commands: connect | quit \n");

    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("Error on read");
        println!("Executing CMD: {}", line);
        match line[..].trim() {
            "connect" => {
                command_loop();
                println!("You may connect to the server again or exit the program.");
                println!("\n Commands: connect | quit \n");
            }
            "quit" => {
                break;
            }

            _ => {
                println!("Please enter either connect or quit.");
            }
        }
    }
    println!("Exiting the program...");
}

fn send_file(mut stream: &TcpStream, path: &str) -> Result<()> {
    let file = File::open(Path::new(path))?;
    let file_size = file.metadata().unwrap().len();

    let buffer = vec![0; file_size as usize];
    let written_amt = stream.write(&buffer)?;
    println!("Bytes written to stream: {}", written_amt);
    Ok(())
}

fn command_loop() {
    match TcpStream::connect("localhost:7878") {
        Ok(stream) => {
            println!(
                "Connected to the server at {}!",
                stream.peer_addr().unwrap()
            );

            let command_list = "\n Commands: \n -- upload \n -- download \n -- help \n -- quit \n";

            println!("{}", command_list);

            loop {
                let mut user_input = String::new();
                io::stdin()
                    .read_line(&mut user_input)
                    .expect("Error on read");
                let cmd = user_input.trim();
                match cmd {
                    "upload" => {
                        if let Err(e) = send_file(&stream, "./client_dir/test.txt") {
                            panic!("The file was not able to be sent: {:?}", e);
                        }
                    }
                    "download" => {
                        println!("Command under construction");
                    }
                    "help" => {
                        println!("{}", command_list);
                    }
                    "quit" => {
                        break;
                    }
                    _ => {
                        print!("Please enter a valid command.");
                    }
                }
            }
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    println!("Connection terminated.");
}
