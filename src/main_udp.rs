#![allow(non_snake_case)]

mod udp_server;
use log::{
    // info,
    // trace,
    debug,
    // warn,
};
use std::{
    env, 
    time::Duration, 
    sync::{Arc, Mutex},
};
use crate::udp_server::UdpServer;

fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "debug");
    env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let reconnectDelay = Duration::from_secs(3);
    let localAddr = "192.168.120.172:5180";
    let remoteAddr = "192.168.120.173:5180";

    debug!("[main] creating UdpServer...");
    let tcpSrv = Arc::new(Mutex::new(
        UdpServer::new(
            localAddr,
            remoteAddr,
            // "127.0.0.1:5180",
            Some(reconnectDelay),
        )
    ));
    debug!("[main] UdpServer created");
    debug!("[main] starting UdpServer...");
    UdpServer::run(tcpSrv);
    debug!("[main] UdpServer started");

    Ok(())
}
