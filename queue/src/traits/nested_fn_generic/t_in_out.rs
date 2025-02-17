use crate::traits::app_core::point::Point;



pub trait TOutput<T>: std::fmt::Debug {
    fn out(&self) -> T;
}


pub trait TInput<T>: std::fmt::Debug {
    fn add(&mut self, point: Point<T>);
}

pub trait TInOut<I, Q>: std::fmt::Debug {
    fn add(&mut self, point: I);
    fn out(&self) -> Q;
}
