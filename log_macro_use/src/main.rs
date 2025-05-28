use log_macro::dbg;

// macro_rules! debug {
//     ($($arg:tt)*) => {
//         log::debug!($($arg)*)
//     };
// }


fn main() {
    println!("Hello, world!");
    let service = Service::new();
    service.run();
}

#[derive(Debug)]
struct Service {
    dbg: String,
}
//
//
impl Service {
    pub fn new() -> Self {
        Self {
            dbg: "Service".into(),
        }
    }
    #[dbg("dbg-custom-name")]
    pub fn run(&self) {
        for i in 0..3 {
            // println!("{}.{__fn_name} | i: {i}", self.dbg)
            log_macro::debug!(i);
            // log_macro::debug!("{}.{} | i: {}", self.dbg, __fn_name, i);
        }
    }
}