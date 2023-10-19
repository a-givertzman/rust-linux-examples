#![allow(non_snake_case)]

use std::{rc::Rc, cell::RefCell};

use crate::traits::nested_fn_enum::t_in_out::FnInOut;

#[derive(Debug)]
pub struct FnMetric {
    pub input: Rc<RefCell<Box<dyn FnInOut>>>,
}



impl FnMetric {
    pub fn out(&self) -> String {
        let pointType = self.input.borrow().out();
        match pointType {
            crate::traits::app_core::point::PointType::Bool(point) => {
                format!("insert into table values ({})", point.value)
            },
            crate::traits::app_core::point::PointType::Int(point) => {
                format!("insert into table values ({})", point.value)
            },
            crate::traits::app_core::point::PointType::Float(point) => {
                format!("insert into table values ({})", point.value)
            },
        }
    }
}
