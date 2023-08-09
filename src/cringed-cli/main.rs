use std::os::unix::net::UnixStream;

use std::io::{BufReader, BufRead, ErrorKind};
use log::{debug, info, warn};

const SOCKET_PATH: &'static str = "/tmp/cringed/events.sock";

fn main() {
    loop{
        debug!("loop");
        std::thread::sleep(std::time::Duration::from_secs(1));
        // Connect to socket
        let stream = match UnixStream::connect(SOCKET_PATH) {
            Err(_) => continue,
            Ok(stream) => stream,
        };
        stream.set_read_timeout(Some(std::time::Duration::from_millis(100))).unwrap();

        let mut reader = BufReader::new(&stream);

        loop {
            let mut my_str = String::new();
            match reader.read_line(&mut my_str) {
                Ok(0) => {
                    info!("Socket closed");
                    break;
                }
                Ok(_) => {
                    my_str = my_str.trim().to_string();
                    match my_str.len() {
                        0 => std::thread::sleep(std::time::Duration::from_millis(50)),
                        _ => println!("{}", my_str)
                    }
                }
                Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                    continue; // no data, but socket alive
                }
                Err(e) => {
                    warn!("Error reading from socket {}",e);
                    break;
                }
            }
        }
    }        
}