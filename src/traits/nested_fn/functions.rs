use std::{fmt::{Display, Debug}, cell::RefCell, sync::Arc};

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
pub struct FnSum<T> {
    pub input1: RefCell<Box<dyn TOutput<T>>>,
    pub input2: RefCell<Box<dyn TOutput<T>>>,
    pub status: u8,
    pub timestamp: DateTime<chrono::Utc>,
}
pub struct FnMul;
pub struct FnCompare;

pub struct FnMetric<T> {
    pub input: RefCell<Box<dyn TOutput<T>>>,
}

pub trait TOutput<T> {
    fn out(&self) -> T;
}


pub trait TInput<T> {
    fn add(&mut self, point: Point<T>);
}


#[allow(non_snake_case)]

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

impl<T> TOutput<T> for FnSum<T> where
    T: std::ops::Add<Output = T> {
    fn out(&self) -> T {
        let value1 = self.input1.borrow().out();
        let value2 = self.input2.borrow().out();
        let sum = value1 + value2;
        sum
    }
}


impl<T: Display> TOutput<String> for FnMetric<T> {
    fn out(&self) -> String {
        let value = self.input.borrow().out();
        format!("insert into table values ({})", value)
    }
}
