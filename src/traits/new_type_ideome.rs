#![allow(non_snake_case)]

struct Secs(f64);
#[derive(std::fmt::Debug)]
struct Milles(f64);
struct Micros(f64);

impl ToMilles for Secs {
    fn toMilles(&self) -> Milles {
        Milles(self.0 * 1000.0)
    }
}
impl ToMilles for Milles {
    fn toMilles(&self) -> Milles {
        Milles(self.0)
    }
}
impl ToMilles for Micros {
    fn toMilles(&self) -> Milles {
        Milles(self.0 / 1000.0)
    }
}

trait ToMilles {
    fn toMilles(&self) -> Milles;
}

fn withDurationInMilles(duration: &Milles) {
    println!("duration in milles: {:?}", duration);
}

fn main() {
    let secs = Secs(1.2);
    let milles = Milles(12.0);
    let micros = Micros(724.0);
    withDurationInMilles(&secs.toMilles());
    withDurationInMilles(&milles);
    withDurationInMilles(&micros.toMilles());
}