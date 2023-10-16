#![allow(non_snake_case)]

use std::ops::Add;
struct Foo;
struct Bar;
#[derive(Debug)]
struct FooBar;
#[derive(Debug)]
struct BarFoo;

impl Add<&Bar> for &Foo {
    type Output = FooBar;
    fn add(self, _rhs: &Bar) -> Self::Output {
        FooBar
    }
}

impl Add<Bar> for Foo {
    type Output = FooBar;
    fn add(self, _rhs: Bar) -> Self::Output {
        FooBar
    }
}

impl Add<Foo> for Bar {
    type Output = BarFoo;
    fn add(self, _rhs: Foo) -> Self::Output {
        BarFoo
    }
}

impl Add<&Foo> for &Bar {
    type Output = BarFoo;
    fn add(self, _rhs: &Foo) -> Self::Output {
        BarFoo
    }
}

fn main() {
let foo = Foo;
let bar = Bar;
let fooBar = Foo + Bar;
let barFoo = Bar + Foo;
println!("Foo + Bar: {:?}", &foo + &bar);
println!("Bar+ Foo: {:?}", bar + foo);
println!("fooBar: {:?}", fooBar);
println!("barFoo: {:?}", barFoo);
}