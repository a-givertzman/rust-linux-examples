#![allow(non_snake_case)]

use std::{cell::RefCell, rc::Rc, fmt::Debug};

trait TInOut<T>: Debug {
    fn add(&mut self, value: T) {

    }
}



#[derive(Debug)]
struct Mutable<T> {
    inner: T,
}
impl<T: Debug + Clone + std::ops::Add<Output = T>> TInOut<T> for Mutable<T> {
    fn add(&mut self, value: T) {
        self.inner = self.inner.clone() + value;
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
    let ref1 = fnAdd(0.123);
    {
        println!("ref1: {:?}", ref1);
        let ref2 = ref1.clone();
        let mut r = ref2.borrow_mut();
        r.add(1.0);
        println!("ref2: {:?}", &ref2);
    }
    // ref1.borrow_mut() = .add(1);

    println!("ref1: {:?}", ref1);
    // println!("ref2: {:?}", ref2);


}
