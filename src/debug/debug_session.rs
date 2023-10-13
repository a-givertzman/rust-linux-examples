use std::{env, sync::Once};

static INIT: Once = Once::new();

#[allow(dead_code)]
pub enum LogLevel {
    Off,
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

pub struct DebugSession {}

impl DebugSession {
    pub fn init(logLevel: LogLevel) {
        INIT.call_once(|| {
            let logLevel = match logLevel {
                LogLevel::Off => "off",
                LogLevel::Error => "error",
                LogLevel::Warn => "warn",
                LogLevel::Info => "info",
                LogLevel::Debug => "debug",
                LogLevel::Trace => "trace",
                // _ => "debug",
            };
            env::set_var("RUST_LOG", logLevel);  // off / error / warn / info / debug / trace
            // env::set_var("RUST_BACKTRACE", "1");
            env::set_var("RUST_BACKTRACE", "full");
            match env_logger::builder().is_test(true).try_init() {
                Ok(_) => {
                    println!("DebugSession.init | Ok\n")
                },
                Err(err) => {
                    println!("DebugSession.init | error: {:?}", err)
                },
            };
        })
    }
}
