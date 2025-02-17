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
};

fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "debug");
    env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    for i in 0..6 {
        let v1 = i << 1;
        let v2 = i << 2;
        let v3 = i << 3;
        let v4 = i << 4;
        debug!("[main] i ({:#b}): {}", i, i);
        debug!("[main] v1 ({:#b}): {}", v1, v1);
        debug!("[main] v2 ({:#b}): {}", v2, v2);
        debug!("[main] v3 ({:#b}): {}", v3, v3);
        debug!("[main] v4 ({:#b}): {}", v4, v4);
    }

    Ok(())
}
