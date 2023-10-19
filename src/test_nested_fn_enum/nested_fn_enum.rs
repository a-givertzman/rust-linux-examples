#![allow(non_snake_case)]

use std::collections::HashMap;

use debug_session::debug_session::{DebugSession, LogLevel};
use log::trace;
use traits::{nested_fn_enum::{fn_inputs::FnInputs, fn_metric::FnMetric}, app_core::point::PointType};

use crate::traits::{app_core::{point::Point, bool::Bool}, nested_fn_enum::conf::{Initial, Conf}};

#[path = "../debug_session/mod.rs"]
mod debug_session;
#[path = "../traits/mod.rs"]
mod traits;



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

fn main() {
    DebugSession::init(LogLevel::Debug);
    
    let mut conf = Conf {
        id: String::from("metric_name"),
        name: String::from("sum"),
        initial: Initial::Float(0.0),
        nested: HashMap::from([
            (String::from("input1"), Conf {
                id: String::new(),
                name: String::from("input"),
                initial: Initial::Float(0.0),
                nested: HashMap::new()
            }),
            (String::from("input2"), Conf {
                id: String::new(),
                name: String::from("input"),
                initial: Initial::Float(0.0),
                nested: HashMap::new()
            }),
        ])
    };

    let mut metricInputs = FnInputs::new();
    let metric = FnMetric::new(&mut conf, &mut metricInputs);
    println!("INPUTS: {:?}", &metricInputs);
    println!("\n");
    for point in points() {
        let pointName = point.name();
        trace!("received point: {:?}", point);
        match metricInputs.get(&pointName) {
            Some(input) => {
                println!("input point: {:?}", &point);
                input.borrow_mut().add(point);
                let out = metric.out();
                println!("metric out: {:?}", out);
            },
            None => trace!("input {:?} - not found", pointName),
        };
    }
}