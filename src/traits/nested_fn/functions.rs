#![allow(non_snake_case)]

use std::{fmt::{Display, Debug}, cell::RefCell, rc::Rc};

use chrono::DateTime;

use crate::traits::nested_fn::{t_in_out::{TInOut, TInput, TOutput}, point::Point};


pub enum FnType {
    Bool(FnInput<bool>),
    Int(FnInput<i64>),
    Float(FnInput<f64>),
}



#[derive(Debug, Clone)]
pub struct FnInput<T> {
    pub id: String,
    pub value: T,
    pub status: u8,
    pub timestamp: DateTime<chrono::Utc>,
}
#[derive(Debug)]
pub struct FnSum<T> {
    pub id: String,
    pub input1: Rc<RefCell<Box<dyn TInOut<Point<T>, T>>>>,
    pub input2: Rc<RefCell<Box<dyn TInOut<Point<T>, T>>>>,
    pub status: u8,
    pub timestamp: DateTime<chrono::Utc>,
}
pub struct FnMul;
pub struct FnCompare;

#[derive(Debug)]
pub struct FnMetric<T> {
    pub input: Rc<RefCell<Box<dyn TInOut<Point<T>, T>>>>,
}

impl<I> TInOut<Point<I>, I> for FnInput<I> where 
    I: std::fmt::Debug + Clone {
    fn add(&mut self, point: Point<I>) {
        self.value = point.value;
        self.status = point.status;
        self.timestamp = point.timestamp;
        println!("FnInput({})<{}>.add | value: {:?}", self.id, std::any::type_name::<I>(), &self.value);
    }
    fn out(&self) -> I {
        println!("FnInput({})<{}>.out | value: {:?}", self.id, std::any::type_name::<I>(), &self.value);
        self.value.clone()
    }    
}


impl<T: std::fmt::Debug> TInput<T> for FnInput<T> {
    fn add(&mut self, point: Point<T>) {
        self.value = point.value;
        self.status = point.status;
        self.timestamp = point.timestamp;
        println!("FnInput({})<{}>.add | value: {:?}", self.id, std::any::type_name::<T>(), &self.value);
    }
}

impl<T: Debug + Clone> TOutput<T> for FnInput<T> {
    fn out(&self) -> T {
        println!("FnInput({})<{}>.out | value: {:?}", self.id, std::any::type_name::<T>(), &self.value);
        self.value.clone()
    }
}

impl<I: std::ops::Add<Output = I>> TInOut<Point<I>, I> for FnSum<I> where 
    I: std::fmt::Debug + Clone {
    fn add(&mut self, value: Point<I>) {
        println!("FnSum({})<{}>.add | value: --", self.id, std::any::type_name::<I>());
    }
    fn out(&self) -> I {
        let value1 = self.input1.borrow().out();
        let value2 = self.input2.borrow().out();
        let sum = value1 + value2;
        sum
    }
}

impl<T: std::fmt::Debug> TInput<T> for FnSum<T> {
    fn add(&mut self, value: Point<T>) {
        println!("FnSum({})<{}>.add | value: --", self.id, std::any::type_name::<T>());
    }
}
    
impl<T> TOutput<T> for FnSum<T> where
    T: Debug + std::ops::Add<Output = T> {
    fn out(&self) -> T {
        let value1 = self.input1.borrow().out();
        let value2 = self.input2.borrow().out();
        let sum = value1 + value2;
        sum
    }
}


impl<T: Debug + Display> TOutput<String> for FnMetric<T> {
    fn out(&self) -> String {
        let value = self.input.borrow().out();
        format!("insert into table values ({})", value)
    }
}
