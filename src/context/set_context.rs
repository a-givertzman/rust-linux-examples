use std::{cell::RefCell, rc::Rc};
use crate::{calc_context::CalcContext, calc_eval::CalcEval};
///
/// 
pub struct SetContext<T> {
    set: Box<dyn Fn(Rc<RefCell<CalcContext>>, T)>,
    exp: Box<dyn CalcEval<T>>,
}
//
//
impl<T> SetContext<T> {
    pub fn new(
        set: Box<dyn Fn(Rc<RefCell<CalcContext>>, T)>,
        exp: Box<dyn CalcEval<T>>,
    ) -> Self {
        Self { set, exp}
    }
}
//
//
impl<T: Clone> CalcEval<T> for SetContext<T> {
    fn eval(&mut self, context: Rc<RefCell<CalcContext>>) -> T {
        let result = self.exp.eval(context.clone());
        (self.set)(context, result.clone());
        result
    }
}
