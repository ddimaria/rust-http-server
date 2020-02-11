# Rust HTTP Server

A lightweight, multi-threaded HTTP server using TCP Sockets.

_Not ready for production._

## Installation

Clone the repo and cd into the repo:

```shell
git clone https://github.com/ddimaria/rust-http-server.git
cd rust-http-server
```

Copy over the example .env file:

```shell
cp .env.example .env
```

## Running the Server

To startup the server:

```shell
cargo run
```

To see request and response data in stdout:

```shell
RUST_LOG=info cargo run
```

## Tests

Integration tests are in the `/src/tests` folder.

### Running Tests

To run all of the tests:

```shell
cargo test
```

## Docker

To build a Docker image of the application:

```shell
docker build -t rust_http_server .
```

Now you can run the container in port 8000:

```shell
docker run -it --rm --env-file=.env.docker -p 8000:8000 --name rust_http_server rust_http_server
```
