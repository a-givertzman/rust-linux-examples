#![allow(non_snake_case)]

use std::{rc::Rc, cell::RefCell};

use crate::traits::{nested_fn_enum::{
    t_in_out::FnInOut,
    conf::{Conf, Initial},
    fn_inputs::FnInputs,
    
}, app_core::{point::{Point, PointType}, bool::Bool}};

use super::nested_fn::NestedFn;

#[derive(Debug)]
pub struct FnMetric {
    id: String,
    pub input: Rc<RefCell<Box<dyn FnInOut>>>,
}



impl FnMetric {
    ///
    /// 
    pub fn new(conf: &mut Conf, inputs: &mut FnInputs) -> FnMetric {
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
            Initial::None => panic!("Unknown type of initial"),
        };
        let func = NestedFn::new(conf, initial, inputs);
        FnMetric {
            id: conf.id.clone(),
            input: func,
        }
    }

    pub fn out(&self) -> String {
        let pointType = self.input.borrow().out();
        match pointType {
            PointType::Bool(point) => {
                format!("insert into table values(id, value, timestamp) ({},{},{})", self.id, point.value, point.timestamp)
            },
            PointType::Int(point) => {
                format!("insert into table values(id, value, timestamp) ({},{},{})", self.id, point.value, point.timestamp)
            },
            PointType::Float(point) => {
                format!("insert into table values(id, value, timestamp) ({},{},{})", self.id, point.value, point.timestamp)
            },
        }
    }
}
