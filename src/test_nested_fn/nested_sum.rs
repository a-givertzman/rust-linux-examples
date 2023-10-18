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
        functions::FnMetric, fn_inputs::{FnInputs, InputType}, bool::Bool,
    },
};


///
/// 
fn points() ->Vec<PointType> {
    vec![
        PointType::Bool(  Point { value: Bool(true),   name:String::from("bool1"),  status: 0, timestamp: chrono::offset::Utc::now() }),
        PointType::Int(   Point { value: 13,     name:String::from("int1"),   status: 0, timestamp: chrono::offset::Utc::now() }),
        PointType::Int(   Point { value: 43,     name:String::from("int1"),   status: 0, timestamp: chrono::offset::Utc::now() }),
        PointType::Bool(  Point { value: Bool(false),  name:String::from("bool1"),  status: 0, timestamp: chrono::offset::Utc::now() }),
        PointType::Float( Point { value: 12.77,  name:String::from("input1"), status: 0, timestamp: chrono::offset::Utc::now() }),
        PointType::Int(   Point { value: 65,     name:String::from("int1"),   status: 0, timestamp: chrono::offset::Utc::now() }),
    ]
}
///
/// 
fn metric(conf: &mut Conf, inputs: &mut FnInputs) -> Box<dyn TOutput<String>> {
    match conf.initial {
        Initial::Int(initial) => {
            let func = function::<i64>(conf, initial, String::new(), inputs);
            Box::new(FnMetric::<i64> {
                input: func,
            })
        },
        Initial::Float(initial) => {
            let func = function::<f64>(conf, initial, String::new(), inputs);
            Box::new(FnMetric::<f64> {
                input: func,
            })
        },
        Initial::Bool(initial) => {
            let func = function::<Bool>(conf, Bool(initial), String::new(), inputs);
            Box::new(FnMetric::<Bool> {
                input: func,
            })
        }
        // panic!("Bool is not implemented"),
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
// fn function<T>(conf: &mut Conf, initial: T, inputName: String, inputs: &mut FnInputs) -> (HashMap<String, Rc<RefCell<Box<dyn TInOut<Point<T>, T>>>>>, Rc<RefCell<Box<dyn TInOut<Point<T>, T>>>>) where 
fn function<T>(conf: &mut Conf, initial: T, inputName: String, inputs: &mut FnInputs) -> Rc<RefCell<Box<dyn TInOut<Point<T>, T>>>> where 
    T: Debug + Clone + std::ops::Add<Output = T> + 'static {
    match conf.name().as_str() {
        "input" => {
            println!("input function");
            // let mut input = fnInput(inputName.clone(), initial);
            // let a = input.borrow_mut();
            let input = match conf.initial {
                Initial::Bool(initial) => {
                    let mut input = fnInput(inputName.clone(), Bool(initial));
                    inputs.add(inputName, InputType::Bool(input.clone()));
                    input
                },
                Initial::Int(initial) => {
                    let mut input = fnInput(inputName.clone(), initial);
                    inputs.addInt(inputName, input.clone());
                    input
                },
                Initial::Float(_initial) => {
                    inputs.addFloat(inputName, input.clone());
                },
                Initial::None => todo!(),
            }
            input
        },
        "sum" => {
            println!("sum function");
            let in1Name = String::from("input1");
            let in2Name = String::from("input2");
            let input1 = function::<T>(conf.nested(&in1Name), initial.clone(), in1Name, inputs);
            let input2 = function::<T>(conf.nested(&in2Name), initial, in2Name, inputs);
            let func = fnSum(inputName, input1, input2);
            func
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
                initial: Initial::Float(0.0),
                nested: HashMap::new()
            }),
            (String::from("input2"), Conf {
                name: String::from("input"),
                initial: Initial::Float(0.0),
                nested: HashMap::new()
            }),
        ])
    };

    let mut metricInputs = FnInputs::new();
    let metric = metric(&mut conf, &mut metricInputs);
    println!("INPUTS: {:?}", &metricInputs);

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


// #[derive(Debug, Clone)]
// enum InputType {
//     Bool(Rc<RefCell<Box<dyn TInOut<Point<bool>, bool>>>>),
//     Int(Rc<RefCell<Box<dyn TInOut<Point<i64>, i64>>>>),
//     Float(Rc<RefCell<Box<dyn TInOut<Point<f64>, f64>>>>),
// }


























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