use std::error::Error;
use std::io::{ErrorKind, Read, Write};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream};
use std::sync::{mpsc, Arc, Mutex};
use std::thread::JoinHandle;
use std::time::Duration;

use super::thread_pool::ThreadPool;

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

fn handle_connection(address: SocketAddr, mut stream: TcpStream) {
    println!("Connection received from: {address:?}");
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    println!("{:?}", buffer)
}
