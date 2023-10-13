extern crate multiqueue;
#[path="../../debug/mod.rs"]
mod debug;
#[path="./functions.rs"]
mod functions;

use std::{collections::HashMap, time::{Instant, Duration}, thread, sync::mpsc::{Sender, Receiver, self}};
use log::{debug, warn, info};
use rand::Rng;

use crate::{debug::debug_session::{DebugSession, LogLevel}, functions::{FnInput, FnType, PointType, Point, TInput, TOutput}};

const ITERATIONS: usize = 10_000_000;

fn points() ->Vec<PointType> {
    vec![
        PointType::Bool(  Point { value:true,   name:String::from("bool1"),  status: 0, timestamp: chrono::offset::Utc::now() }),
        PointType::Int(   Point { value:13,     name:String::from("int1"),   status: 0, timestamp: chrono::offset::Utc::now() }),
        PointType::Int(   Point { value:43,     name:String::from("int1"),   status: 0, timestamp: chrono::offset::Utc::now() }),
        PointType::Bool(  Point { value:false,  name:String::from("bool1"),  status: 0, timestamp: chrono::offset::Utc::now() }),
        PointType::Float( Point {value: 12.77,  name:String::from("float1"), status: 0, timestamp: chrono::offset::Utc::now() }),
        PointType::Int(   Point { value:65,     name:String::from("int1"),   status: 0, timestamp: chrono::offset::Utc::now() }),
    ]
}

fn producer1(iterations: usize, send: Sender<PointType>) {
    let h = thread::Builder::new().name("name".to_owned()).spawn(move || {
        let name = "prodicer";
        debug!("Task({}).run | calculating step...", name);
        let points = points();
        let mut random = rand::thread_rng();
        let max = points.len();
        let mut sent = 0;
        for _ in 0..iterations {
            let index = random.gen_range(0..max);
            let point = &points[index];
            match send.send(point.clone()) {
                Ok(_) => {
                    sent += 1;
                },
                Err(err) => {
                    warn!("Error write to queue: {:?}", err);
                },
            }
        }        
        info!("Sent points: {}", sent);
        thread::sleep(Duration::from_secs_f32(0.1));
        // debug!("Task({}).run | calculating step - done ({:?})", name, cycle.elapsed());
    }).unwrap();    
}

fn producer2(iterations: usize, send: Sender<PointType>) {
    let h = thread::Builder::new().name("name".to_owned()).spawn(move || {
        let name = "prodicer";
        debug!("Task({}).run | calculating step...", name);

        let points = points();
        let mut random = rand::thread_rng();
        let max = points.len();
        let mut sent = 0;
        for _ in 0..iterations {
            let index = random.gen_range(0..max);
            let point = &points[index];
            match send.send(point.clone()) {
                Ok(_) => {
                    sent += 1;
                },
                Err(err) => {
                    warn!("Error write to queue: {:?}", err);
                },
            }
        }        
        info!("Sent points: {}", sent);
        thread::sleep(Duration::from_secs_f32(0.1));
        // debug!("Task({}).run | calculating step - done ({:?})", name, cycle.elapsed());
    }).unwrap();    
}

fn main() {
    DebugSession::init(LogLevel::Trace);

    // let (send, recv) = mpmc_queue(QSIZE);
    let (send, recv): (Sender<PointType>, Receiver<PointType>) = mpsc::channel();

    producer1(ITERATIONS, send.clone());
    producer2(ITERATIONS, send);
    thread::sleep(Duration::from_secs_f32(1.1));
    
    let mut inputs: HashMap<String, FnType> = HashMap::from([
        (String::from("float1"), FnType::Float( FnInput { value: 0.0, status: 0, timestamp: chrono::offset::Utc::now() } )), 
        (String::from("int1"), FnType::Int( FnInput { value: 0, status: 0, timestamp: chrono::offset::Utc::now() } )), 
        (String::from("bool1"), FnType::Bool( FnInput { value: false, status: 0, timestamp: chrono::offset::Utc::now() } )), 
    ]);
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
