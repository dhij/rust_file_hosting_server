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
        // buffer for reading stdin
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("Error on read");
        // match on input read from stdin
        match line[..].trim() {
            "connect" => {
                // start user loop after connecting
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

// function to handle user interaction loop after connecting to server
fn command_loop() {
    let mut authenticated_user = false;

    // try connecting to server address
    match TcpStream::connect("localhost:7878") {
        Ok(stream) => {
            println!(
                "Connected to the server at {}!",
                stream.peer_addr().unwrap()
            );

            loop {
                // user needs to be authenticated to access commands
                if authenticated_user {
                    // list of possible commands for authenticated users
                    let command_list =
                "\n Commands: \n -- upload (-p) <file_path> \n -- download (-p) <file_name> \n -- search (-p, -x) <file_name or file_extension> \n -- makePrivate <file_name> \n -- makePublic <file_name> \n -- help \n -- quit \n";

                    loop {
                        println!("{}", command_list);

                        let mut user_input = String::new();
                        io::stdin()
                            .read_line(&mut user_input)
                            .expect("Error on read");
                        // split command from input into tokens
                        let cmd: Vec<&str> = user_input.trim().split_whitespace().collect();

                        // first token should be the actual command
                        match cmd[0] {
                            "upload" => {
                                let publicFlag: bool = cmd.contains(&"-p");
                                let mut filePath= "";
                                if publicFlag{
                                    filePath = cmd[2];
                                }
                                else {
                                    filePath = cmd[1];
                                }
                                if cmd.len() < 2 {
                                    println!("Command needs to be in the form: upload <file_path>");
                                    println!("\nPlease try again: ");
                                    continue;
                                }
                                else if  cmd.len() < 3 && publicFlag {
                                    println!("Command needs to be in the form: upload (-p) <file_path>");
                                    println!("\nPlease try again: ");
                                    continue;
                                }
                                if let Err(e) = send_file(&stream, filePath, publicFlag) {
                                    println!("Upload failed: {:?}", e);
                                    println!("\nPlease try again: ");
                                }
                            }
                            "download" => {
                                let public_option = cmd.contains(&"-p");
                                if cmd.len() < 2 {
                                    println!(
                                        "Command needs to be in the form: download (-p) <file_name>"
                                    );
                                    continue;
                                }
                                else if public_option && cmd.len() < 3 {
                                    println!(
                                        "Command needs to be in the form: download (-p) <file_name>"
                                    );
                                    continue;
                                }
                                if let Err(e) = receive_file(&stream, &cmd) {
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
                                else if ((public_option && !ext_option) || (!public_option && ext_option)) && cmd.len() < 3 {
                                    println!(
                                        "Command needs to be in the form: search (-p, -x) <file_name or file_extension>"
                                    );
                                    continue;
                                }
                                if let Err(e) = search(&stream, &cmd) {
                                    println!("Search failed: {:?}", e);
                                }
                            }
                            "makePublic" => {
                                if cmd.len() < 2 {
                                    println!(
                                        "Command needs to be in the form: makePublic <file_name>"
                                    );
                                    continue;
                                }
                                if let Err(e) = makePublic(&stream, cmd[1]){
                                    println!("Make public failed: {:?}", e);
                                }
                            }
                            "makePrivate" => {
                                if cmd.len() < 2 {
                                    println!(
                                        "Command needs to be in the form: makePrivate <file_name>"
                                    );
                                    continue;
                                }
                                if let Err(e) = makePrivate(&stream, cmd[1]){
                                    println!("Make private failed: {:?}", e);
                                }
                            }
                            "help" => {
                                println!("{}", command_list);
                            }
                            "quit" => {
                                authenticated_user = false;
                                println!("Logged Out");
                                break;
                            }
                            _ => {
                                print!("Please enter a valid command.");
                            }
                        }
                    }
                } else {
                    // commands for unauthenticated user
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
                            if cmd.len() != 2 {
                                println!("Command needs to be in the form: login <username>");
                                println!("\nPlease try again: ");
                                continue;
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

                            if cmd.len() != 2 {
                                println!("Command needs to be in the form: create <username>");
                                println!("\nPlease try again: ");
                                continue;
                            }
                            print!("Enter a new password: \n");
                            let mut password_input = String::new();
                            io::stdin()
                                .read_line(&mut password_input)
                                .expect("Error on reading the password");
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

fn makePublic(mut stream: &TcpStream, filename: &str) -> Result<()> {

    let data = format!("makePublic {}", filename)
        .as_bytes()
        .to_vec();

    match stream.write(&data){
        Ok(_) => {
            println!("File change information sent");
            ()
        }
        Err(e) => {
            println!("Error sending file change information to server: {}", e);
        }
    }

    Ok(())
}

fn makePrivate(mut stream: &TcpStream, filename: &str) -> Result<()> {

    let data = format!("makePrivate {}", filename)
        .as_bytes()
        .to_vec();

    match stream.write(&data){
        Ok(_) => {
            println!("File change information sent");
            ()
        }
        Err(e) => {
            println!("Error sending file change information to server: {}", e);
        }
    }

    Ok(())
}

// function to handle the login operation
fn login(mut stream: &TcpStream, username: &str, password: &str) -> bool {

    let mut server_result: bool = false;

    // format the information needed to be sent to the server
    let data = format!("login {} {}", username, password)
        .as_bytes()
        .to_vec();

    // send formatted command to server through tcpstream
    match stream.write(&data) {
        Ok(_) => {
            println!("Login information sent");
            ()
        }
        Err(e) => {
            println!("Error sending login information to server: {}", e);
        }
    }

    //BufReader to read response, login_result to store it 
    let mut reader = BufReader::new(stream);
    let mut login_result = Vec::new();

    //read until \n byte from server
    match reader.read_until(b'\n', &mut login_result) {
        Ok(_) => (),
        Err(e) => {
            println!("Error reading file size: {}", e);
        }
    }

    login_result.pop(); // pop the \n

    let login_res = str::from_utf8(&login_result).unwrap();

    println!("{}", login_res);

    if login_res == "Login Successful" {
        server_result = true;
    }

    server_result

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

    //BufReader to read response, create_result to store it 
    let mut reader = BufReader::new(stream);
    let mut create_result = Vec::new();

    //read until \n byte from server
    match reader.read_until(b'\n', &mut create_result) {
        Ok(_) => (),
        Err(e) => {
            println!("Error reading file size: {}", e);
        }
    }

    create_result.pop(); // pop the \n

    let create_res = str::from_utf8(&create_result).unwrap();

    println!("{}", create_res);

    Ok(())
}

// function to handle the search operation for client
fn search(mut stream: &TcpStream, command: &Vec<&str>) -> Result<()> {
    let mut data = String::new();
    
    // push all necessary info to string
    for cmd in command {
        data.push_str(cmd);
        data.push_str(" ");
    }

    // send necessary info to server through stream
    match stream.write(&data.as_bytes().to_vec()) {
        Ok(_) => {
            println!("Search request sent");
            ()
        }
        Err(e) => {
            println!("Error sending search request to server: {}", e);
        }
    }

    //BufReader to read from steam, vector to store file names matching search criteria 
    let mut reader = BufReader::new(stream);
    let mut files = Vec::new();

    //read until \n character into buffer
    match reader.read_until(b'\n', &mut files) {
        Ok(_) => (),
        Err(e) => {
            println!("Error reading file size: {}", e);
        }
    }
    // pop trailing \n
    files.pop();

    let files = std::str::from_utf8(&files).unwrap(); 

    let file_names: Vec<&str> = files.trim().split_whitespace().collect();
    // print results of search function
    if file_names.len() == 0 {
        println!("No files matching that input were found.\n");
    }
    else {
        let mut result = String::from("Matching files: ");
        for i in 0..file_names.len() {
            result.push_str(&file_names[i]);
            if i != file_names.len() -1 {
                result.push_str(", ");
            }
            else {
                result.push_str("\n");
            }
        }
        println!("{}", result);
    }

    Ok(())
}

fn send_file(mut stream: &TcpStream, path: &str, publicFlag: bool) -> Result<()> {
    // file to sent
    let file = File::open(Path::new(path))?;
    // store file size from metadata
    let file_size = file.metadata().unwrap().len();
    // collect all tokens in path
    let path_names: Vec<&str> = path.split("/").collect();
    // file name is last token
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

    let mut data:Vec<u8> = Vec::new();
    if publicFlag {
        data = format!("upload -p {} {} ", file_size, file_name)
            .as_bytes()
            .to_vec();
        data.extend(&buffer);
    }
    else {
        data = format!("upload {} {} ", file_size, file_name)
            .as_bytes()
            .to_vec();
        data.extend(&buffer);
    }

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

fn receive_file(mut stream: &TcpStream, command: &Vec<&str>) -> Result<()> {
    let mut data = String::new();
    
    for cmd in command {
        data.push_str(cmd);
        data.push_str(" ");
    }

    println!("Command: {}", &data);

    // sending command to server
    match stream.write(&data.as_bytes().to_vec()) {
        Ok(_) => {
            println!("Download Command Sent");
            ()
        }
        Err(e) => {
            println!("Error sending upload command to server: {}", e);
        }
    }

    let mut path = PathBuf::from("../client_dir/");

    //push filename to path
    if command[1] == "-p" {
        path.push(command[2]);
    }
    else {
        path.push(command[1]);
    }

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

    if filesize_buf.len() == 0 {
        println!("No matching file found.");
        return Ok(());
    }

    //create buffer for file data
    let filesize;  
    match str::from_utf8(&filesize_buf).unwrap().parse::<usize>() {
        Ok(file_data) => {
            filesize = file_data;
        }
        Err(e) => {
            println!("Error parsing file: {}", e);
            return Ok(());
        }
    }

    let mut file_data = vec![0; filesize as usize];

    //read data from user, put data into fileData vec
    match reader.read_exact(&mut file_data) {
        Ok(_) => (),
        Err(e) => {
            println!("Error reading file data: {}", e);
        }
    }

    //create file
    let mut file = std::fs::File::create(&path).expect("Error creating file");

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
