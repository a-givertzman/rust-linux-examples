mod cache;
mod field;
mod pair;

use std::time::Instant;
use field::Field;
use pair::Pair;

macro_rules! fields(
    { $($key:ident: $value:expr),+ } => {
        {
            let mut m = ::indexmap::IndexMap::new();
            $(
                m.insert(stringify!($key), $value);
            )+
            m
        }
     };
);


///
/// 
fn main() {
    unsafe { std::env::set_var("RUST_LOG", "info") };
    env_logger::init();
    let path = "config.yaml";
    let rdr = std::fs::OpenOptions::new().read(true).open(path).unwrap();

    let total_time = Instant::now();
    
    let r = fields!{
        arg1: 1.0,
        arg2: 0.0,
        arg3: 0.0,
        arg4: 0.0
    };


    let test_data = [
        (01, 0.1, vec![Pair::new(1, 2), Pair::new(5, 6)]),
        (02, 0.2, vec![Pair::new(0, 1), Pair::new(0, 4)]),
        (03, 0.3, vec![Pair::new(0, 1), Pair::new(0, 1)]),
        (04, 0.4, vec![Pair::new(0, 1), Pair::new(0, 1)]),
        (05, 0.0, vec![Pair::new(0, 1), Pair::new(0, 1)]),
    ];

    let field: Field<f64> = Field::new(vec![0.0, 0.1, 0.2, 0.3, 0.2, 0.1, 0.0, -0.1]);
    for (step, val, target) in test_data {
        let result = field.get(val);
        log::info!("main | step {step}  val: {:?} => Pairs: {:?}", r, result);
    }
    
    let total_elapsed = total_time.elapsed();
    log::info!("main | math cache ");
    log::info!("main | ---------------------------");
    log::info!("main | args: {:?}", r);
    log::info!("main | ---------------------------");
    // log::info!("main | Producers: {:?}", producers.len());
    // log::info!("main | Total produced: {:?}", total_produced);
    // log::info!("main | ---------------------------");
    // log::info!("main | Receivers: {:?}", receivers.len());
    // log::info!("main | Total received: {:?}", total_received);
    // log::info!("main | ---------------------------");
    // log::info!("main | Loads: {:?}", loads.len());
    // log::info!("main | ---------------------------");
    log::info!("main | Total elapsed: {:?}", total_elapsed);
}
