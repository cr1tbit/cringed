use std::os::unix::net::UnixStream;

use std::io::{BufReader, BufRead, ErrorKind};
use log::{debug, info, warn};

use std::path::PathBuf;

use clap::{arg, command, value_parser, ArgAction, Command};

const EVENT_SOCKET_PATH: &'static str = "/tmp/cringed/events.sock";

/*

cringed

cringed status

cringed remote 
cringed remote ping
cringed remote 



*/



fn main() {
    env_logger::init();

    let matches = command!() // requires `cargo` feature
        // .arg(arg!([name] "Optional name to operate on"))
        // .arg(
        //     arg!(
        //         -c --config <FILE> "Sets a custom config file"
        //     )
        //     // We don't have syntax yet for optional options, so manually calling `required`
        //     .required(false)
        //     .value_parser(value_parser!(PathBuf)),
        // )
        .arg(arg!(
            -d --debug ... "Turn debugging information on"
        ))
        .subcommand(
            Command::new("status")
                .about("print the status of the daemon and connected devices")
        )
        .get_matches();

    // You can check the value provided by positional arguments, or option arguments
    if let Some(name) = matches.get_one::<String>("name") {
        println!("Value for name: {name}");
    }

    if let Some(config_path) = matches.get_one::<PathBuf>("config") {
        println!("Value for config: {}", config_path.display());
    }

    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences
    match matches
        .get_one::<u8>("debug")
        .expect("Count's are defaulted")
    {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode is on"),
        _ => println!("Don't be crazy"),
    }

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    if let Some(matches) = matches.subcommand_matches("status") {
        println!("Printing status...");
    }

    // peek_events(EVENT_SOCKET_PATH);      
}

fn peek_events(socket_path : &str){
    loop{
        let stream = match UnixStream::connect(socket_path) {
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
        debug!("loop");
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}