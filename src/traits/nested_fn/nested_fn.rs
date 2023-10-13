use std::{collections::HashMap, fmt::Debug};

#[derive(Clone)]
struct FnInput<T> {
    value: T
}
struct FnSum;
struct FnMul;
struct FnCompare;

trait TOutput<T> {
    fn out(&self) -> T;
}


trait TInput<T> {
    fn add(&mut self, value: T);
}


impl<T: Debug> TInput<T> for FnInput<T> {
    fn add(&mut self, value: T) {
        self.value = value;
        println!("FnInput<{}>.add | value: {:?}", std::any::type_name::<T>(), &self.value)
    }
}


fn main() {
    let mut inputs: HashMap<String, FnType> = HashMap::from([
        (String::from("float1"), FnType::Float( FnInput { value: 0.0 } )), 
        (String::from("int1"), FnType::Int( FnInput { value: 0 } )), 
        (String::from("bool1"), FnType::Bool( FnInput { value: false } )), 
    ]);
    // let inputs = RefCell::new(inputs);

    
    let queue = vec![
        PointType::Bool(Point {value:true, name: String::from("bool1") }),
        PointType::Int(Point {value:13, name: String::from("int1") }),
        PointType::Int(Point {value:43, name: String::from("int1") }),
        PointType::Bool(Point {value:false, name: String::from("bool1") }),
        PointType::Float(Point {value:12.77, name: String::from("float1") }),
        PointType::Int(Point {value:65, name: String::from("int1") }),
    ];
    for point in queue {
        match point {
            PointType::Bool(point) => {
                let input = inputs.get_mut(&point.name);
                if input.is_some() {
                    match input.unwrap() {
                        FnType::Bool(i) => {
                            i.add(point.value);
                            println!("FnInput.value: {:?}", i.value);
                        },
                        _ => panic!("wrong type"),
                    };
                }
            },
            PointType::Int(point) => {
                let input = inputs.get_mut(&point.name);
                if input.is_some() {
                    match input.unwrap() {
                        FnType::Int(i) => {
                            i.add(point.value);
                            println!("FnInput.value: {:?}", i.value);
                        },
                        _ => panic!("wrong type"),
                    };                
                }
            },
            PointType::Float(point) => {
                let input = inputs.get_mut(&point.name);
                if input.is_some() {
                    match input.unwrap() {
                        FnType::Float(i) => {
                            i.add(point.value);
                            println!("FnInput.value: {:?}", i.value);
                        },
                        _ => panic!("wrong type"),
                    };                
                }
            },
        }
    }
}


struct Point<T> {
    name: String,
    value: T,
}

enum PointType {
    Bool(Point<bool>),
    Int(Point<i64>),
    Float(Point<f64>),
}

enum FnType {
    Bool(FnInput<bool>),
    Int(FnInput<i64>),
    Float(FnInput<f64>),
}