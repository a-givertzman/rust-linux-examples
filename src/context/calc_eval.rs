use std::{cell::RefCell, rc::Rc};

use crate::calc_context::CalcContext;

///
/// 
pub trait CalcEval<T> {
    fn eval(&mut self, context: Rc<RefCell<CalcContext>>) -> T;
}
