#![allow(non_snake_case)]

use std::marker::PhantomData;

trait TInput<T> {
    fn add(&self);
}

struct SomeInput<T> {
    inner: T, 
}

impl<i64, SomeInput<i64>> TInput<i64> for SomeInput<i64> {
    fn add(&self, value: i64) {
        self.inner = value;
    }
}


fn main() {
    let someInput = SomeInput { inner: 0 };
}