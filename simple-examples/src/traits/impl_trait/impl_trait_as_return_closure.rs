#![allow(non_snake_case)]

use std::fmt::Debug;


fn getFunction<T: Debug>(name: &str) ->impl Fn(T) {
    match name {
        "bool" => move |v| {println!("bool closure received: {:?}", v)},
        "int" => move |v| {println!("int closure received: {:?}", v)},
        "float" => move |v| {println!("float closure received: {:?}", v)},
        _ => panic!("Not implemented")
    }
    // let closure = move |v| {println!("closure received: {:?}", v)};
    // closure
}

fn main() {
    let boolFn = getFunction("bool");
    println!("get function result:");
    boolFn(true);

    let intFn = getFunction("int");
    println!("get function result:");
    intFn(44);

    let floatFn = getFunction("float");
    println!("get function result:");
    floatFn(44.44);
}