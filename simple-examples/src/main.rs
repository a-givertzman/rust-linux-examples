mod debug_session;
mod traits;


fn main() {
    println!("main");
    // let x = Int::new("ddd");
    let x = Int::new(777);
}


struct Int {
    val: i64,
}
impl Int {
    fn new(val: i64) -> Int {
        Int { val }
    }
}