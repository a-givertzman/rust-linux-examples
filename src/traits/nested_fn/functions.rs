#![allow(non_snake_case)]

use std::{fmt::{Display, Debug}, cell::RefCell, rc::Rc};

use chrono::DateTime;


#[derive(Clone, Debug)]
pub struct Point<T> {
    pub name: String,
    pub value: T,
    pub status: u8,
    pub timestamp: DateTime<chrono::Utc>,
}

#[derive(Debug, Clone)]
pub enum PointType {
    Bool(Point<bool>),
    Int(Point<i64>),
    Float(Point<f64>),
}
impl PointType {
    pub fn name(&self) -> String {
        match self {
            PointType::Bool(point) => point.name.clone(),
            PointType::Int(point) => point.name.clone(),
            PointType::Float(point) => point.name.clone(),
        }
    }
}

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
    pub input1: Rc<RefCell<Box<dyn TInOut<T, T>>>>,
    pub input2: Rc<RefCell<Box<dyn TInOut<T, T>>>>,
    pub status: u8,
    pub timestamp: DateTime<chrono::Utc>,
}
pub struct FnMul;
pub struct FnCompare;

#[derive(Debug)]
pub struct FnMetric<T> {
    pub input: Rc<RefCell<Box<dyn TInOut<T, T>>>>,
}

pub trait TOutput<T>: Debug {
    fn out(&self) -> T;
}


pub trait TInput<T>: Debug {
    fn add(&mut self, point: Point<T>);
}

pub trait TInOut<I, Q>: Debug {
    fn add(&mut self, point: Point<I>);
    fn out(&self) -> Q;
}


impl<I> TInOut<I, I> for FnInput<I> where 
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

impl<I: std::ops::Add<Output = I>> TInOut<I, I> for FnSum<I> where 
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
