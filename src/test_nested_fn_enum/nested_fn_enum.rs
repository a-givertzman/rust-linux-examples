#![allow(non_snake_case)]

use std::{collections::HashMap, cell::RefCell, rc::Rc};

use debug_session::debug_session::{DebugSession, LogLevel};
use log::{debug, warn};
use traits::{nested_fn_enum::{fn_inputs::FnInputs, functions::{FnInput, FnSum}, t_in_out::{FnIn, FnOut, FnInOut}, fn_metric::FnMetric}, app_core::point::PointType};

use crate::traits::app_core::{point::Point, bool::Bool};

#[path = "../debug_session/mod.rs"]
mod debug_session;
#[path = "../traits/mod.rs"]
mod traits;



#[derive(Clone)]
enum Initial {
    Bool(bool),
    Int(i64),
    Float(f64),
    None,
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




///
/// 
fn points() -> Vec<PointType> {
    vec![
        PointType::Bool(  Point { value: Bool(true),   name:String::from("bool1"),  status: 0, timestamp: chrono::offset::Utc::now() }),
        PointType::Int(   Point { value: 13,     name:String::from("int1"),   status: 0, timestamp: chrono::offset::Utc::now() }),
        PointType::Int(   Point { value: 43,     name:String::from("int1"),   status: 0, timestamp: chrono::offset::Utc::now() }),
        PointType::Bool(  Point { value: Bool(false),  name:String::from("bool1"),  status: 0, timestamp: chrono::offset::Utc::now() }),
        PointType::Float( Point { value: 12.77,  name:String::from("input1"), status: 0, timestamp: chrono::offset::Utc::now() }),
        PointType::Float( Point { value: 1.0,  name:String::from("input2"), status: 0, timestamp: chrono::offset::Utc::now() }),
        PointType::Int(   Point { value: 65,     name:String::from("int1"),   status: 0, timestamp: chrono::offset::Utc::now() }),
    ]
}
///
/// 
fn boxFnInput(input: FnInput) -> Box<(dyn FnInOut)> {
    Box::new(input)
}
fn fnInput(inputName: String, initial: PointType) -> Rc<RefCell<Box<dyn FnInOut>>> {
    // let f: Box<dyn FnInOut> = Box::new(FnInput { 
    //     id: inputName.clone(),
    //     point: initial, 
    // });
    Rc::new(RefCell::new(
        boxFnInput(
            FnInput { 
                id: inputName.clone(),
                point: initial, 
            }
        )
    ))
}
///
/// 
fn boxFnSum(input: FnSum) -> Box<(dyn FnInOut)> {
    Box::new(input)
}
fn fnSum(
    inputName: String, 
    input1: Rc<RefCell<Box<dyn FnInOut>>>, 
    input2: Rc<RefCell<Box<dyn FnInOut>>>
) -> Rc<RefCell<Box<dyn FnInOut>>> {
    Rc::new(RefCell::new(
        boxFnSum(        
            FnSum {
                id: inputName,
                input1: input1, 
                input2: input2, 
            }
        )
    ))
}
///
/// 
fn function(conf: &mut Conf, initial: PointType, inputName: String, inputs: &mut FnInputs) -> Rc<RefCell<Box<dyn FnInOut>>> {
    match conf.name().as_str() {
        "input" => {
            println!("input function {:?}...", inputName);
            let input = fnInput(inputName.clone(), initial);
            inputs.add(inputName, input.clone());
            // let a = input.borrow_mut();
            println!("input function: {:?}", input);
            input
        },
        "sum" => {
            println!("sum function");
            let in1Name = String::from("input1");
            let in2Name = String::from("input2");
            let input1 = function(conf.nested(&in1Name), initial.clone(), in1Name, inputs);
            let input2 = function(conf.nested(&in2Name), initial, in2Name, inputs);
            let func = fnSum(inputName, input1, input2);
            func
        }
        _ => panic!("Unknown function name: {:?}", conf.name())
    }
}
///
/// 
fn metric(conf: &mut Conf, inputs: &mut FnInputs) -> FnMetric {
    let initial = match conf.initial {
        Initial::Bool(initial) => {
            PointType::Bool(  Point { value: Bool(initial),   name:String::from("bool"),  status: 0, timestamp: chrono::offset::Utc::now() })
        }
        Initial::Int(initial) => {
            PointType::Int(   Point { value: initial,     name:String::from("int"),   status: 0, timestamp: chrono::offset::Utc::now() })
        },
        Initial::Float(initial) => {
            PointType::Float( Point { value: initial,  name:String::from("float"), status: 0, timestamp: chrono::offset::Utc::now() })
        },
        // panic!("Bool is not implemented"),
        Initial::None => panic!("Unknown type of initial"),
    };
    let func = function(conf, initial, String::new(), inputs);
    FnMetric {
        input: func,
    }
}

fn main() {
    DebugSession::init(LogLevel::Trace);
    
    let mut conf = Conf {
        name: String::from("sum"),
        initial: Initial::Float(0.0),
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
        match metricInputs.get(&pointName) {
            Some(input) => {
                println!("input point: {:?}", &point);
                input.borrow_mut().add(point);
                let out = metric.out();
                println!("metric out: {:?}", out);
            },
            None => warn!("input {:?} - not found", pointName),
        };
    }
}