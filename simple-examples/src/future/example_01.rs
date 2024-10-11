use future::Future;

mod future;

fn main() {
    let calc = calc();
    let result = calc.then(Box::new(|event| {
        println!("event: {:?}", event);
        event        
    }));

    println!("result: {:?}", result);
}
///
/// 
fn calc() -> Future<Result<f64, String>> {
    Future::<Result<f64, String>>::new(Box::new(|| {
        let mut av = 0.0;
        for i in 0..100000 {
            let v = (i as f64) * (i as f64) * (i as f64) / 1000.0;
            av = av + v
        }
        println!("av: {:?}", av);
        Ok(av)
    }))
}