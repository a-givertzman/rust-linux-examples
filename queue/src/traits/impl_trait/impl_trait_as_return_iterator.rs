#![allow(non_snake_case)]

fn doublePos<'a>(numbers: &'a Vec<i32>) ->impl Iterator<Item = i32> + 'a {
    numbers
        .iter()
        .filter(|item| {
            item >= &&0
        })
        .map(|item| {item * 2})
}

fn main() {
    let sequence = vec![2, 1, 0, -1, -2, -3];
    let dpIterator = doublePos(&sequence);
    println!("source sequence: {:?}", sequence);
    let dp = dpIterator.collect::<Vec<i32>>();
    println!("double positives: {:?}", dp);
    assert_eq!(dp, vec![4, 2, 0]);
}