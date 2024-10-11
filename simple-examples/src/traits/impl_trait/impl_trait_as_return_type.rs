#![allow(non_snake_case)]

use std::{iter, vec::IntoIter};

fn combineVecsEcplisitReturnType(
    v: Vec<u32>,
    u: Vec<u32>,
) -> iter::Cycle<iter::Chain<IntoIter<u32>, IntoIter<u32>>> {
   v.into_iter().chain(u.into_iter()).cycle()
}

fn combineVecs<T: Clone>(
    v: Vec<T>,
    u: Vec<T>,
) ->impl Iterator<Item = T> {
    v.into_iter().chain(u.into_iter()).cycle()
}

fn main() {
    let v1 = vec![1, 2, 3];
    let v2 = vec![5, 6];
    let mut combined = combineVecsEcplisitReturnType(v1.clone(), v2.clone());
    println!("iterate using combineVecsEcplisitReturnType()...");
    println!("combined iten: {:?}", combined.next());
    println!("combined iten: {:?}", combined.next());
    println!("combined iten: {:?}", combined.next());
    println!("combined iten: {:?}", combined.next());
    println!("combined iten: {:?}", combined.next());
    println!("combined iten: {:?}", combined.next());
    println!("combined iten: {:?}", combined.next());

    let v1 = vec![1.0, 2.0, 3.0];
    let v2 = vec![5.0, 6.0];
    let mut combined = combineVecs(v1, v2);
    println!("iterate using combineVecs()...");
    println!("combined iten: {:?}", combined.next());
    println!("combined iten: {:?}", combined.next());
    println!("combined iten: {:?}", combined.next());
    println!("combined iten: {:?}", combined.next());
    println!("combined iten: {:?}", combined.next());
    println!("combined iten: {:?}", combined.next());
    println!("combined iten: {:?}", combined.next());
}