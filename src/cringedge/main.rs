mod serial;
pub mod cringeTypes;

use std::io::Write;
use std::os::unix::net::UnixListener;
use std::os::unix::prelude::PermissionsExt;
use std::process;
use std::sync::mpsc;
use std::thread;

use std::fs;
use log::{debug, info, warn};

use crate::cringeTypes::{CringeEvt, CRINGED_TMP_PATH};


fn main() {
    env_logger::init();

    let (tx, rx)
        : (mpsc::Sender<CringeEvt>, mpsc::Receiver<CringeEvt>) = mpsc::channel();

    let socket_filename = format!("{}/events.sock", CRINGED_TMP_PATH);

    thread::spawn(move || {
        fs::create_dir(CRINGED_TMP_PATH).ok();
        fs::remove_file(&socket_filename).ok();

        let listener = match UnixListener::bind(
            format!("{}/events.sock", CRINGED_TMP_PATH)) {
            Ok(sock) => sock,
            Err(e) => {
                warn!("Couldn't connect: {e:?}");
                process::exit(1);
            }
        };

        match fs::metadata(&socket_filename){
            Ok(p) => {
                let mut perms = p.permissions();
                perms.set_mode(0o777);
                fs::set_permissions(&socket_filename, perms).ok();
            }
            Err(_) => {
                warn!("Couldn't set permissions for socket file");
                return
            }
        };
        loop {
            match listener.accept() {
                Ok((mut socket, addr)) => {
                    debug!("Got a client: {:?} - {:?}", socket, addr);

                    let hello_evt = CringeEvt {
                        io_bank_num: 0,
                        event_type: cringeTypes::EvtType::BoardBoot,
                        timestamp_ms: 0
                    };

                    let hello_evt_u8 = [
                        serde_json::to_string(&hello_evt).unwrap().as_bytes(),
                        "\n".as_bytes()
                    ].concat();
                    
                    socket.write_all(&hello_evt_u8).ok();

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
