extern crate bcrypt;

use bcrypt::{hash, verify, DEFAULT_COST};
use std::fs;
use std::fs::{File, read_dir};
use std::io::{BufRead, BufReader, Read, Result, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::str;
use std::thread;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = vec![0; 4096];
    let mut current_user: String = String::from("");

    loop {
        match stream.read(&mut buffer) {
            Ok(size) => {
                let command = String::from_utf8_lossy(&buffer[0..size]);
                let words: Vec<&str> = command.trim().split_whitespace().collect();

                if words[0] == "upload" {
                    //get file path
                    //let mut path = PathBuf::from("./server_publicFiles/");
                    let mut path = PathBuf::from(format!("./server_privateFiles/{}", current_user));

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
                } else if words[0] == "download" {
                    if let Err(e) = send_file(&stream, words[1]) {
                        println!("The file was not able to be downloaded: {:?}", e);
                    }
                } else if words[0] == "search" {
                    if let Err(e) = search(&stream, &words) {
                        println!("Search Unsucessful: {:?}", e);
                    }
                } else if words[0] == "login" {
                    if let Err(e) = login(&stream, &words[1], &words[2]) {
                        println!("Login Unsuccessful: {:?}", e);
                    }

                    current_user = words[1].to_string();
                } else if words[0] == "create" {
                    // hash the password
                    let hashed_password = hash(&words[2], DEFAULT_COST).unwrap();
                    let user_input = words[1];
                    let user_info = format!("{}={}", &user_input, hashed_password);

                    // read list of existing users from users.txt
                    let f = File::open("./users/users.txt").expect("Unable to open file");
                    let f = BufReader::new(f);

                    let mut existing_users: Vec<String> = Vec::new();

                    // store the list of existing users (username=password) in a vector
                    for line in f.lines() {
                        let line = line.expect("Unable to read line");
                        existing_users.push(line);
                    }

                    // loop through the list of existing users and check if username exists
                    for user in existing_users.clone().into_iter() {
                        // split "username=password" into username and password
                        let user_info: Vec<&str> = user[..].split("=").collect();

                        // if username already exists
                        if user_info[0] == user_input {
                            // TODO: throw error to the client
                            eprintln!("Username exists. Please try again.");
                            return;
                        }
                    }

                    // if the username does not already exist, append the new user_info to the existing user_info
                    existing_users.push(user_info.clone());

                    // join the list of existing users into a string separated by carriage returns
                    let joined_users = existing_users.join("\n");

                    // write new list of users to the users.txt file
                    let path = Path::new("./users/users.txt");
                    let mut file = File::create(&path).expect("Error opening file");
                    file.write_all(joined_users[..].as_bytes())
                        .expect("Unable to write data");

                    let new_directory_path = format!("./server_privateFiles/{}", &words[1]);

                    // create a new user directory under server_publicFiles
                    fs::create_dir_all(new_directory_path).unwrap();
                }
            } // ok
            Err(_) => {
                println!(
                    "An error occurred, terminating connection with {}",
                    stream.peer_addr().unwrap()
                );
                break;
            } // err
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

fn login(mut stream: &TcpStream, givenUsername: &str, givenPassword: &str) -> Result<()> {
    println!(
        "Login - Username: {}, Password: {}",
        givenUsername, givenPassword
    );
    let mut loginResult = String::from("Hello\n");
    // check if username exists
    let mut usernameFound = false;

    //read from existing users file
    let f = File::open("./users/users.txt").expect("Unable to open file");
    let f = BufReader::new(f);

    let mut existing_users: Vec<String> = Vec::new();

    //store users in vec
    for line in f.lines() {
        let line = line.expect("Unable to read line");
        existing_users.push(line);
    }

    for username in existing_users.clone().into_iter() {
        let user_info: Vec<&str> = username[..].split("=").collect();

        if user_info[0] == givenUsername {
            usernameFound = true;
            println!("Username matched");

            // if it exists, check the text file and compare the hashpassword
            match verify(givenPassword, user_info[1]) {
                Ok(boo) => {
                    if boo {
                        println!("Login Successful");
                        loginResult = String::from("Login Successful\n");
                    } else {
                        println!("Password Incorrect");
                        loginResult = String::from("Password Incorrect\n");
                    }
                }
                Err(e) => println!("{}", e),
            }
        }
    }

    // if username is not found, send error message to user
    if !usernameFound {
        println!("Username not found");
        loginResult = String::from("Username not found\n");
    }

    match stream.write(&loginResult.as_bytes()) {
        Ok(_) => {
            println!("Login result sent");
            ()
        }
        Err(e) => {
            println!("Error sending file size to server: {}", e);
        }
    }

    Ok(())
}

fn search(mut stream: &TcpStream, command: &Vec<&str>) -> Result<()> {
    // if searching in public folder
    let public_option = command.contains(&"-p");
    // if searching only extensions
    let ext_option = command.contains(&"-x");
    // vector of file names matching given name
    let mut files_in_dir: Vec<String> = Vec::new();
    // String which will be sent as server response
    let mut data = String::new();
    let path: PathBuf;
    // set path based on if searching public files or not
    if public_option {
        path = PathBuf::from("./server_publicFiles/");
    }
    else {
        path = PathBuf::from(format!("./server_privateFiles/pranay/"));
    }
    match read_dir(Path::new(&path)) {
        Ok(dir_files) => {
            for entry in dir_files {
                let entry = entry?;
                let file_path = entry.path();
                if !file_path.is_dir() {
                    if let Ok(name) = entry.file_name().into_string() {
                        if ext_option {
                            if let Some(ext) = file_path.extension() {
                                if let Some(last_elem) = command.last() {
                                    if ext.to_str() == Some(last_elem) {
                                        files_in_dir.push(name.clone());
                                    }
                                }
                            }
                        }
                        else {
                                if let Some(last_elem) = command.last() {
                                    if name.contains(last_elem) {
                                        files_in_dir.push(name.clone());
                                    }
                            }
                        }
                    }
                }
            }
            for file_name in files_in_dir {
                data.push_str(&file_name);
                data.push_str(" ");
            }
        }
        Err(e) => {
            println!("Error searching for file: {}", e);
        }
    }
    data.push_str("\n");
    match stream.write(&data.as_bytes().to_vec()) {
        Ok(_) => {
            println!("Search results sent");
            ()
        }
        Err(e) => {
            println!("Error sending search request to client: {}", e);
        }
    }
    Ok(())
}

fn send_file(mut stream: &TcpStream, file_name: &str) -> Result<()> {
    // currently downloads files only from server_publicFiles folder
    let mut path = PathBuf::from("./server_publicFiles/");
    path.push(file_name);

    let file = File::open(Path::new(&path)).expect("Error opening file");
    let file_size = file.metadata().unwrap().len();

    // file length string ending with \n so server knows when to stop reading
    let mut file_length = file_size.to_string();
    file_length.push_str("\n");

    // send file size
    match stream.write(&file_length.as_bytes()) {
        Ok(_) => {
            println!("File size sent");
            ()
        }
        Err(e) => {
            println!("Error sending file size to server: {}", e);
        }
    }

    // read data from file to send to client
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

    // send file data
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
