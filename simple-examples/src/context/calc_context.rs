use indexmap::IndexMap;
use nested_value::{const_value::ConstValue, fetch_value::FetchValue, mut_value::MutValue};
///
/// 
#[derive(Debug)]
pub struct CalcContext {
    pub src: CalcSrc,
    pub results: Results,
}
//
//
impl CalcContext {
    pub fn new(src: CalcSrc, results: Results) -> Self {
        Self { src, results }
    }
}
///
/// 
#[derive(Debug)]
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
#[derive(Clone, Debug)]
pub struct Results {
    pub mul2: Mul2Results,
    pub add_field1: AddField1Results,
}
//
//
impl Results {
    pub fn new(
        mul2: Mul2Results,
        add_field1: AddField1Results,
        ) -> Self {
        Self {
            mul2,
            add_field1,
        }
    }
}
///
/// 
#[derive(Clone, Debug)]
pub struct Mul2Results {
    pub field1: MutValue<f64>,
    pub field2: MutValue<Vec<f64>>,
    pub field3: MutValue<f64>,
}
//
//
impl Mul2Results {
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
///
/// 
#[derive(Clone, Debug)]
pub struct AddField1Results {
    pub field1: MutValue<f64>,
    pub field2: MutValue<Vec<f64>>,
    pub field3: MutValue<f64>,
}
//
//
impl AddField1Results {
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
