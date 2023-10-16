#![allow(non_snake_case)]

#[path="../../debug/mod.rs"]
mod debug;
#[path="./functions.rs"]
mod functions;

use std::{collections::HashMap, cell::RefCell, sync::Arc, borrow::BorrowMut};

use debug::debug_session::{DebugSession, LogLevel};
use functions::{FnSum, FnMetric, FnType};

use crate::functions::{FnInput, TOutput, TInput, PointType, Point};

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


fn metric(conf: &mut Conf) -> (Box<dyn TOutput<String>>, Vec<InputType>) {
    match conf.initial {
        Initial::Int(initial) => {
            let (func, inputs) = function::<i64>(conf, initial);
            (
                Box::new(FnMetric::<i64> {
                    input: func,
                }),
                inputs.into_iter().map(|v| {
                    InputType::Int(v)
                }).collect()
            )
        },
        Initial::Float(initial) => {
            let (func, inputs) = function::<f64>(conf, initial);
            (
                Box::new(FnMetric::<f64> {
                    input: func,
                }),
                inputs.into_iter().map(|v| {
                    InputType::Float(v)
                }).collect()
            )
        },
        Initial::Bool(_) => panic!("Bool is not implemented"),
        Initial::None => panic!("Unknown type of initial"),
    }
}
fn function<T: Clone + std::ops::Add<Output = T> + 'static>(conf: &mut Conf, initial: T) -> (Arc<dyn TOutput<T>>, Vec<Arc<FnInput<T>>>) {
    // T: Clone + std::ops::Add<Output = T> + 'static {
    match conf.name().as_str() {
        "input" => {
            println!("input function");
            let mut input = Arc::new(
                FnInput::<T> { 
                    value: initial, 
                    status: 0, 
                    timestamp: chrono::offset::Utc::now() 
                }
            );
            (
                input.clone(),
                vec![input]
            )
        },
        "sum" => {
            println!("sum function");
            let mut inputs = vec![];
            let (input1, mut inputs1) = function::<T>(conf.nested("input1"), initial.clone());
            let (input2, mut inputs2) = function::<T>(conf.nested("input2"), initial);
            inputs.append(&mut inputs1);
            inputs.append(&mut inputs2);
            (
                Arc::new(                
                    FnSum::<T> { 
                        input1: input1, 
                        input2: input2, 
                        status: 0, 
                        timestamp: chrono::offset::Utc::now(),
                    }
                ),
                inputs
            )
        }
        _ => panic!("Unknown function name: {:?}", conf.name())
    }
}

fn main() {
    DebugSession::init(LogLevel::Debug);
    let mut conf = Conf {
        name: String::from("sum"),
        initial: Initial::Float(0.123),
        nested: HashMap::from([
            (String::from("input1"), Conf {
                name: String::from("input"),
                initial: Initial::None,
                nested: HashMap::new()
            }),
            (String::from("input2"), Conf {
                name: String::from("input"),
                initial: Initial::None,
                nested: HashMap::new()
            }),
        ])
    };

    let testData = vec![0, 1, 2, 3];

    let (metric, mut inputs) = metric(&mut conf);
    for point in points() {
        for input in &mut inputs {
            match point.clone() {
                PointType::Bool(point) => {
                    match input {
                        InputType::Bool(input) => {
                            input.add(point.clone());
                        },
                        _ => panic!("Incompatible type of: {:?}", point),
                    }                    
                },
                PointType::Int(point) => {
                    match input {
                        InputType::Int(input) => {
                            input.borrow_mut().add(point);
                        },
                        _ => panic!("Incompatible type of: {:?}", point),
                    }
                    
                },
                PointType::Float(point) => {
                    match input {
                        InputType::Float(input) => {
                            input.borrow_mut().add(point);
                        },
                        _ => panic!("Incompatible type of: {:?}", point),
                    }                    
                    
                },
            }
        }
        let out = metric.out();
        println!("metric out: {:?}", out);
    }
}


struct Conf {
    name: String,
    initial: Initial,
    nested: HashMap<String, Conf>,
}

impl Conf {
    pub fn name(&self) -> String {
        self.name.clone()
    }
    pub fn get(&self, key: &str) -> String {
        self.name.clone()
    }
    pub fn nested(&mut self, key: &str) -> &mut Conf {
        self.nested.get_mut(key).unwrap()
    }
}

#[derive(Clone)]
enum Initial {
    Bool(bool),
    Int(i64),
    Float(f64),
    None,
}


enum InputType {
    Bool(Arc<FnInput<bool>>),
    Int(Arc<FnInput<i64>>),
    Float(Arc<FnInput<f64>>),
}


























    // pub fn initialAsBool(&self) ->bool {
    //     match self.initial {
    //         Initial::Bool(v) => v,
    //         _ => panic!("Incorrect type of initoal - BOOL"),
    //     }
    // }
    // pub fn initialAsInt(&self) -> i64 {
    //     match self.initial {
    //         Initial::Int(v) => v,
    //         _ => panic!("Incorrect type of initoal - Int"),
    //     }
    // }
    // pub fn initialAsFLoat(&self) -> f64 {
    //     match self.initial {
    //         Initial::Float(v) => v,
    //         _ => panic!("Incorrect type of initoal - Float"),
    //     }
    // }