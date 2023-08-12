extern crate bincode;

use chrono::{DateTime, Utc};
use std::error::Error;
use std::io::{ErrorKind, Read, Write};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream};
use std::sync::{mpsc, Arc, Mutex};
use std::thread::JoinHandle;
use std::time::Duration;

use crate::file_handler;
use crate::types::{HttpVersion, Method, Request, Response, ResponseCode};

use super::thread_pool::ThreadPool;

const METHOD_NOT_IMPLEMENTED_STR : &'static str = "<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"UTF-8\"><meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\"><title>Not Implemented</title></head><body>Method Not Implemented</body></html>";
const METHOD_NOT_FOUND_STR : &'static str = "<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"UTF-8\"><meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\"><title>Not Found</title></head><body>Method Not Found</body></html>";
pub struct Server {
    port: u16,
    listen_thread: Option<JoinHandle<()>>,
    cli_thread: Option<JoinHandle<()>>,
    state: State,
}
enum State {
    Running,
    Terminated,
}
impl Server {
    pub fn new(port: u16) -> Server {
        Server {
            port,
            listen_thread: None,
            cli_thread: None,
            state: State::Running,
        }
    }

    pub fn start(&mut self) -> Result<(), Box<dyn Error>> {
        let socket_address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), self.port);
        let bind_result = TcpListener::bind(socket_address);

        match bind_result {
            Ok(listener) => Ok(self.start_threads(listener)),
            Err(e) => Err(Box::new(e)),
        }
    }

    fn start_threads(&mut self, listener: TcpListener) {
        println!("Starting server on port {:?}", self.port);
        let port = self.port.clone();

        let (main_sender, main_receiver) = mpsc::channel();
        let main_receiver = Arc::new(Mutex::new(main_receiver));

        let clone1 = Arc::clone(&main_receiver);
        let listen_thread = std::thread::spawn(move || do_listen_work(port, &listener, clone1));

        self.listen_thread = Some(listen_thread);

        let (cli_sender, cli_receiver) = mpsc::channel();
        let clone1 = Arc::clone(&main_receiver);
        let cli_thread = std::thread::spawn(move || do_cli_work(cli_sender, clone1));
        self.cli_thread = Some(cli_thread);

        loop {
            let state = cli_receiver.recv().unwrap();

            match state {
                State::Running => {
                    continue;
                }
                State::Terminated => {
                    self.state = State::Terminated;
                    main_sender.send(State::Terminated).unwrap();
                    println!("Sever has been terminated");
                    break;
                }
            }
        }

        self.listen_thread.take().unwrap().join().unwrap();
        self.cli_thread.take().unwrap().join().unwrap();
    }
}

fn log(request: &Request, response: &Response) {
    let status_code: ResponseCode = response.get_status_code();
    let formatted_status_code = match status_code.clone() as usize {
        0..=199 => format!("\x1b[1;34m{}\x1b[0m", status_code.clone() as usize),
        200..=299 => format!("\x1b[1;32m{}\x1b[0m", status_code.clone() as usize),
        300..=399 => format!("\x1b[1;33m{}\x1b[0m", status_code.clone() as usize),
        _ => format!("\x1b[1;31m{}\x1b[0m", status_code as usize),
    };
    println!(
        "[{}] {} {}",
        response.get_header("Date"),
        request.get_request_line(),
        formatted_status_code
    );
}
fn do_listen_work(
    port: u16,
    listener: &TcpListener,
    state_receiver: Arc<Mutex<mpsc::Receiver<State>>>,
) {
    listener.set_nonblocking(true).unwrap();
    let thread_pool = ThreadPool::new(8);
    println!("Server Started on {:?}", port);
    loop {
        match state_receiver
            .lock()
            .unwrap()
            .recv_timeout(Duration::from_millis(100))
        {
            Ok(state) => match state {
                State::Terminated => {
                    println!("Terminating connection worker");
                    break;
                }
                _ => (),
            },
            _ => (),
        }
        let stream = listener.accept();
        match stream {
            Ok((stream, address)) => {
                thread_pool.execute(move || handle_connection(address, stream));
            }

            Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => println!("Error getting client {:?}", e),
        };
    }
}
fn do_cli_work(sender: mpsc::Sender<State>, state_receiver: Arc<Mutex<mpsc::Receiver<State>>>) {
    println!("Starting cli worker");
    loop {
        let mut input = String::from("");
        match state_receiver
            .lock()
            .unwrap()
            .recv_timeout(Duration::from_millis(100))
        {
            Ok(state) => match state {
                State::Terminated => {
                    println!("Terminating Cli worker");
                    break;
                }
                _ => (),
            },
            _ => (),
        };
        print!("> ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut input).unwrap();
        if input.to_lowercase().starts_with("exit") {
            sender.send(State::Terminated).unwrap();
            println!("Terminating Cli worker");
            break;
        }
        input.clear();
    }
}
fn handle_get_request(request: &Request) -> Response {
    let resource = format!(
        "www/{}",
        if &request.request_line.resource == "/" {
            "index.html"
        } else {
            &request.request_line.resource
        }
    );
    let resource_data = file_handler::read_file(std::path::Path::new(&resource));
    let mut response = Response::new(
        request.request_line.version.clone(),
        ResponseCode::Ok,
        Vec::<u8>::new(),
    );
    match resource_data {
        Some(data) => response.body(data),
        None => {
            response.status_code(ResponseCode::NotFound);
            response.body(format!("{}", METHOD_NOT_FOUND_STR).into_bytes())
        }
    }
    response.add_header("Content-type", "text/html");
    response.add_header(
        "Content-length",
        format!("{}", response.body_length()).as_str(),
    );
    response
}
fn handle_post_request(_request: &Request) -> Response {
    Response::new(HttpVersion::HttpV1_1, ResponseCode::Ok, vec![])
}
fn handle_request_string(buffer_string: String) -> Response {
    let request = Request::parse_from_string(&buffer_string.to_string());
    let mut response = match &request {
        Some(valid_request) => {
            let method = &valid_request.request_line.method;
            match method {
                Method::Get => handle_get_request(valid_request),
                Method::Post => handle_post_request(valid_request),
                _ => {
                    let mut response = Response::new(
                        valid_request.request_line.version.clone(),
                        ResponseCode::NotImplemented,
                        format!("{}", METHOD_NOT_IMPLEMENTED_STR).into_bytes(),
                    );
                    response.add_header("Content-type", "text/html");
                    response.add_header(
                        "Content-length",
                        format!("{}", response.body_length()).as_str(),
                    );
                    response
                }
            }
        }
        None => {
            let response = Response::new(
                HttpVersion::HttpV1_1,
                ResponseCode::BadRequest,
                format!("invalid request").into_bytes(),
            );
            response
        }
    };

    let date_time: DateTime<Utc> = std::time::SystemTime::now().into();
    response.add_header(
        "Date",
        format!("{}", date_time.format("%a, %d %b %Y %H:%M:%S UTC")).as_str(),
    );
    log(&request.unwrap(), &response);
    response
}
fn handle_connection(address: SocketAddr, mut stream: TcpStream) {
    println!("Connection received from: {address:?}");
    let mut buffer = [0; 512];
    match stream.read(&mut buffer) {
        Ok(_) => {
            match std::str::from_utf8(&buffer) {
                Ok(buffer_string) => {
                    let response = handle_request_string(buffer_string.to_string());
                    stream.write(response.as_string().as_bytes()).unwrap();
                }
                Err(_) => {
                    let response = Response::new(
                        HttpVersion::HttpV1_1,
                        ResponseCode::BadRequest,
                        format!("invalid request").into_bytes(),
                    );
                    stream.write(response.as_string().as_bytes()).unwrap();
                }
            };
        }
        Err(error) => eprintln!("{:?}", error),
    };
    stream.shutdown(std::net::Shutdown::Read).unwrap();
}
