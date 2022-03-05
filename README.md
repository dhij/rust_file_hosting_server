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

To Connect:
```
Commands: connect | quit

connect
```

To Upload a File from the client_dir Directory
```
Commands: 
-- upload <file_path> 
-- download <file_name> 
-- help 
-- quit 
 
upload ../client_dir/test1.txt
```

To Download a File from publicFiles Directory
```
Commands: 
-- upload <file_path> 
-- download <file_name> 
-- help 
-- quit 

download serverFile.txt
```
