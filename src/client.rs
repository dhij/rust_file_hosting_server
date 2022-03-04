use std::fs::File;
use std::io::{self, BufRead, BufReader, Read, Result, Write};
use std::net::TcpStream;
use std::path::{Path, PathBuf};
use std::str;

fn main() {
    println!("\nThis is a filehosting server written in Rust.\n");

    println!("\n Commands: connect | quit \n");

    // loop for connecting to server or quitting, can later be modified for variable ports
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

fn command_loop() {
    match TcpStream::connect("localhost:7878") {
        Ok(stream) => {
            println!(
                "Connected to the server at {}!",
                stream.peer_addr().unwrap()
            );

            let command_list =
                "\n Commands: \n -- upload filepath \n -- download filename \n -- help \n -- quit \n";

            println!("{}", command_list);

            loop {
                let mut user_input = String::new();
                io::stdin()
                    .read_line(&mut user_input)
                    .expect("Error on read");
                let cmd: Vec<&str> = user_input.trim().split_whitespace().collect();

                match cmd[0] {
                    "upload" => {
                        if cmd.len() < 2 {
                            println!("Command needs to be in the form upload filepath");
                            continue;
                        }
                        if let Err(e) = send_file(&stream, cmd[1]) {
                            println!("The file was not able to be sent: {:?}", e);
                        }
                    }
                    "download" => {
                        if cmd.len() < 2 {
                            println!("Command needs to be in the form download filename");
                            continue;
                        }
                        if let Err(e) = receive_file(&stream, cmd[1]) {
                            println!("The file was not able to be downloaded: {:?}", e);
                        }
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

fn send_file(mut stream: &TcpStream, path: &str) -> Result<()> {
    let file = File::open(Path::new(path))?;
    let file_size = file.metadata().unwrap().len();

    let path_names: Vec<&str> = path.split("/").collect();
    let file_name = path_names[path_names.len() - 1];
    let mut command = String::from("upload ");

    //command will have upload filename, not file path for server
    command.push_str(file_name);
    println!("Command: {}", command);

    //file length string ending with \n so server knows when to stop reading
    let mut file_length = file_size.to_string();
    file_length.push_str("\n");

    // sending command to server
    match stream.write(command.as_bytes()) {
        Ok(_) => {
            println!("Upload Command Sent");
            ()
        }
        Err(e) => {
            println!("Error sending upload command to server: {}", e);
        }
    }

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

    // Reading data from file to send to server
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

fn receive_file(mut stream: &TcpStream, file_name: &str) -> Result<()> {
    let mut command = String::from("download ");

    //command will have upload filename
    command.push_str(file_name);
    println!("Command: {}", command);

    // sending command to server
    match stream.write(command.as_bytes()) {
        Ok(_) => {
            println!("Download Command Sent");
            ()
        }
        Err(e) => {
            println!("Error sending upload command to server: {}", e);
        }
    }

    let mut path = PathBuf::from("./client_dir/");

    //push filename to path
    path.push(&file_name);

    //create file
    let mut file = std::fs::File::create(&path).expect("Error creating file");


    //BufReader to read filesize, filesize_buf to store filesize
    let mut reader = BufReader::new(stream);
    let mut filesize_buf = Vec::new();

    //read file size from server
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

    println!("File downloaded");
    Ok(())
}
