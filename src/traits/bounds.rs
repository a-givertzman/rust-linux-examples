use std::fmt::Debug;

trait HasArea {
    fn area(&self) -> f64;
}

#[derive(Debug)]
struct Rectangle {
    w: f64,
    h: f64,
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 {
        self.w * self.h
    }
}

#[derive(Debug)]
struct Triangle {
    length: f64,
    hight: f64,
}
impl HasArea for Triangle {
    fn area(&self) -> f64 {
        self.length * self.hight
    }
}

fn debug<T>(target: T) 
    where
        T: Debug + HasArea 
{
    println!("object: {:?}, area: {:?}", target, target.area());

}

fn main() {
    let rectangle = Rectangle { w: 3.0, h: 4.0 };
    let triangle = Triangle { length: 3.0, hight: 4.0 };
    debug(rectangle);
    debug(triangle);
}