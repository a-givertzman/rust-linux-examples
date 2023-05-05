#![allow(non_snake_case)]

use log::{
    info,
    // trace,
    debug,
    warn,
};
use std::{
    env, 
    net::UdpSocket, 
    time::Duration, 
    sync::{Arc, Mutex},
};

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

struct UdpServer {
    localAddr: String, //SocketAddr,
    remoteAddr: String, //SocketAddr,
    reconnectDelay: Duration,
    pub isConnected: bool,
    cancel: bool,
}

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
            info!("[UdpServer.run] try to open on: {:?}\n", localAddr);
            match UdpSocket::bind(localAddr) {
                Ok(socket) => {
                    info!("[UdpServer.run] opened on: {:?}\n", localAddr);
                    thisMutax.isConnected = true;
                    info!("[UdpServer.run] isConnected true done: {:?}\n", thisMutax.isConnected);
                    const QSIZE_SRC: usize = 1024;
                    const QSIZE_TARGET: usize = QSIZE_SRC / 2;
                    let mut buf = [0; QSIZE_SRC];
                    let mut bufTarget = [0; QSIZE_TARGET];
                    match socket.send_to(&buf, remoteAddr) {
                        Ok(_) => {},
                        Err(err) => {
                            warn!("[UdpServer.run] send error: {:#?}", err);
                        },
                    };
                    loop {
                        match socket.recv_from(&mut buf) {
                            Ok((amt, src)) => {
                                debug!("[UdpServer.run] receaved bytes({}) from{:?}: {:?}",amt, src, buf);
                                let mut bytes = [0_u8; 2];
                                for i in 0..QSIZE_TARGET {
                                    bytes[0] = buf[i * 2];
                                    bytes[1] = buf[i * 2 + 1];
                                    bufTarget[i] = u16::from_be_bytes(bytes);
                                }
                                debug!("[UdpServer.run] receaved bytes({}) from{:?}: {:?}",amt, src, bufTarget);
                                bufTarget.fill(0);
                                buf.fill(0)
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
}
