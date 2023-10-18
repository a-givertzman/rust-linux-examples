#![allow(non_snake_case)]

#[path = "../debug_session/mod.rs"]
mod debug_session;
#[path = "../traits/mod.rs"]
mod traits;

use std::{cell::RefCell, rc::Rc, fmt::Debug, collections::HashMap};

use traits::nested_fn::t_in_out::TInOut;

use crate::traits::nested_fn::{fn_inputs::{FnInputs, InputType}, bool::Bool};



#[derive(Debug)]
struct Mutable<T> {
    inner: T,
}
impl<T: Debug + Clone + std::ops::Add<Output = T>> TInOut<T, T> for Mutable<T> {
    fn add(&mut self, value: T) {
        self.inner = value;
    }
    fn out(&self) -> T {
        todo!()
    }
}


fn fnAdd<T: Debug + Clone + std::ops::Add<Output = T> + 'static>(initial: T) -> Rc<RefCell<Box<dyn TInOut<T, T>>>> {
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
    inputs.getFloat("ref2").borrow_mut().add(3.3);

    println!("\n");
    println!("Chanjed in the FnInputs");
    println!("ref0: {:?}", ref0);
    println!("ref1: {:?}", ref1);
    println!("ref2: {:?}", ref2);
}
