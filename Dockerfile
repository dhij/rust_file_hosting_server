# Use Ubuntu base image, could also start from rust base image
FROM ubuntu:21.10

WORKDIR /home

# Install necessary software
RUN apt-get update
RUN apt-get install -y curl gcc vim
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

ENV PATH="/root/.cargo/bin:${PATH}"

# Copy over rust dependency and source files
COPY src ./src
COPY ./Cargo.toml .
COPY ./Cargo.lock .
COPY client_dir ./client_dir
COPY publicFiles ./publicFiles
COPY ./init.sh ./init.sh

# Compile rust files in debug mode
RUN chmod +x init.sh
RUN cargo build

# Run bash as main process
CMD ["/home/init.sh"]
