use axum::extract::path;
use serialport::{available_ports, SerialPortType};
use std::io::BufReader;
use std::io::BufRead;
use std::os::unix::net::UnixListener;
use futures::io::ErrorKind;

use std::fs::OpenOptions;
use std::fs::File;
use std::io::Write;
use std::fs;
use chrono::prelude::*;
use std::os::unix::net::UnixStream;


fn main() {
    loop{
        let ports = get_compatible_ports();
        println!("Found {} compatible ports", ports.len());
        for p in &ports {
            println!("{}: {}", p.0, p.1);
        }

        if ports.is_empty() {
            println!("No compatible ports found");
            std::thread::sleep(std::time::Duration::from_secs(1));
            continue;
        }

        receive_loop(&ports[0].0, &ports[0].1)
    }
}

struct tmpLogFile {
    file: File,
}

impl tmpLogFile {
    fn new(path: &str, dev_name: &str) -> std::io::Result<Self> {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&format!("{}/{}.txt",path,dev_name))?;
        Ok(Self { file })
    }

    fn write(&mut self, message: &str){
        write!(self.file, "{}", message).ok();
    }
}

fn receive_loop(path: &str, devname: &str){
    let port_try_open = serialport::new(path, 115200)
        .timeout(std::time::Duration::from_millis(1))
        .open();
    
    let port = match port_try_open {
        Ok(p) => p,
        Err(e) => {
            println!("Error opening serial port: {}", e);
            return;
        }
    };

    let mut reader = BufReader::new(port);

    let device_tmp_path = format!("/tmp/{}",devname);
    fs::create_dir_all(&device_tmp_path).ok();
    let mut logger = tmpLogFile::new(&device_tmp_path, "runtime_log").unwrap();
    logger.write(&format!(
        "start log, date {}", 
        Local::now().format("%Y-%m-%d").to_string())
    );

    // let mut socket = match UnixListener::bind(format!("/tmp/{}/events.sock",devname)) {
    //     Ok(sock) => sock,
    //     Err(e) => {
    //         println!("Couldn't connect: {e:?}");
    //         return
    //     }
    // };

    // socket.write_all(b"<H>").ok();

    loop {
        let mut my_str = String::new();
        match reader.read_line(&mut my_str) {
            Ok(_) => {
                match my_str.chars().next() {
                    Some('<') => {
                        print!("command: {}", my_str);
                    }
                    None => {continue;}
                    _ => {logger.write(&my_str);}
                }
            }
            Err(e) if e.kind() == ErrorKind::TimedOut => {
                //called all the time, safe
                continue;
            }
            Err(e) => {
                println!("Error reading from serial port: {}", e);
                return;
            }
            _ => {}
        }
    }
}



fn get_compatible_ports() -> Vec<(String, String)> {
    let mut port_list = Vec::new();
    match available_ports() {
        Ok(ports) => {
            for p in ports {
                match p.port_type {
                    SerialPortType::UsbPort(info) => {
                        if info.vid == 0x303A && info.pid == 0x1001 {
                            port_list.push(
                                (p.port_name,
                                info.serial_number.unwrap_or("".to_string())
                            ));
                        }
                    }
                    _ => {}
                }
            }
        }
        Err(_e) => {
            return port_list
        }
    }
    port_list
}