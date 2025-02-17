use crate::traits::app_core::point::PointType;


pub trait FnIn: std::fmt::Debug {
    fn add(&mut self, point: PointType);
}

pub trait FnOut: std::fmt::Debug {
    fn out(&self) -> PointType;
}

pub trait FnInOut: FnIn + FnOut {
    
}