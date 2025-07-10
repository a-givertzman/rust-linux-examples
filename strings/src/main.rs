mod concat_test;
mod concat_string_test;
mod domain;

use concat_test::*;
use concat_string_test::*;
pub use domain::*;

///
/// 
fn main() {
    unsafe { std::env::set_var("RUST_LOG", "info") };
    env_logger::init();
    let count = 10_000;
    let tests: &[(i32, Box<dyn Test>)] = &[
        (02, Box::new(ConcatStringTest::new(""))),
        (01, Box::new(ConcatTest::new(""))),
    ];
    for (step, test) in tests {
        match test.run(count) {
            Ok(result) => log::info!("main | test {step}: \n{:?}", result),
            Err(err) => log::info!("main | test {step}: Error: {:?}", err),
        }
    }
}
