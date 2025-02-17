#![allow(non_snake_case)]


use std::{rc::Rc, cell::RefCell};

use log::{debug, trace};

use crate::traits::app_core::point::PointType;

use super::t_in_out::{FnIn, FnOut, FnInOut};




///
/// 
#[derive(Debug, Clone)]
pub struct FnInput {
    pub id: String,
    pub point: PointType,
}
///
/// 
impl FnIn for FnInput {
    fn add(&mut self, point: PointType) {
        self.point = point;
        trace!("FnInput({}).add | value: {:?}", self.id, &self.point);
    }
}
///
/// 
impl FnOut for FnInput {
    fn out(&self) -> PointType {
        trace!("FnInput({}).out | value: {:?}", self.id, &self.point);
        self.point.clone()
    }
}
///
/// 
// impl<T: FnInOut> FnInOut for T {}
impl FnInOut for FnInput {}





///
/// Function do Add of input1 and input2
#[derive(Debug)]
pub struct FnSum {
    pub id: String,
    pub input1: Rc<RefCell<Box<dyn FnInOut>>>,
    pub input2: Rc<RefCell<Box<dyn FnInOut>>>,
}
///
/// 
impl FnIn for FnSum {
    fn add(&mut self, _: PointType) {
        panic!("FnSum.add | method is not used")
    }
}
impl FnOut for FnSum { 
    fn out(&self) -> PointType {
        let value1 = self.input1.borrow().out();
        trace!("FnSum({}).out | value1: {:?}", self.id, &value1);
        let value2 = self.input2.borrow().out();
        trace!("FnSum({}).out | value2: {:?}", self.id, &value2);
        let out = match value1 {
            PointType::Bool(value1) => {
                PointType::Bool(value1 | value2.asBool())
            },
            PointType::Int(value1) => {
                PointType::Int(value1 + value2.asInt())
            },
            PointType::Float(value1) => {
                PointType::Float(value1 + value2.asFloat())
            },
        };
        trace!("FnSum({}).out | out: {:?}", self.id, &out);
        out
    }
}
impl FnInOut for FnSum {}





















pub struct FnMul;
pub struct FnOr;
pub struct FnCompare;
























// impl<T: std::fmt::Debug> TInput<T> for FnInput<T> {
//     fn add(&mut self, point: Point<T>) {
//         self.value = point.value;
//         self.status = point.status;
//         self.timestamp = point.timestamp;
//         println!("FnInput({})<{}>.add | value: {:?}", self.id, std::any::type_name::<T>(), &self.value);
//     }
// }

// impl<T: Debug + Clone> TOutput<T> for FnInput<T> {
//     fn out(&self) -> T {
//         println!("FnInput({})<{}>.out | value: {:?}", self.id, std::any::type_name::<T>(), &self.value);
//         self.value.clone()
//     }
// }

// impl<I: std::ops::Add<Output = I>> TInOut<Point<I>, I> for FnSum<I> where 
//     I: std::fmt::Debug + Clone {
//     fn add(&mut self, value: Point<I>) {
//         println!("FnSum({})<{}>.add | value: --", self.id, std::any::type_name::<I>());
//     }
//     fn out(&self) -> I {
//         let value1 = self.input1.borrow().out();
//         let value2 = self.input2.borrow().out();
//         let sum = value1 + value2;
//         sum
//     }
// }

// impl<T: std::fmt::Debug> TInput<T> for FnSum<T> {
//     fn add(&mut self, value: Point<T>) {
//         println!("FnSum({})<{}>.add | value: --", self.id, std::any::type_name::<T>());
//     }
// }
    
// impl<T> TOutput<T> for FnSum<T> where
//     T: Debug + std::ops::Add<Output = T> {
//     fn out(&self) -> T {
//         let value1 = self.input1.borrow().out();
//         let value2 = self.input2.borrow().out();
//         let sum = value1 + value2;
//         sum
//     }
// }
