#![allow(non_snake_case)]

#[derive(Debug, Clone, Copy)]
struct Unit<T>(T);

#[derive(Debug, Clone)]
struct Pair(Box<i32>, Box<i32>);

fn main() {
    let unit = Unit("origin");
    let copiedUnit = unit;
    println!("origin unit: {:?}", unit);
    println!("cpied unit: {:?}", copiedUnit);

    let pair = Pair(Box::new(12), Box::new(17));
    println!("origin pair: {:?}", pair);
    let movedPair = pair;
    // println!("origin pair: {:?}", pair);    // gives compirer error, because pair already moved
    println!("moved pair: {:?}", movedPair);
    let clonedPair = movedPair.clone();
    println!("cloned pair: {:?}", clonedPair);
    println!("drop the moved par...");
    drop(movedPair);
    // println!("moved pair: {:?}", movedPair);    // gives a cpmpiler error, because movedPair is droped
    println!("cloned pair steel live: {:?}", clonedPair);
}