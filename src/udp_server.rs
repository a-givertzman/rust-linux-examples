#![allow(non_snake_case)]

use log::{
    info,
    // trace,
    debug,
    warn,
};
use std::{
    net::UdpSocket, 
    time::Duration, 
    sync::{Arc, Mutex},
};
pub struct UdpServer {
    localAddr: String, //SocketAddr,
    remoteAddr: String, //SocketAddr,
    reconnectDelay: Duration,
    pub isConnected: bool,
    cancel: bool,
}

// T, uc	QSIZE
// 976.563	1 024
// 488.281	2 048
// 244.141	4 096
// 122.070	8 192
// 61.035	16 384
// 30.518	32 768
// 15.259	65 536
// 7.629	131 072
// 3.815	262 144
// 1.907	524 288

const SYN: u8 = 22;
const EOT: u8 = 4;
const QSIZE: usize = 512;
const QSIZE_DOUBLE: usize = QSIZE * 2;

impl UdpServer {
    ///
    pub fn new(
        localAddr: &str,
        remoteAddr: &str,
        // inputSignal: Arc<Mutex<InputSignal>>,
        reconnectDelay: Option<Duration>,
    ) -> Self {
        Self {
            localAddr: String::from(localAddr),
            remoteAddr: String::from(remoteAddr),
            reconnectDelay: match reconnectDelay {Some(rd) => rd, None => Duration::from_secs(3)},
            isConnected: false,
            cancel: false,
        }
    }
    ///
    pub fn run(this: Arc<Mutex<Self>>) -> () {
    // pub fn run(this: Arc<Mutex<Self>>) -> Result<(), Box<dyn Error>> {
        info!("[UdpServer.run] enter");
        let mut thisMutax = this.lock().unwrap();
        let cancel = thisMutax.cancel;
        let localAddr = &thisMutax.localAddr.clone();
        let remoteAddr = &thisMutax.remoteAddr.clone();
        let reconnectDelay = thisMutax.reconnectDelay;
        info!("[UdpServer.run] started");
        while !cancel {
            info!("[UdpServer.run] try to bind on: {:?}", localAddr);
            match UdpSocket::bind(localAddr) {
                Ok(socket) => {
                    info!("[UdpServer.run] ready on: {:?}\n", localAddr);
                    thisMutax.isConnected = true;
                    info!("[UdpServer.run] isConnected: {:?}\n", thisMutax.isConnected);
                    let mut bufDouble = [0; QSIZE_DOUBLE];
                    let mut buf = [0; QSIZE];
                    info!("[UdpServer.run] sending handshake({}): {:?}\n", bufDouble.len(), bufDouble);
                    match socket.send_to(&Self::handshake(), remoteAddr) {
                        Ok(_) => {},
                        Err(err) => {
                            warn!("[UdpServer.run] send error: {:#?}", err);
                        },
                    };
                    loop {
                        match socket.recv_from(&mut bufDouble) {
                            Ok((amt, src)) => {
                                // debug!("[UdpServer.run] receaved bytes({}) from{:?}: {:?}",amt, src, buf);
                                let mut bytes = [0_u8; 2];
                                for i in 0..QSIZE {
                                    bytes[0] = bufDouble[i * 2];
                                    bytes[1] = bufDouble[i * 2 + 1];
                                    buf[i] = u16::from_be_bytes(bytes);
                                }
                                debug!("[UdpServer.run] receaved bytes({}) from{:?}: {:?}",amt, src, buf);
                                buf.fill(0);
                                bufDouble.fill(0)
                            },
                            Err(err) => {
                                warn!("[UdpServer.run] read error: {:#?}", err);
                            },
                        };
                    }
                }
                Err(err) => {
                    thisMutax.isConnected = false;
                    debug!("[UdpServer.run] binding error on: {:?}\n\tdetailes: {:?}", localAddr, err);
                    std::thread::sleep(reconnectDelay);
                }
            }
            std::thread::sleep(reconnectDelay);
        }
        info!("[UdpServer.run] exit");
    }
    ///
    fn handshake() -> [u8; 2] {
        [SYN, EOT]
    }
}
