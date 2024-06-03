extern crate vsock;
use std::io::Result;
use vsock::{VsockAddr, VsockListener};

fn main() -> Result<()> {
    match check_vsock_support() {
        Ok(_) => println!("Vsock is supported on this system."),
        Err(e) => println!("Vsock is not supported on this system: {}", e),
    }
    Ok(())
}

fn check_vsock_support() -> Result<()> {
    // Attempt to bind a vsock listener to a well-known CID and port
    // If this fails, it indicates that vsock is not supported
    let addr = VsockAddr::new(2, 1234);
    match VsockListener::bind(&addr) {
        Ok(listener) => {
            // Successfully created a listener, drop it immediately
            drop(listener);
            Ok(())
        }
        Err(e) => Err(e),
    }
}
