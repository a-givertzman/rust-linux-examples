#![allow(non_snake_case)]

use std::ops::Add;

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
    let fibonacciI32 = Fibonacci { curr: 0, next: 1 };
    let fibonacciF32 = Fibonacci { curr: 0.0, next: 1.0 };
    for i in sequence.clone() {
        println!("squence: {:?}", i);
    }
    println!("squence: {:?}", sequence.next());
    println!("squence: {:?}", sequence.next());
    println!("squence: {:?}", sequence.next());
    println!("squence: {:?}", sequence.next());

    for i in fibonacciI32.take(10) {
        println!("fibonacci i32: {:?}", i);
    }

    for i in fibonacciF32.take(10) {
        println!("fibonacci f32: {:?}", i);
    }
}