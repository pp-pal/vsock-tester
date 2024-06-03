extern crate vsock;
use std::io::{Read, Write};
use vsock::{VsockAddr, VsockStream};

fn main() -> std::io::Result<()> {
    let addr = VsockAddr::new(2, 1234);
    let mut stream = VsockStream::connect(&addr)?; // CID 2 is typically the host
    println!("Connected to the server!");

    stream.write_all(b"Hello from client!")?;

    let mut buffer = [0; 512];
    stream.read(&mut buffer)?;
    println!("Received: {:?}", String::from_utf8_lossy(&buffer));

    Ok(())
}
