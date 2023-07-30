mod serial;
pub mod cringeTypes;

use std::io::Write;
use std::os::unix::net::UnixListener;
use std::sync::mpsc;
use std::thread;

use std::fs;
use log::{debug, info, warn};

use crate::cringeTypes::{CringeEvt, CRINGED_TMP_PATH};


fn main() {
    env_logger::init();

    let (tx, rx)
        : (mpsc::Sender<CringeEvt>, mpsc::Receiver<CringeEvt>) = mpsc::channel();

    thread::spawn(move || {
        fs::remove_file(format!("{}/events.sock",CRINGED_TMP_PATH)).ok();

        let listener = match UnixListener::bind(
            format!("{}/events.sock", CRINGED_TMP_PATH)) {
            Ok(sock) => sock,
            Err(e) => {
                warn!("Couldn't connect: {e:?}");
                return
            }
        };
        loop {
            match listener.accept() {
                Ok((mut socket, addr)) => {
                    debug!("Got a client: {:?} - {:?}", socket, addr);
                    socket.write_all(b"hello world").ok();

                    loop {
                        let event = rx.recv().unwrap();
                        let msg_u8 = [
                            serde_json::to_string(&event).unwrap().as_bytes(),
                            "\n".as_bytes()
                        ].concat();

                        match socket.write_all(&msg_u8){
                            Ok(_) => {}
                            Err(e) => {
                                warn!("Error sending event: {}", e);
                                break;
                            }
                        };
                    }
                },
                Err(e) => warn!("accept function failed: {:?}", e),
            }
            debug!("conn closed")
        }
    });
    serial::serial_monitor_loop(tx);
}
