# Rust File Server

## Dockerfile Commands
```
cd rust_file_hosting_server

docker build -t rust-file-server .

docker run -it rust-file-server
```
## To Run

Server is listening on: 127.0.0.1:7878 upon running the container
```
cd src

cargo run --bin client
```
## CLI Examples

To connect:
```
Commands: connect | quit

connect
```


### Commands available upon connection:
```
Commands: 
-- login <username> 
-- create <username> 
-- help 
-- quit
```

### Login and Create User

Create a new username & enter a new password when prompted
- Upon successful creation of a username, a private directory is created under the server_privateFiles directory with the given username
```
create user1

Enter a new password:
```

Login & enter a password when prompted
```
login user1

Enter your password:
```

### Commands available once logged in:
```
Commands: 
-- upload (-p) <file_path> 
-- download (-p) <file_name> 
-- search (-p, -x) <file_name or file_extension> 
-- help 
-- quit
```

### Upload

Upload a file from client_dir directory to the private directory in the server under server_privateFiles/\<username\>/
```
upload ../client_dir/test1.txt
```

Upload a file to the public directory server_publicFiles/
```
upload -p ../client_dir/test1.txt
```

### Download

Download a file from the private directory in the server under server_privateFiles/\<username\>/
```
download test1.txt
```

Download a file from the public directory server_publicFiles/
```
download -p serverFile.txt
```

### Search

Search a file by filename or extension from the private directory in the server under server_privateFiles/\<username\>/
```
search test1.txt

search -x txt
```

Search a file by filename or extension from the public directory server_publicFiles/
```
search -p something.pdf

search -p -x pdf
```
