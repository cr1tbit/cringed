use serialport;
use std::io::{BufReader,BufRead,Read,Write};
use std::os::unix::net::UnixListener;
use futures::io::ErrorKind;

use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::{thread, fmt};

use std::fs;
use chrono::Local;

const CRINGED_TMP_PATH: &str = "/tmp/cringed";

//define enum for button event
enum EvtType {
    ButtonPress,
    ButtonRelease,
    Overcurrent,
    CriticalError,
    TransportError
}

struct CringeEvt {
    io_bank_num: u8,
    event_type: EvtType,
    timestamp_ms: u32
}

impl fmt::Display for CringeEvt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.event_type {
            EvtType::ButtonPress => {
                write!(f, "{}: button {} press",self.io_bank_num, self.timestamp_ms)
            }
            EvtType::ButtonRelease => {
                write!(f, "{}: button {} release",self.io_bank_num, self.timestamp_ms)
            }
            EvtType::Overcurrent => {
                write!(f, "{}: overcurrent", self.timestamp_ms)
            }
            EvtType::CriticalError => {
                write!(f, "{}: critical error", self.timestamp_ms)
            }
            EvtType::TransportError => {
                write!(f, "transport error")
            }
        }
    }
}

fn main() {
    let (tx, rx): (Sender<CringeEvt>, Receiver<CringeEvt>) = mpsc::channel();

    thread::spawn(move || {
        let listener = match UnixListener::bind(format!("{}/events.sock",CRINGED_TMP_PATH)) {
            Ok(sock) => sock,
            Err(e) => {
                println!("Couldn't connect: {e:?}");
                return
            }
        };
        loop {
            match listener.accept() {
                Ok((mut socket, addr)) => {
                    println!("Got a client: {:?} - {:?}", socket, addr);
                    socket.write_all(b"hello world").ok();

                    loop {
                        let event = rx.recv().unwrap();
                        socket.write_all(event.to_string().as_bytes()).ok();
                    }
                },
                Err(e) => println!("accept function failed: {:?}", e),
            }
            println!("conn closed")
        }
    });

    serial_monitor_loop(tx);
}

struct tmpLogFile {
    file: fs::File,
}

impl tmpLogFile {
    fn new(path: &str, dev_name: &str) -> std::io::Result<Self> {
        let file = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&format!("{}/{}.txt",path,dev_name))?;
        Ok(Self { file })
    }

    fn write(&mut self, message: &str){
        write!(self.file, "{}", message).ok();
    }
}

fn serial_monitor_loop(tx: Sender<CringeEvt>){
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

        receive_loop(&ports[0].0, &ports[0].1,tx.clone())
    }
}

fn receive_loop(path: &str, devname: &str, tx: Sender<CringeEvt>){
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

    let device_tmp_path = format!("{}/{}",CRINGED_TMP_PATH,devname);
    fs::create_dir_all(&device_tmp_path).ok();
    let mut logger = tmpLogFile::new(&device_tmp_path, "runtime_log").unwrap();
    logger.write(&format!(
        "start log, date {}", 
        Local::now().format("%Y-%m-%d").to_string())
    );
    
    loop {
        let mut my_str = String::new();
        match reader.read_line(&mut my_str) {
            Ok(_) => {
                match my_str.chars().next() {
                    Some('<') => {
                        print!("command: {}", my_str);
                        match tx.send(CringeEvt{
                            io_bank_num: 0,
                            event_type: EvtType::ButtonPress,
                            timestamp_ms: 0
                        }) {
                            Ok(_) => {}
                            Err(e) => {
                                println!("Error sending event: {}", e);
                                continue;
                            }
                        }
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


    // let mut listener = match UnixListener::bind(format!("/tmp/{}/events.sock",devname)) {
    //     Ok(sock) => sock,
    //     Err(e) => {
    //         println!("Couldn't connect: {e:?}");
    //         return
    //     }
    // };

    // match listener.accept() {
    //     Ok((mut socket, addr)) => {
    //         println!("Got a client: {:?} - {:?}", socket, addr);
    //         socket.write_all(b"hello world").ok();
    //         let mut response = String::new();
    //         socket.read_to_string(&mut response).ok();
    //         println!("{}", response);
    //     },
    //     Err(e) => println!("accept function failed: {:?}", e),
    // }

fn get_compatible_ports() -> Vec<(String, String)> {
    let mut port_list = Vec::new();
    match serialport::available_ports() {
        Ok(ports) => {
            for p in ports {
                match p.port_type {
                    serialport::SerialPortType::UsbPort(info) => {
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