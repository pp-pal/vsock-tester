use std::fs;
use std::process::Command;

fn main() {
    match check_vsock_support() {
        Ok(supported) => {
            if supported {
                println!("Vsock is supported on this system.");
            } else {
                println!("Vsock is not supported on this system.");
            }
        }
        Err(e) => println!("Failed to check vsock support: {}", e),
    }
}

fn check_vsock_support() -> Result<bool, Box<dyn std::error::Error>> {
    // Check if the vhost_vsock module is loaded
    let output = Command::new("lsmod").output()?;
    let lsmod_output = String::from_utf8(output.stdout)?;

    if !lsmod_output.contains("vhost_vsock") {
        return Ok(false);
    }

    // Check if the vsock device exists
    if fs::metadata("/dev/vsock").is_ok() {
        return Ok(true);
    }

    Ok(false)
}
