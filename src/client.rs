use std::fs::{File};
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
    let mut quit: bool = false;
    let mut authenticated_user = false;

    match TcpStream::connect("localhost:7878") {
        Ok(stream) => {
            println!(
                "Connected to the server at {}!",
                stream.peer_addr().unwrap()
            );

            loop {
                if authenticated_user {
                    if quit == true {
                        break;
                    }
                    let command_list =
                "\n Commands: \n -- upload <file_path> \n -- download <file_name> \n -- search (-p, -x) <file_name or file_extension> \n -- help \n -- quit \n";

                    loop {
                        println!("{}", command_list);

                        let mut user_input = String::new();
                        io::stdin()
                            .read_line(&mut user_input)
                            .expect("Error on read");
                        let cmd: Vec<&str> = user_input.trim().split_whitespace().collect();

                        match cmd[0] {
                            "upload" => {
                                if cmd.len() < 2 {
                                    println!("Command needs to be in the form: upload <file_path>");
                                    println!("\nPlease try again: ");
                                    continue;
                                }
                                if let Err(e) = send_file(&stream, cmd[1]) {
                                    println!("Upload failed: {:?}", e);
                                    println!("\nPlease try again: ");
                                }
                            }
                            "download" => {
                                if cmd.len() < 2 {
                                    println!(
                                        "Command needs to be in the form: download <file_name>"
                                    );
                                    continue;
                                }
                                if let Err(e) = receive_file(&stream, cmd[1]) {
                                    println!("Download failed: {:?}", e);
                                }
                            }
                            "search" => {
                                let public_option = cmd.contains(&"-p");
                                let ext_option = cmd.contains(&"-x");
                                if cmd.len() < 2 {
                                    println!(
                                        "Command needs to be in the form: search (-p, -x) <file_name or file_extension>"
                                    );
                                    continue;
                                }
                                else if public_option && ext_option && cmd.len() < 4 {
                                    println!(
                                        "Command needs to be in the form: search (-p, -x) <file_name or file_extension>"
                                    );
                                    continue;
                                }
                                else if (public_option && !ext_option) || (!public_option && ext_option) && cmd.len() < 3 {
                                    println!(
                                        "Command needs to be in the form: search (-p, -x) <file_name or file_extension>"
                                    );
                                    continue;
                                }
                                if let Err(e) = search(&stream, &cmd) {
                                    println!("Search failed: {:?}", e);
                                }
                            }
                            "help" => {
                                println!("{}", command_list);
                            }
                            "quit" => {
                                quit = true;
                                break;
                            }
                            _ => {
                                print!("Please enter a valid command.");
                            }
                        }
                    }
                } else {
                    // not authenticated
                    let command_list =
                "\n Commands: \n -- login <username> \n -- create <username> \n -- help \n -- quit \n";
                    println!("{}", command_list);

                    let mut user_input = String::new();
                    io::stdin()
                        .read_line(&mut user_input)
                        .expect("Error on read");
                    let cmd: Vec<&str> = user_input.trim().split_whitespace().collect();
                    match cmd[0] {
                        "login" => {
                            if cmd.len() < 2 {
                                println!("Command needs to be in the form: login <username>");
                                println!("\nPlease try again: ");
                            }

                            println!("Enter your password:");
                            let mut password_input = String::new();
                            io::stdin()
                                .read_line(&mut password_input)
                                .expect("Error on reading the password");

                            let result = login(&stream, cmd[1], &password_input[..]);

                            if !result {
                                println!("Login failed:");
                                println!("\nPlease try again: ");
                            } else {
                                authenticated_user = true;
                            }

                            // login successful
                            //authenticated_user = true;
                            continue;
                        }
                        "create" => {
                            // prompt the user for the password
                            print!("Enter a new password: \n");
                            let mut password_input = String::new();
                            io::stdin()
                                .read_line(&mut password_input)
                                .expect("Error on reading the password");

                            if cmd.len() < 2 {
                                println!("Command needs to be in the form: create <username>");
                                println!("\nPlease try again: ");
                            }
                            if let Err(e) = create_user(&stream, cmd[1], &password_input[..]) {
                                println!("Creating user failed: {:?}", e);
                                println!("\nPlease try again: ");
                            }
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
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    println!("Connection terminated.");
}

fn login(mut stream: &TcpStream, username: &str, password: &str) -> bool {

    let mut serverResult: bool = false;

    let data = format!("login {} {}", username, password)
        .as_bytes()
        .to_vec();

    match stream.write(&data) {
        Ok(_) => {
            println!("Login information sent");
            ()
        }
        Err(e) => {
            println!("Error sending login information to server: {}", e);
        }
    }

    //BufReader to read filesize, filesize_buf to store filesize
    let mut reader = BufReader::new(stream);
    let mut loginResult = Vec::new();

    //read file size from server
    match reader.read_until(b'\n', &mut loginResult) {
        Ok(_) => (),
        Err(e) => {
            println!("Error reading file size: {}", e);
        }
    }

    loginResult.pop(); // pop the \n

    let loginRes = str::from_utf8(&loginResult).unwrap();

    println!("{}", loginRes);

    if loginRes == "Login Successful" {
        serverResult = true;
    }

    serverResult

}

fn create_user(mut stream: &TcpStream, username: &str, password: &str) -> Result<()> {
    let data = format!("create {} {}", username, password)
        .as_bytes()
        .to_vec();

    match stream.write(&data) {
        Ok(_) => {
            println!("New user information sent");
            ()
        }
        Err(e) => {
            println!("Error sending user information to server: {}", e);
        }
    }

    Ok(())
}

fn search(mut stream: &TcpStream, command: &Vec<&str>) -> Result<()> {
    let mut data = String::new();
    
    for cmd in command {
        data.push_str(cmd);
        data.push_str(" ");
    }

    match stream.write(&data.as_bytes().to_vec()) {
        Ok(_) => {
            println!("Search request sent");
            ()
        }
        Err(e) => {
            println!("Error sending search request to server: {}", e);
        }
    }

    Ok(())
}

fn send_file(mut stream: &TcpStream, path: &str) -> Result<()> {
    let file = File::open(Path::new(path))?;
    let file_size = file.metadata().unwrap().len();
    let path_names: Vec<&str> = path.split("/").collect();
    let file_name = path_names[path_names.len() - 1];

    // Reading data from file to send to server
    let mut buffer = vec![0; file_size as usize];
    match File::open(&path) {
        Ok(mut file) => {
            match file.read(&mut buffer) {
                Ok(_) => (),
                Err(e) => {
                    println!("Error reading file to buffer: {}", e);
                }
            };
        }
        Err(e) => {
            println!("Error opening file from path: {}", e);
        }
    };

    let mut data = format!("upload {} {} ", file_size, file_name)
        .as_bytes()
        .to_vec();
    data.extend(&buffer);

    // sending file data
    match stream.write(&data) {
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

    //command will have downloaded filename
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
