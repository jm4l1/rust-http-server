# Rust Http Server

### Starting the server

```rust
cargo run
   Compiling rust_http_server v0.1.0 (rust-http-server)
    Finished dev [unoptimized + debuginfo] target(s) in 0.77s
     Running `target/debug/rust_http_server`
Server Started on 50000
```

### Testing connections connection to the server

In another terminal run the following.

```bash
curl localhost:50000
```

Server output

```
...
connection received from: 127.0.0.1:63025
```
