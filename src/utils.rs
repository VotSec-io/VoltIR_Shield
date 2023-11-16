use std::io::BufRead;
use std::net::{IpAddr, SocketAddr};
use std::process::Command;
use std::sync::Mutex;
use std::thread;

fn read_to_string(path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
