#![allow(non_snake_case)]

#[path = "../debug_session/mod.rs"]
mod debug_session;
#[path = "../traits/mod.rs"]
mod traits;

extern crate multiqueue;

use std::{collections::HashMap, time::{Instant, Duration}, thread, sync::mpsc::{Sender, Receiver, self}};
use log::{warn, info};

use crate::{
    debug_session::debug_session::{DebugSession, LogLevel},
    traits::{
        app_core::{bool::Bool, point::PointType},
        nested_fn_generic::{functions::{FnInput, FnType}, producer::ProducerChannel, t_in_out::TInOut},
    }, 
};

const ITERATIONS: usize = 100_000_000;



fn main() {
    DebugSession::init(LogLevel::Trace);

    let (send, recv): (Sender<PointType>, Receiver<PointType>) = mpsc::channel();
    let mut p1 = ProducerChannel::new(ITERATIONS, send.clone());
    let mut p2 = ProducerChannel::new(ITERATIONS, send);
    p1.run();
    p2.run();
    thread::sleep(Duration::from_secs_f32(1.1));
    
    let mut inputs: HashMap<String, FnType> = HashMap::from([
        (String::from("float1"), FnType::Float( FnInput { id: String::from("float1"), value: 0.0, status: 0, timestamp: chrono::offset::Utc::now() } )), 
        (String::from("int1"), FnType::Int( FnInput { id: String::from("float1"), value: 0, status: 0, timestamp: chrono::offset::Utc::now() } )), 
        (String::from("bool1"), FnType::Bool( FnInput { id: String::from("float1"), value: Bool(false), status: 0, timestamp: chrono::offset::Utc::now() } )), 
    ]);
    info!("Receiving...: {}", ITERATIONS);
    let mut received = 0;
    let time = Instant::now();
    loop {
        match recv.recv() {
            Ok(point) => {
                received += 1;
                match point {
                    PointType::Bool(point) => {
                        let input = inputs.get_mut(&point.name);
                        if input.is_some() {
                            match input.unwrap() {
                                FnType::Bool(i) => {
                                    // i.add(point);
                                    i.out();
                                    // trace!("FnInput.value: {:?}", i.value);
                                },
                                _ => panic!("wrong type"),
                            }
                        } else {
                            panic!("FnInput '{}' not found", point.name)
                        }
                    },
                    PointType::Int(point) => {
                        let input = inputs.get_mut(&point.name);
                        if input.is_some() {
                            match input.unwrap() {
                                FnType::Int(i) => {
                                    i.add(point);
                                    i.out();
                                    // trace!("FnInput.value: {:?}", i.value);
                                },
                                _ => panic!("wrong type"),
                            }
                        } else {
                            panic!("FnInput '{}' not found", point.name)
                        }
                    },
                    PointType::Float(point) => {
                        let input = inputs.get_mut(&point.name);
                        if input.is_some() {
                            match input.unwrap() {
                                FnType::Float(i) => {
                                    i.add(point);
                                    i.out();
                                    // trace!("FnInput.value: {:?}", i.value);
                                },
                                _ => panic!("wrong type"),
                            }
                        } else {
                            panic!("FnInput '{}' not found", point.name)
                        }
                    },
                };
            },
            Err(err) => {
                warn!("Error read from queue: {:?}", err);
                break;
            },
        };
    }
    println!("elapsed: {:?}", time.elapsed());
    info!("Received points: {}", received);
}
