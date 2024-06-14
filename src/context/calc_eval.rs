use std::{cell::RefCell, rc::Rc};
use crate::calc_context::CalcContext;
///
/// 
pub trait CalcEval<I, O> {
    fn eval(&mut self, context: Rc<RefCell<CalcContext>>) -> O;
}
