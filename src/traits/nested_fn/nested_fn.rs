extern crate multiqueue;
#[path="../../debug/mod.rs"]
mod debug;
use std::{collections::HashMap, fmt::Debug, time::{Instant, Duration}, thread, sync::atomic::Ordering};
use multiqueue::{mpmc_queue, MPMCSender};
use log::{trace, debug, warn, info};
use rand::Rng;

use crate::debug::debug_session::{DebugSession, LogLevel};

#[derive(Clone)]
struct FnInput<T> {
    value: T
}
struct FnSum;
struct FnMul;
struct FnCompare;

trait TOutput<T> {
    fn out(&self) -> T;
}


trait TInput<T> {
    fn add(&mut self, value: T);
}


impl<T: Debug> TInput<T> for FnInput<T> {
    fn add(&mut self, value: T) {
        self.value = value;
        // f
        // println!("FnInput<{}>.add | value: {:?}", std::any::type_name::<T>(), &self.value)
    }
}

const QSIZE: u64 = 1_000;


fn producer(send: MPMCSender<PointType>) {
    let iterations = 10_000_000;
    
    let h = thread::Builder::new().name("name".to_owned()).spawn(move || {
        let name = "prodicer";
        debug!("Task({}).run | calculating step...", name);

        let queue = vec![
            PointType::Bool(Point {value:true, name: String::from("bool1") }),
            PointType::Int(Point {value:13, name: String::from("int1") }),
            PointType::Int(Point {value:43, name: String::from("int1") }),
            PointType::Bool(Point {value:false, name: String::from("bool1") }),
            PointType::Float(Point {value:12.77, name: String::from("float1") }),
            PointType::Int(Point {value:65, name: String::from("int1") }),
        ];
        let mut random = rand::thread_rng();
        let max = queue.len() - 1;
        let mut sent = 0;
        for _ in 1..iterations {
            let index = random.gen_range(0..max);
            let point = &queue[index];
            match send.try_send(point.clone()) {
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

    let (send, recv) = mpmc_queue(QSIZE);

    producer(send);
    
    let mut inputs: HashMap<String, FnType> = HashMap::from([
        (String::from("float1"), FnType::Float( FnInput { value: 0.0 } )), 
        (String::from("int1"), FnType::Int( FnInput { value: 0 } )), 
        (String::from("bool1"), FnType::Bool( FnInput { value: false } )), 
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
                                    i.add(point.value);
                                    // trace!("FnInput.value: {:?}", i.value);
                                },
                                _ => panic!("wrong type"),
                            };
                        }
                    },
                    PointType::Int(point) => {
                        let input = inputs.get_mut(&point.name);
                        if input.is_some() {
                            match input.unwrap() {
                                FnType::Int(i) => {
                                    i.add(point.value);
                                    // trace!("FnInput.value: {:?}", i.value);
                                },
                                _ => panic!("wrong type"),
                            };                
                        }
                    },
                    PointType::Float(point) => {
                        let input = inputs.get_mut(&point.name);
                        if input.is_some() {
                            match input.unwrap() {
                                FnType::Float(i) => {
                                    i.add(point.value);
                                    // trace!("FnInput.value: {:?}", i.value);
                                },
                                _ => panic!("wrong type"),
                            };                
                        }
                    },
                }                
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


#[derive(Clone)]
struct Point<T> {
    name: String,
    value: T,
}

#[derive(Clone)]
enum PointType {
    Bool(Point<bool>),
    Int(Point<i64>),
    Float(Point<f64>),
}

enum FnType {
    Bool(FnInput<bool>),
    Int(FnInput<i64>),
    Float(FnInput<f64>),
}