#![allow(non_snake_case)]

#[path = "../debug_session/mod.rs"]
mod debug_session;
#[path = "../traits/mod.rs"]
mod traits;

use std::{collections::HashMap, cell::RefCell, borrow::BorrowMut, fmt::Debug, rc::Rc};
use log::{warn, debug};
use traits::nested_fn::{t_in_out::TInOut, functions::{FnInput, FnSum}};

use crate::{
    debug_session::debug_session::{DebugSession, LogLevel}, 
    traits::nested_fn::{
        t_in_out::TOutput, 
        point::{Point, PointType},
        functions::FnMetric,
    },
};


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


fn fnInput<T: Debug + Clone + 'static>(inputName: String, initial: T) -> Rc<RefCell<Box<dyn TInOut<Point<T>, T>>>> {
    Rc::new(RefCell::new(
        Box::new(
            FnInput { 
                id: inputName.clone(),
                value: initial, 
                status: 0, 
                timestamp: chrono::offset::Utc::now() 
            }
        )
    ))
}
fn fnSum<T: Debug + Clone + std::ops::Add<Output = T> + 'static>(inputName: String, input1: Rc<RefCell<Box<dyn TInOut<Point<T>, T>>>>, input2: Rc<RefCell<Box<dyn TInOut<Point<T>, T>>>>) -> Rc<RefCell<Box<dyn TInOut<Point<T>, T>>>> {
    Rc::new(RefCell::new(
        Box::new(        
            FnSum {
                id: inputName,
                input1: input1, 
                input2: input2, 
                status: 0, 
                timestamp: chrono::offset::Utc::now(),
            }
        )
    ))
}

///
/// 
fn function<T>(conf: &mut Conf, initial: T, inputName: String) -> (HashMap<String, Rc<RefCell<Box<dyn TInOut<Point<T>, T>>>>>, Rc<RefCell<Box<dyn TInOut<Point<T>, T>>>>) where 
    T: Debug + Clone + std::ops::Add<Output = T> + 'static {
    match conf.name().as_str() {
        "input" => {
            println!("input function");
            let mut input = fnInput(inputName.clone(), initial);
            let a = input.borrow_mut();
            // a.add()
            // let input = Rc::new(RefCell::new(Box::new(
            //     FnInput::<T> { 
            //         id: inputName.clone(),
            //         value: initial, 
            //         status: 0, 
            //         timestamp: chrono::offset::Utc::now() 
            //     }
            // )));
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
            let func = fnSum(inputName, input1, input2);
            (
                inputs,
                func
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

    let (metric, inputs) = metric(&mut conf);
    println!("INPUTS: {:?}", &inputs);
    for point in points() {
        let pointName = point.name();
        debug!("received point: {:?}", point);
        match inputs.get(&pointName) {
            Some(mut input) => {
                debug!("input found: {:?}", &input);
                match point.clone() {
                    PointType::Bool(point) => {
                        match input {
                            InputType::Bool(input) => {
                                debug!("adding point to input...");
                                let mut input = input.to_owned().to_owned();
                                let mut input = input.borrow_mut();
                                // input.add(point);
                                // input.get_mut().add(point);
                                debug!("adding point to input - done");
                                debug!("modified input: {:?}", input);
                            },
                            _ => warn!("Incompatible type of: {:?}", point),
                        }                    
                    },
                    PointType::Int(point) => {
                        match input {
                            InputType::Int(input) => {
                                debug!("adding point to input...");
                                // input.get_mut().add(point);
                                debug!("adding point to input - done");
                                debug!("modified input: {:?}", input);
                            },
                            _ => warn!("Incompatible type of: {:?}", point),
                        }
                        
                    },
                    PointType::Float(point) => {
                        match input {
                            InputType::Float(input) => {
                                debug!("adding point to input...");
                                // input.get_mut().add(point);
                                debug!("adding point to input - done");
                                debug!("modified input: {:?}", input);
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
    Bool(Rc<RefCell<Box<dyn TInOut<Point<bool>, bool>>>>),
    Int(Rc<RefCell<Box<dyn TInOut<Point<i64>, i64>>>>),
    Float(Rc<RefCell<Box<dyn TInOut<Point<f64>, f64>>>>),
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