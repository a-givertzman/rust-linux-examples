use indexmap::IndexMap;
use nested_value::{const_value::ConstValue, fetch_value::FetchValue, mut_value::MutValue};

pub struct Task1Context {
    pub src: Task1Src,
    pub results: Task1Results,
}
//
//
impl Task1Context {
    pub fn new(src: Task1Src, results: Task1Results) -> Self {
        Self { src, results }
    }
}
///
/// 
pub struct Task1Src {
    pub const_f64_field1: ConstValue<f64>,
    pub const_map_field2: FetchValue<IndexMap<String, f64>>,
    pub mut_f64_field3: MutValue<f64>,
}
//
//
impl Task1Src {
    pub fn new(
        const_f64_field1: ConstValue<f64>,
        const_map_field2: FetchValue<IndexMap<String, f64>>,
        mut_f64_field3: MutValue<f64>,
    ) -> Self {
        Self {
            const_f64_field1,
            const_map_field2,
            mut_f64_field3,
        }
    }
}
///
/// 
pub struct Task1Results {
    pub mut_f64_field1: MutValue<f64>,
    pub mut_vec_field2: MutValue<Vec<f64>>,
    pub mut_f64_field3: MutValue<f64>,
}
//
//
impl Task1Results {
    pub fn new(
        mut_f64_field1: MutValue<f64>,
        mut_vec_field2: MutValue<Vec<f64>>,
        mut_f64_field3: MutValue<f64>,
        ) -> Self {
        Self {
            mut_f64_field1,
            mut_vec_field2,
            mut_f64_field3,
        }
    }
}
