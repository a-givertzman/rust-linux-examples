use std::{collections::HashMap, rc::Rc, cell::RefCell};

use crate::traits::{
    app_core::{point::Point, bool::Bool},
    nested_fn_generic::t_in_out::TInOut,
};


///
/// A container for storing FnInput by name
#[derive(Debug)]
pub struct FnInputs {
    refs: HashMap<String, InputType>,
}
impl FnInputs {
    ///
    /// Creates new container for storing FnInput
    pub fn new() -> Self {
        Self {
            refs: HashMap::new()
        }
    }
    ///
    /// Adding new input refeerence
    pub fn add(&mut self, name: impl Into<String>, input: InputType) {
        self.refs.insert(name.into(), input);
    }
    ///
    /// Adding new Bool input refeerence
    pub fn addBool(&mut self, name: impl Into<String>, input: Rc<RefCell<Box<dyn TInOut<Point<Bool>, Bool>>>>) {
        self.refs.insert(name.into(), InputType::Bool(input));
    }
    ///
    /// Adding new Int input refeerence
    pub fn addInt(&mut self, name: impl Into<String>, input: Rc<RefCell<Box<dyn TInOut<Point<i64>, i64>>>>) {
        self.refs.insert(name.into(), InputType::Int(input));
    }
    ///
    /// Adding new Float input refeerence
    pub fn addFloat(&mut self, name: impl Into<String>, input: Rc<RefCell<Box<dyn TInOut<Point<f64>, f64>>>>) {
        self.refs.insert(name.into(), InputType::Float(input));
    }
    ///
    /// Returns input by it's name
    pub fn get(&self, name: &str) -> &InputType {
        self.refs.get(name.into()).unwrap()
    }
    ///
    /// Returns input::Bool by it's name
    pub fn getBool(&self, name: &str) -> Rc<RefCell<Box<dyn TInOut<Point<Bool>, Bool>>>> {
        match self.refs.get(name.into()) {
            Some(input) => {
                match input {
                    InputType::Bool(input) => input.clone(),
                    _ => panic!("invalid type Bool of requested input {:?}", name),
                }
            },
            None => panic!("Unknown input name {:?}", name),
        }
    }
    ///
    /// Returns input::Int by it's name
    pub fn getInt(&self, name: &str) -> Rc<RefCell<Box<dyn TInOut<Point<i64>, i64>>>> {
        match self.refs.get(name.into()) {
            Some(input) => {
                match input {
                    InputType::Int(input) => input.clone(),
                    _ => panic!("invalid type Int of requested input {:?}", name),
                }
                
            },
            None => panic!("Unknown input name {:?}", name),
        }
    }
    ///
    /// Returns input::Float by it's name
    pub fn getFloat(&self, name: &str) -> Rc<RefCell<Box<dyn TInOut<Point<f64>, f64>>>> {
        match self.refs.get(name.into()) {
            Some(input) => {
                match input {
                    InputType::Float(input) => input.clone(),
                    _ => panic!("invalid type Float of requested input {:?}", name),
                }                
            },
            None => panic!("Unknown input name {:?}", name),
        }
    }
}

#[derive(Debug)]
pub enum InputType {
    Bool(Rc<RefCell<Box<dyn TInOut<Point<Bool>, Bool>>>>),
    Int(Rc<RefCell<Box<dyn TInOut<Point<i64>, i64>>>>),
    Float(Rc<RefCell<Box<dyn TInOut<Point<f64>, f64>>>>),
}
