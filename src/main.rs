extern crate vsock;
use std::io::{Read, Write};
use std::thread;
use vsock::{VsockAddr, VsockListener, VsockStream};

fn handle_client(mut stream: VsockStream) {
    let mut buffer = [0; 512];
    match stream.read(&mut buffer) {
        Ok(_) => {
            println!("Received: {:?}", String::from_utf8_lossy(&buffer));
            stream.write_all(b"Hello from server!").unwrap();
        }
        Err(e) => println!("Failed to read from connection: {}", e),
    }
}

fn main() -> std::io::Result<()> {
    let addr = VsockAddr::new(2, 1234);
    let listener = VsockListener::bind(&addr)?; // CID 2 is typically the host
    println!("Server listening on vsock:2:1234");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection!");
                thread::spawn(move || handle_client(stream));
            }
            Err(e) => println!("Connection failed: {}", e),
        }
    }
    Ok(())
}
