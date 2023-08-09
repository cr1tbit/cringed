use serialport;
use std::io::{BufReader,BufRead,Write};
use futures::io::ErrorKind;
use std::fs;
use std::sync::mpsc;
use log::{debug, info, warn};
use chrono::Local;

use crate::cringe_types::{CringeEvt, CRINGED_TMP_PATH};

struct TmpLogFile {
    file: fs::File,
}

impl TmpLogFile {
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

pub(crate) fn serial_monitor_loop(tx: mpsc::Sender<CringeEvt>){
    loop{
        let ports = get_compatible_ports();
        debug!("Found {} compatible ports", ports.len());
        for p in &ports {
            debug!("{}: {}", p.0, p.1);
        }

        if ports.is_empty() {
            debug!("No compatible ports found");
            std::thread::sleep(std::time::Duration::from_secs(1));
            continue;
        }

        receive_loop(&ports[0].0, &ports[0].1,tx.clone())
    }
}

fn receive_loop(path: &str, devname: &str, tx: mpsc::Sender<CringeEvt>){
    let port_try_open = serialport::new(path, 115200)
        .timeout(std::time::Duration::from_millis(1))
        .open();
    
    let port = match port_try_open {
        Ok(p) => {
            info!("Opened serial {}", path);
            p
        },
        Err(e) => {
            warn!("Error opening serial port: {}", e);
            return;
        }
    };

    let mut reader = BufReader::new(port);

    let device_tmp_path = format!("{}/{}",CRINGED_TMP_PATH,devname);
    fs::create_dir_all(&device_tmp_path).ok();
    let mut logger = TmpLogFile::new(&device_tmp_path, "runtime_log").unwrap();
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
                        debug!("command: {}", my_str.trim());

                        let _evt = match CringeEvt::from_serial(&my_str) {
                            Some(e) => e,
                            None => {
                                warn!("Error parsing event");
                                continue;
                            }
                        };

                        match tx.send(_evt) {
                            Ok(_) => {}
                            Err(e) => {
                                warn!("Error sending event: {}", e);
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
        }
    }
}


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