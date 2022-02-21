# Rust_File_Hosting_Server

## Dockerfile

Build and store the docker image:
docker build -t "image-name" .

Run container based off of image and enter an interactive shell in container:
docker run -it image-name

## To Run

Run server: cargo run --bin server

Run client: cargo run --bin client

## Semantic Commit messages

- feat: :zap: new feature
- fix: :bug: fix bug
- refactor: :hammer: refactor code
