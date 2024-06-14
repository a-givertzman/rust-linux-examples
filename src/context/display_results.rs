use std::{cell::RefCell, rc::Rc};
use crate::{calc_context::CalcContext, calc_eval::CalcEval};
///
/// 
pub struct DisplayCalcResults<I, O> {
    id: String,
    label: String,
    exp: Box<dyn CalcEval<I, O>>,
}
//
//
impl<I, O> DisplayCalcResults<I, O> {
    pub fn new(
        label: impl Into<String>,
        exp: Box<dyn CalcEval<I, O>>,
    ) -> Self {
        Self {
            id: format!("DisplayCalcResults"),
            label: label.into(),
            exp,
        }
    }
    ///
    /// 
    pub fn eval(&mut self, context: Rc<RefCell<CalcContext>>) -> O {
        let result = self.exp.eval(context.clone());
        let context = context.borrow();
        println!("\n{}", self.label);
        println!("{}.eval results/field1: {}", self.id, context.results.mul2.field1.get());
        println!("{}.eval results/field2: {:#?}", self.id, context.results.mul2.field2.get());
        println!("{}.eval results/field3: {}", self.id, context.results.mul2.field3.get());
        result
    }
}
//
//
impl<I, O> CalcEval<I, O> for DisplayCalcResults<I, O> {
    fn eval(&mut self, context: Rc<RefCell<CalcContext>>) -> O {
        DisplayCalcResults::eval(self, context)
    }
}
