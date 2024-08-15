use std::{env, fs, io::Write};

use rand::Rng;

fn main() {
    env::set_var("RUST_LOG", "debug");
    env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let path = "src/multi_thread/test_data.txt";
    let count = 1_000;
    let template: Vec<String> = ["qwe", "rty", "uiop", "asdf", "ghj", "klz", "xcv", "bnm"].into_iter().map(|item| item.to_owned()).collect();

    let mut out = "".to_owned();
    let mut rng = rand::thread_rng();
    for item in 1..=count {
        let index = rng.gen_range(0..template.len());
        let mut value = template.clone();
        let k1: i32 = rng.gen_range(1..99);
        let k2: i32 = rng.gen_range(1..99);
        let k3: i32 = rng.gen_range(1..99);
        let k4: i32 = rng.gen_range(1..99);
        value[index] = format!("{:0>4}-{:0>4}-{:0>4}-{:0>4}", item * k1, item * k2, item * k3, item * k4);
        out.push_str(&value.join(" == "));
        out.push_str("\r\n");
    }
    // println!("{:#?}", out);
    let f = fs::OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(path);
    match f {
        Ok(mut f) => match f.write_all(out.as_bytes()) {
            Ok(_) => log::info!("TestData | test data generated"),
            Err(err) => log::info!("TestData | Write error: {:#?}", err),
        }
        Err(err) => log::info!("TestData | Open error: {:#?}", err),
    }
}