#![allow(non_snake_case)]

#[path="../../debug/mod.rs"]
mod debug;
#[path="./functions.rs"]
mod functions;

use std::{collections::{HashMap, hash_map::Entry}, cell::RefCell, sync::Arc, borrow::BorrowMut, fmt::Debug};

use debug::debug_session::{DebugSession, LogLevel};
use functions::{FnSum, FnMetric, FnType};
use log::{warn, debug};

use crate::functions::{FnInput, TOutput, TInput, PointType, Point};

///
/// 
fn points() ->Vec<PointType> {
    vec![
        PointType::Bool(  Point { value:true,   name:String::from("bool1"),  status: 0, timestamp: chrono::offset::Utc::now() }),
        PointType::Int(   Point { value:13,     name:String::from("int1"),   status: 0, timestamp: chrono::offset::Utc::now() }),
        PointType::Int(   Point { value:43,     name:String::from("int1"),   status: 0, timestamp: chrono::offset::Utc::now() }),
        PointType::Bool(  Point { value:false,  name:String::from("bool1"),  status: 0, timestamp: chrono::offset::Utc::now() }),
        PointType::Float( Point {value: 12.77,  name:String::from("input1"), status: 0, timestamp: chrono::offset::Utc::now() }),
        PointType::Int(   Point { value:65,     name:String::from("int1"),   status: 0, timestamp: chrono::offset::Utc::now() }),
    ]
}
///
/// 
fn metric(conf: &mut Conf) -> (Box<dyn TOutput<String>>, HashMap<String, InputType>) {
    match conf.initial {
        Initial::Int(initial) => {
            let (inputs, func) = function::<i64>(conf, initial, String::new());
            (
                Box::new(FnMetric::<i64> {
                    input: func,
                }),
                inputs.into_iter().map(|(key,v)| {
                    (key, InputType::Int(v))
                }).collect()
            )
        },
        Initial::Float(initial) => {
            let (inputs, func) = function::<f64>(conf, initial, String::new());
            (
                Box::new(FnMetric::<f64> {
                    input: func,
                }),
                inputs.into_iter().map(|(key, v)| {
                    (key, InputType::Float(v))
                }).collect()
            )
        },
        Initial::Bool(_) => panic!("Bool is not implemented"),
        Initial::None => panic!("Unknown type of initial"),
    }
}
///
/// 
fn function<T>(conf: &mut Conf, initial: T, inputName: String) -> (HashMap<String, RefCell<Box<FnInput<T>>>>, RefCell<Box<dyn TOutput<T>>>) where 
    T: Debug + Clone + std::ops::Add<Output = T> + 'static {
    match conf.name().as_str() {
        "input" => {
            println!("input function");
            let input = RefCell::new(Box::new(
                FnInput::<T> { 
                    id: inputName.clone(),
                    value: initial, 
                    status: 0, 
                    timestamp: chrono::offset::Utc::now() 
                }
            ));
            (
                HashMap::from([
                    (inputName, input.clone())
                ]),
                input
            )
        },
        "sum" => {
            println!("sum function");
            let mut inputs = HashMap::new();
            let in1Name = String::from("input1");
            let in2Name = String::from("input2");
            let (inputs1, input1) = function::<T>(conf.nested(&in1Name), initial.clone(), in1Name);
            let (inputs2, input2) = function::<T>(conf.nested(&in2Name), initial, in2Name);
            inputs.extend(inputs1);
            inputs.extend(inputs2);
            (
                inputs,
                RefCell::new(Box::new(         
                    FnSum::<T> { 
                        input1: input1, 
                        input2: input2, 
                        status: 0, 
                        timestamp: chrono::offset::Utc::now(),
                    }
                ))
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

    let (metric, mut inputs) = metric(&mut conf);
    println!("INPUTS: {:?}", &inputs);
    for point in points() {
        let pointName = point.name();
        debug!("received point: {:?}", point);
        match inputs.get_mut(&pointName) {
            Some(input) => {
                debug!("input found: {:?}", &input);
                match point.clone() {
                    PointType::Bool(point) => {
                        match input {
                            InputType::Bool(input) => {
                                let input = input.clone();
                                debug!("adding point to input...");
                                input.borrow_mut().add(point);
                                debug!("adding point to input - done");
                            },
                            _ => warn!("Incompatible type of: {:?}", point),
                        }                    
                    },
                    PointType::Int(point) => {
                        match input {
                            InputType::Int(input) => {
                                let input = input.clone();
                                input.borrow_mut().add(point);
                            },
                            _ => warn!("Incompatible type of: {:?}", point),
                        }
                        
                    },
                    PointType::Float(point) => {
                        match input {
                            InputType::Float(input) => {
                                let input = input.clone();
                                input.borrow_mut().add(point);
                            },
                            _ => warn!("Incompatible type of: {:?}", point),
                        }                    
                        
                    },
                }
            },
            None => {
                warn!("Input {:?} - not wound", &pointName);
            },
        };
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


#[derive(Debug, Clone)]
enum InputType {
    Bool(RefCell<Box<FnInput<bool>>>),
    Int(RefCell<Box<FnInput<i64>>>),
    Float(RefCell<Box<FnInput<f64>>>),
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