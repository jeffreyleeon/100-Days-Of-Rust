use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    while let Ok(size) = stream.read(&mut buffer) {
        if size == 0 {
            break;
        }
        stream.write_all(&buffer[..size]).unwrap();
    }
}

fn run_server(address: &str) -> std::io::Result<()> {
    let listener = TcpListener::bind(address)?;
    println!("Server listening on {}", address);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
    Ok(())
}

fn run_client(address: &str) -> std::io::Result<()> {
    let mut stream = TcpStream::connect(address)?;
    println!("Connected to server at {}", address);

    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        if input.trim().is_empty() {
            break;
        }

        stream.write_all(input.as_bytes())?;

        let mut buffer = [0; 1024];
        let size = stream.read(&mut buffer)?;
        println!("Received: {}", String::from_utf8_lossy(&buffer[..size]));
    }
    Ok(())
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <server|client> <address:port>", args[0]);
        std::process::exit(1);
    }

    let mode = &args[1];
    let address = &args[2];

    match mode.as_str() {
        "server" => run_server(address),
        "client" => run_client(address),
        _ => {
            eprintln!("Invalid mode. Use 'server' or 'client'.");
            std::process::exit(1);
        }
    }
}
