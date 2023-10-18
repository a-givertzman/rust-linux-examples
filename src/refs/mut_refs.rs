#![allow(non_snake_case)]

use std::{cell::RefCell, rc::Rc, fmt::Debug, collections::HashMap};

pub trait TInOut<T>: Debug {
    fn add(&mut self, value: T) {

    }
}

#[derive(Debug, Clone)]
pub struct Bool(bool);
impl std::ops::Add for Bool {
    type Output = Bool;
    fn add(self, rhs: Self) -> Self::Output {
        Bool(self.0 | rhs.0)
    }
}

#[derive(Debug)]
struct Mutable<T> {
    inner: T,
}
impl<T: Debug + Clone + std::ops::Add<Output = T>> TInOut<T> for Mutable<T> {
    fn add(&mut self, value: T) {
        self.inner = value;
    }
}


fn fnAdd<T: Debug + Clone + std::ops::Add<Output = T> + 'static>(initial: T) -> Rc<RefCell<Box<dyn TInOut<T>>>> {
    Rc::new(RefCell::new(
        Box::new(
            Mutable {inner: initial}
        )
    ))
}

fn main() {
    // let mutable = Mutable {inner: 123.0};
    // let ref1 = Rc::new(RefCell::new(Box::new(mutable)));
    let mut inputs = FnInputs::new();
    let mut refs = HashMap::new();
    let ref0 = fnAdd(Bool(false));
    refs.insert("ref0", InputType::Bool(ref0.clone()));
    let ref1 = fnAdd(123);
    refs.insert("ref1", InputType::Int(ref1.clone()));
    let ref2 = fnAdd(0.123);
    refs.insert("ref2", InputType::Float(ref2.clone()));
    inputs.addBool("ref0", ref0.clone());
    inputs.addInt("ref1", ref1.clone());
    inputs.addFloat("ref2", ref2.clone());
    {
        println!("ref1: {:?}", ref1);
        let ref2 = ref1.clone();
        let mut r = ref2.borrow_mut();
        r.add(1);
        println!("ref2: {:?}", &ref2);
    }
    println!("\n");
    println!("ref0: {:?}", ref0);
    println!("ref1: {:?}", ref1);
    println!("ref2: {:?}", ref2);

    for (_, input) in refs {
        match input {
            InputType::Bool(input) => input.borrow_mut().add(Bool(true)),
            InputType::Int(input) => input.borrow_mut().add(2),
            InputType::Float(input) => input.borrow_mut().add(2.2),
        };
    }

    println!("\n");
    println!("Chanjed in the HasMap");
    println!("ref0: {:?}", ref0);
    println!("ref1: {:?}", ref1);
    println!("ref2: {:?}", ref2);

    inputs.getBool("ref0").borrow_mut().add(Bool(false));
    inputs.getInt("ref1").borrow_mut().add(3);
    inputs.getFloat("ref3").borrow_mut().add(3.3);

    println!("\n");
    println!("Chanjed in the FnInputs");
    println!("ref0: {:?}", ref0);
    println!("ref1: {:?}", ref1);
    println!("ref2: {:?}", ref2);
}

///
/// A container for storing FnInput by name
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
    pub fn addBool(&mut self, name: impl Into<String>, input: Rc<RefCell<Box<dyn TInOut<Bool>>>>) {
        self.refs.insert(name.into(), InputType::Bool(input));
    }
    ///
    /// Adding new Int input refeerence
    pub fn addInt(&mut self, name: impl Into<String>, input: Rc<RefCell<Box<dyn TInOut<i64>>>>) {
        self.refs.insert(name.into(), InputType::Int(input));
    }
    ///
    /// Adding new Float input refeerence
    pub fn addFloat(&mut self, name: impl Into<String>, input: Rc<RefCell<Box<dyn TInOut<f64>>>>) {
        self.refs.insert(name.into(), InputType::Float(input));
    }
    ///
    /// Returns input by it's name
    pub fn get(&self, name: &str) -> &InputType {
        self.refs.get(name.into()).unwrap()
    }
    ///
    /// Returns input::Bool by it's name
    pub fn getBool(&self, name: &str) -> Rc<RefCell<Box<dyn TInOut<Bool>>>> {
        match self.refs.get(name.into()) {
            Some(input) => {
                match input {
                    InputType::Bool(input) => input.clone(),
                    _ => panic!("invalid input {:?} type BOOL, ", name),
                }
            },
            None => panic!("Unknown input name {:?}", name),
        }
    }
    ///
    /// Returns input::Int by it's name
    pub fn getInt(&self, name: &str) -> Rc<RefCell<Box<dyn TInOut<i64>>>> {
        match self.refs.get(name.into()) {
            Some(input) => {
                match input {
                    InputType::Int(input) => input.clone(),
                    _ => panic!("invalid input {:?} type INT, ", name),
                }
                
            },
            None => panic!("Unknown input name {:?}", name),
        }
    }
    ///
    /// Returns input::Float by it's name
    pub fn getFloat(&self, name: &str) -> Rc<RefCell<Box<dyn TInOut<f64>>>> {
        match self.refs.get(name.into()) {
            Some(input) => {
                match input {
                    InputType::Float(input) => input.clone(),
                    _ => panic!("invalid input {:?} type FLOAT, ", name),
                }                
            },
            None => panic!("Unknown input name {:?}", name),
        }
    }
}

pub enum InputType {
    Bool(Rc<RefCell<Box<dyn TInOut<Bool>>>>),
    Int(Rc<RefCell<Box<dyn TInOut<i64>>>>),
    Float(Rc<RefCell<Box<dyn TInOut<f64>>>>),
}
