use indexmap::IndexMap;
use nested_value::{const_value::ConstValue, fetch_value::FetchValue, mut_value::MutValue};
///
/// 
pub struct CalcContext {
    pub src: CalcSrc,
    pub results: CalcResults,
}
//
//
impl CalcContext {
    pub fn new(src: CalcSrc, results: CalcResults) -> Self {
        Self { src, results }
    }
}
//
//
// impl CalcEval for Rc<RefCell<CalcContext>> {
//     fn eval(&mut self) -> Rc<RefCell<CalcContext>> {
//         self.clone()
//     }
// }
///
/// 
pub struct CalcSrc {
    pub field1: ConstValue<f64>,
    pub field2: FetchValue<IndexMap<String, f64>>,
    pub field3: ConstValue<f64>,
}
//
//
impl CalcSrc {
    pub fn new(
        field1: ConstValue<f64>,
        field2: FetchValue<IndexMap<String, f64>>,
        field3: ConstValue<f64>,
    ) -> Self {
        Self {
            field1,
            field2,
            field3,
        }
    }
}
///
/// 
pub struct CalcResults {
    pub field1: MutValue<f64>,
    pub field2: MutValue<Vec<f64>>,
    pub field3: MutValue<f64>,
}
//
//
impl CalcResults {
    pub fn new(
        field1: MutValue<f64>,
        field2: MutValue<Vec<f64>>,
        field3: MutValue<f64>,
        ) -> Self {
        Self {
            field1,
            field2,
            field3,
        }
    }
}
