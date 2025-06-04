///
/// Provide approximately equality for f36, f64
pub trait AproxEq<T> {
    ///
    /// Returns true if self & other rounded to specified digits are equals
    fn aprox_eq(&self, _other: T, _decimals: usize) -> bool {
        panic!("{}.aprox_eq | Not implemented yet",  self.type_of())
    }
    ///
    /// Returns true if self & other truncated to specified digits are equals
    #[allow(unused)]
    fn trunc_eq(&self, _other: T, _decimals: usize) -> bool {
        panic!("{}.trunc_eq | Not implemented yet", self.type_of())
    }
}
//
// 
impl AproxEq<f32> for f32 {
    fn aprox_eq(&self, other: f32, decimals: usize) -> bool {
        let factor = 10.0f64.powi(decimals as i32) as f32;
        let a = (self * factor).round();
        let b = (other * factor).round();
        a == b
    }
    fn trunc_eq(&self, other: f32, decimals: usize) -> bool {
        let factor = 10.0f64.powi(decimals as i32) as f32;
        let a = (self * factor).trunc();
        let b = (other * factor).trunc();
        a == b
    }
}
//
// 
impl AproxEq<f64> for f64 {
    fn aprox_eq(&self, other: f64, decimals: usize) -> bool {
        let factor = 10.0f64.powi(decimals as i32);
        let a = (self * factor).round();
        let b = (other * factor).round();
        a == b
    }
    fn trunc_eq(&self, other: f64, decimals: usize) -> bool {
        let factor = 10.0f64.powi(decimals as i32);
        let a = (self * factor).trunc();
        let b = (other * factor).trunc();
        a == b
    }
}
//
// 
impl AproxEq<&testing::entities::test_value::Value> for testing::entities::test_value::Value {
    fn aprox_eq(&self, other: &testing::entities::test_value::Value, decimals: usize) -> bool {
        match &self {
            testing::entities::test_value::Value::Bool(self_value) => {
                match other {
                    testing::entities::test_value::Value::Bool(other_value) => self_value == other_value,
                    _ => {
                        log::warn!("AproxEq<Value>.aprox_eq | Incompitable types self Value<Bool> and other {}", other.type_of());
                        false
                    }
                }
            }
            testing::entities::test_value::Value::Int(self_value) => {
                match other {
                    testing::entities::test_value::Value::Int(other_value) => self_value == other_value,
                    _ => {
                        log::warn!("AproxEq<Value>.aprox_eq | Incompitable types self Value<Int> and other {}", other.type_of());
                        false
                    }
                }
            }
            testing::entities::test_value::Value::Real(self_value) => {
                match other {
                    testing::entities::test_value::Value::Real(other_value) => self_value.aprox_eq(*other_value, decimals),
                    _ => {
                        log::warn!("AproxEq<Value>.aprox_eq | Incompitable types self Value<Real> and other {}", other.type_of());
                        false
                    }
                }
            }
            testing::entities::test_value::Value::Double(self_value) => {
                match other {
                    testing::entities::test_value::Value::Double(other_value) => self_value.aprox_eq(*other_value, decimals),
                    _ => {
                        log::warn!("AproxEq<Value>.aprox_eq | Incompitable types self Value<Double> and other {}", other.type_of());
                        false
                    }
                }
            }
            testing::entities::test_value::Value::String(self_value) => {
                match other {
                    testing::entities::test_value::Value::String(other_value) => self_value == other_value,
                    _ => {
                        log::warn!("AproxEq<Value>.aprox_eq | Incompitable types self Value<String> and other {}", other.type_of());
                        false
                    }
                }
            }
        }
    }
}

trait TypeOf<T> {
    fn type_of(&self) -> &str {
        std::any::type_name::<T>()
    }
}

impl<T> TypeOf<T> for T {

}