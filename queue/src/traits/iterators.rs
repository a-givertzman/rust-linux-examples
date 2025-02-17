#![allow(non_snake_case)]

use std::{ops::Add, rc::Rc, cell::RefCell};

struct Fibonacci<T> where
    T: Copy + Add {
    curr: T,
    next: T,
}


impl<T> Iterator for Fibonacci<T> where
    T: Copy + Add<Output = T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let curr = self.curr;
        self.curr = self.next;
        self.next = curr + self.next;
        Some(curr)
    }
}

fn main() {
    let mut sequence = 0..3;
    let fibonacciF32 = Fibonacci { curr: 0.0, next: 1.0 };
    
    println!("iterate through 0..3 using for..");
    for i in sequence.clone() {
        println!("squence: {:?}", i);
    }

    println!("iterate through 0..3 using next()..");
    println!("squence: {:?}", sequence.next());
    println!("squence: {:?}", sequence.next());
    println!("squence: {:?}", sequence.next());
    println!("squence: {:?}", sequence.next());
    
    
    let fibonacciI32 = Fibonacci { curr: 0, next: 1 };
    println!("first 5 in fibonacci i32..");
    for i in fibonacciI32.take(5) {
        println!("fibonacci i32: {:?}", i);
    }
    let fibonacciI32 = Fibonacci { curr: 0, next: 1 };
    println!("next 5 in fibonacci i32..");
    for i in fibonacciI32.skip(5).take(5) {
        println!("fibonacci i32: {:?}", i);
    }

    println!("ferst 10 in fibonacci f32..");
    for i in fibonacciF32.take(10) {
        println!("fibonacci f32: {:?}", i);
    }
}