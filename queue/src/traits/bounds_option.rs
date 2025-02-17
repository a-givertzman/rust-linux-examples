#![allow(non_snake_case)]

use std::fmt::Debug;

trait PrintInOption {
    fn printInOption(&self);
}

#[derive(Debug)]
struct Rectangle {
    w: f64,
    h: f64,
}


impl<U> PrintInOption for U where
    U: Debug {
    fn printInOption(&self) {
        println!("Some(self): {:?}", Some(self));
    }
}

fn main() {
    let rectangle = Rectangle { w: 3.0, h: 4.0 };
    rectangle.printInOption();
    64.printInOption();
}