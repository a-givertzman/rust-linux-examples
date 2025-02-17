#![allow(non_snake_case)]

trait DropDouble<T> {
    fn dropDouble(self, _: T);
}

impl<T, U> DropDouble<T> for U {
    fn dropDouble(self, _: T) {
        // here object of type T will be dropped
    }
}


struct A<T> {
    value: T,
}


fn main() {
    let a = A {value: 123.456};
    let aa = A {value: 123.456};
    a.dropDouble(aa);
    let b = A {value: 123.456};
    b.dropDouble(12);
}