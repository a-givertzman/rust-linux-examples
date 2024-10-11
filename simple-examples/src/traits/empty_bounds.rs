#![allow(non_snake_case)]

struct Cardinal;
struct BlueJay;
struct Turkey;

trait Red {}
trait Blue {}

impl Red for Cardinal {}
impl Blue for BlueJay {}

fn red<T: Red>(_: &T) -> &'static str {
    "red"
}

fn blue<T: Blue>(_: &T) -> &'static str {
    "blue"
}

fn main() {
    let cardinal = Cardinal;
    let blueJay = BlueJay;
    let _tutkey = Turkey;
    println!("Cardinal is : {:?}", red(&cardinal));
    println!("BlueJay is : {:?}", blue(&blueJay));
}