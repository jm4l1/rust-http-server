use rust_http_server::http::Server;

fn main() {
    let mut server = Server::new(50000);
    let result = server.start();
    match result {
        Ok(_) => (),
        Err(e) => println!("{:?}", e),
    }
}
