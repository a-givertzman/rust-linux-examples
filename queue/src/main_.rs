#![allow(non_snake_case)]

mod tcp_server;
mod ds_point;

use std::{sync::{Arc, Mutex}, env};

use log::{
    // info,
    // trace,
    debug,
    // warn,
};
use crate::tcp_server::TcpServer;

const QSIZE: usize = 8;
const QSIZE_DOUBLE: usize = QSIZE * 2;
fn main() {
    env::set_var("RUST_LOG", "debug");
    env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    
    let dataBuf = [510_u16; QSIZE];
    let mut sendBuf = [0_u8; QSIZE_DOUBLE];
    let mut j = 0;
    for (i, item) in dataBuf.iter().enumerate() {
        let bytes = item.to_be_bytes();
        j = i * 2;
        sendBuf[j] = bytes[0];
        sendBuf[j + 1] = bytes[1];
    }
    println!("send buffer({}): {:?}", sendBuf.len(), sendBuf);

    debug!("[main] creating TcpServer...");
    let tcpSrv = Arc::new(Mutex::new(
        TcpServer::new(
            "192.168.120.172:5180",
            // "127.0.0.1:5180",
            // inputSignal.clone(),
        )
    ));
    debug!("[main] TcpServer created");
    debug!("[main] starting TcpServer...");
    TcpServer::run(tcpSrv).unwrap();
    debug!("[main] TcpServer started");
}
