#![allow(non_snake_case)]

#[path = "../debug_session/mod.rs"]
mod debug_session;
#[path = "../traits/mod.rs"]
mod traits;

use std::{cell::RefCell, rc::Rc, fmt::Debug, collections::HashMap};

use traits::{
    nested_fn_generic::t_in_out::TInOut,
    app_core::{
        point::{Point, PointType},
        bool::Bool,
    }
};

use crate::traits::nested_fn_generic::fn_inputs::{FnInputs, InputType};



#[derive(Debug)]
struct Mutable<T> {
    inner: T,
}
impl<T: Debug + Clone + std::ops::Add<Output = T>> TInOut<Point<T>, T> for Mutable<T> {
    fn add(&mut self, point: Point<T>) {
        self.inner = point.value;
    }
    fn out(&self) -> T {
        todo!()
    }
}


fn fnAdd<T: Debug + Clone + std::ops::Add<Output = T> + 'static>(initial: T) -> Rc<RefCell<Box<dyn TInOut<Point<T>, T>>>> {
    Rc::new(RefCell::new(
        Box::new(
            Mutable {inner: initial}
        )
    ))
}

fn points() ->Vec<PointType> {
    vec![
        PointType::Bool(  Point { value:Bool(true),   name:String::from("bool1"),  status: 0, timestamp: chrono::offset::Utc::now() }),
        PointType::Bool(  Point { value:Bool(false),  name:String::from("bool1"),  status: 0, timestamp: chrono::offset::Utc::now() }),
        PointType::Int(   Point { value:1,     name:String::from("int1"),   status: 0, timestamp: chrono::offset::Utc::now() }),
        PointType::Int(   Point { value:2,     name:String::from("int1"),   status: 0, timestamp: chrono::offset::Utc::now() }),
        PointType::Int(   Point { value:3,     name:String::from("int1"),   status: 0, timestamp: chrono::offset::Utc::now() }),
        PointType::Float( Point {value: 1.1,  name:String::from("input1"), status: 0, timestamp: chrono::offset::Utc::now() }),
        PointType::Float( Point {value: 2.2,  name:String::from("input1"), status: 0, timestamp: chrono::offset::Utc::now() }),
        PointType::Float( Point {value: 3.3,  name:String::from("input1"), status: 0, timestamp: chrono::offset::Utc::now() }),
    ]
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
    let ref2 = fnAdd(1.1);
    refs.insert("ref2", InputType::Float(ref2.clone()));
    inputs.addBool("ref0", ref0.clone());
    inputs.addInt("ref1", ref1.clone());
    inputs.addFloat("ref2", ref2.clone());
    {
        println!("ref1: {:?}", ref1);
        let ref2 = ref1.clone();
        let mut r = ref2.borrow_mut();
        r.add(points()[2].asInt());
        println!("ref2: {:?}", &ref2);
    }
    println!("\n");
    println!("ref0: {:?}", ref0);
    println!("ref1: {:?}", ref1);
    println!("ref2: {:?}", ref2);

    for (_, input) in refs {
        match input {
            InputType::Bool(input) => input.borrow_mut().add(points()[0].asBool()),
            InputType::Int(input) => input.borrow_mut().add(points()[3].asInt()),
            InputType::Float(input) => input.borrow_mut().add(points()[6].asFloat()),
        };
    }

    println!("\n");
    println!("Chanjed in the HasMap");
    println!("ref0: {:?}", ref0);
    println!("ref1: {:?}", ref1);
    println!("ref2: {:?}", ref2);

    inputs.getBool("ref0").borrow_mut().add(points()[1].asBool());
    inputs.getInt("ref1").borrow_mut().add(points()[4].asInt());
    inputs.getFloat("ref2").borrow_mut().add(points()[7].asFloat());

    println!("\n");
    println!("Chanjed in the FnInputs");
    println!("ref0: {:?}", ref0);
    println!("ref1: {:?}", ref1);
    println!("ref2: {:?}", ref2);
}
