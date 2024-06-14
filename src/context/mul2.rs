use std::{cell::RefCell, rc::Rc};
use nested_value::mut_value::MutValue;
use crate::{calc_context::{CalcContext, Mul2Results}, calc_eval::CalcEval};
///
/// 2 x Multiplications
pub struct Mul2<T, X> {
    id: String,
    exp: Box<dyn CalcEval<X, T>>,
}
//
//
impl<T, X> Mul2<T, X> {
    pub fn new(
        exp: Box<dyn CalcEval<X, T>>,
    ) -> Self {
        Self {
            id: format!("Mul2"),
            exp,
        }
    }
    ///
    /// 
    pub fn eval(&mut self, context: Rc<RefCell<CalcContext>>) -> Mul2Results {
        let _ = self.exp.eval(context.clone());
        let context = context.borrow_mut();
        let field1 = context.src.field1.get();
        let field2 = context.src.field2.get().unwrap();
        let field3 = context.src.field3.get();
        Mul2Results::new(
            MutValue::new(field1 * 2.0),
            MutValue::new(field2.iter().map(|(_key, value)| value * 2.0).collect()),
            MutValue::new(field3 * 2.0),
        )
    }
}
//
//
impl<T, X> CalcEval<T, Mul2Results> for Mul2<T, X> {
    fn eval(&mut self, context: Rc<RefCell<CalcContext>>) -> Mul2Results {
        Mul2::eval(self, context)
    }
}
