use std::sync::Arc;
use once_cell::sync::Lazy;

use serde::Deserialize;
use socketioxide::{adapter::LocalAdapter, Socket};
use tracing::info;

use std::sync::{RwLock, Weak};
use std::collections::HashMap;

pub(crate) static SOCKS: Lazy<RwLock<HashMap<String, Weak<Socket<LocalAdapter>>>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

// events:
// log: debug info about system
// 

pub async fn handler(socket: Arc<Socket<LocalAdapter>>) {
    info!("Socket connected on / with id: {}", socket.sid);
    SOCKS.write().unwrap().insert(socket.sid.to_string(), Arc::downgrade(&socket));

    socket.emit("log", format!("hello there {}",socket.sid)).ok();
    socket.join(["ESP","logs"]);
    socket.to("logs").emit("log", "new challenger approaches").ok();


    socket.on("probe", |socket, _room: Option<String>, _, _| async move {

        socket.emit("log", "probed").ok();
        socket.emit("probe", "testdata").ok();
    });

    

    // socket.on("nickname", |socket, nickname: Nickname, _, _| async move {
    //     let previous = socket.extensions.insert(nickname.clone());
    //     info!("Nickname changed from {:?} to {:?}", &previous, &nickname);
    //     let msg = format!(
    //         "{} changed his nickname to {}",
    //         previous.map(|n| n.0).unwrap_or_default(),
    //         nickname.0
    //     );
    //     socket.to("default").emit("message", msg).ok();
    // });
}