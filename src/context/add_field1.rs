use std::{cell::RefCell, rc::Rc};
use nested_value::mut_value::MutValue;
use crate::{calc_context::{AddField1Results, CalcContext}, calc_eval::CalcEval};
///
/// 2 x Multiplications
pub struct AddField1<T, X> {
    id: String,
    exp: Box<dyn CalcEval<X, T>>,
}
//
//
impl<T, X> AddField1<T, X> {
    pub fn new(
        exp: impl CalcEval<X, T> + 'static,
    ) -> Self {
        Self {
            id: format!("AddField1"),
            exp: Box::new(exp),
        }
    }
    ///
    /// 
    pub fn eval(&mut self, context: Rc<RefCell<CalcContext>>) -> AddField1Results {
        let _ = self.exp.eval(context.clone());
        let context = context.borrow_mut();
        let add_field1 = context.src.field1.get();
        let field1 = context.results.mul2.field1.get();
        let field2 = context.results.mul2.field2.get();
        let field3 = context.results.mul2.field3.get();
        AddField1Results::new(
            MutValue::new(field1 + add_field1),
            MutValue::new(field2.iter().map(|value| value + add_field1).collect()),
            MutValue::new(field3 + add_field1),
        )
    }
}
//
//
impl<T, X> CalcEval<T, AddField1Results> for AddField1<T, X> {
    fn eval(&mut self, context: Rc<RefCell<CalcContext>>) -> AddField1Results {
        AddField1::eval(self, context)
    }
}
