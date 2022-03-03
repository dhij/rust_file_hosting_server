use std::fs::File;
use std::io::{self, Read, Result, Write};
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

    let pathNames: Vec<&str> = path.split("/").collect();
    let fileName = pathNames[pathNames.len()-1];
    let mut command = String::from("upload ");

    //command will have upload filename, not file path for server
    command.push_str(fileName);
    println!("Command: {}", command);

    //file length string ending with \n so server knows when to stop reading
    let mut fileLength = file_size.to_string();
    fileLength.push_str("\n");

    // sending command to server
    match stream.write(command.as_bytes()){
        Ok(_) => {
            println!("Upload Command Sent");
            ()
        },
        Err(e) => {
            println!("Error sending upload command to server: {}", e);
        }
    }

    // sending file size
    match stream.write(&fileLength.as_bytes()){
        Ok(_) => {
            println!("File size sent");
            ()
        },
        Err(e) => {
            println!("Error sending file size to server: {}", e);
        }
    }

    // Reading data from file to send to server
    let mut buffer = Vec::new();
    match File::open(&path){
        Ok(mut file) => {
            match file.read_to_end(&mut buffer){
                Ok(_) => (),
                Err(e) => {
                    println!("Error reading file to copy data: {}", e);
                }
            };
        },
        Err(e) => {
            println!("Error opening file to copy data: {}", e);
        }
    };


    // sending file data
    match stream.write(&buffer){
        Ok(_) => {
            println!("File data sent");
            ()
        },
        Err(e) => {
            println!("Error sending file data to server: {}", e);
        }
    }

    println!("File sent successfully!");
    Ok(())

}

fn command_loop() {
    match TcpStream::connect("localhost:7878") {
        Ok(stream) => {
            println!(
                "Connected to the server at {}!",
                stream.peer_addr().unwrap()
            );

            let command_list = "\n Commands: \n -- upload filepath \n -- download \n -- help \n -- quit \n";

            println!("{}", command_list);

            loop {
                let mut user_input = String::new();
                io::stdin()
                    .read_line(&mut user_input)
                    .expect("Error on read");
                let cmd: Vec<&str> = user_input.trim().split_whitespace().collect();

                match cmd[0] {
                    "upload" => {
                        if let Err(e) = send_file(&stream, cmd[1]) {
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
