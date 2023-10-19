use std::{rc::Rc, cell::RefCell};

use crate::traits::{
    nested_fn_enum::{
        conf::Conf,
        functions::{
            FnInput,
            FnSum,
        },
        t_in_out::FnInOut,
        fn_inputs::FnInputs,
    }, 
    app_core::point::PointType
};

///
/// Creates nested functions tree from it config
pub struct NestedFn {}
impl NestedFn {
    ///
    /// Creates nested functions tree from it config
    pub fn new(conf: &mut Conf, initial: PointType, inputs: &mut FnInputs) -> Rc<RefCell<Box<dyn FnInOut>>> {
        Self::function(conf, initial, String::new(), inputs)
    }
    ///
    /// 
    fn boxFnInput(input: FnInput) -> Box<(dyn FnInOut)> {
        Box::new(input)
    }
    fn fnInput(inputName: String, initial: PointType) -> Rc<RefCell<Box<dyn FnInOut>>> {
        Rc::new(RefCell::new(
            Self::boxFnInput(
                FnInput { 
                    id: inputName.clone(),
                    point: initial, 
                }
            )
        ))
    }
    ///
    /// 
    fn boxFnSum(input: FnSum) -> Box<(dyn FnInOut)> {
        Box::new(input)
    }
    fn fnSum(
        inputName: String, 
        input1: Rc<RefCell<Box<dyn FnInOut>>>, 
        input2: Rc<RefCell<Box<dyn FnInOut>>>
    ) -> Rc<RefCell<Box<dyn FnInOut>>> {
        Rc::new(RefCell::new(
            Self::boxFnSum(        
                FnSum {
                    id: inputName,
                    input1: input1, 
                    input2: input2, 
                }
            )
        ))
    }
    ///
    /// 
    fn function(conf: &mut Conf, initial: PointType, inputName: String, inputs: &mut FnInputs) -> Rc<RefCell<Box<dyn FnInOut>>> {
        match conf.name().as_str() {
            "input" => {
                println!("input function {:?}...", inputName);
                let input = Self::fnInput(inputName.clone(), initial);
                inputs.add(inputName, input.clone());
                // let a = input.borrow_mut();
                println!("input function: {:?}", input);
                input
            },
            "sum" => {
                println!("sum function");
                let in1Name = String::from("input1");
                let in2Name = String::from("input2");
                let input1 = Self::function(conf.nested(&in1Name), initial.clone(), in1Name, inputs);
                let input2 = Self::function(conf.nested(&in2Name), initial, in2Name, inputs);
                let func = Self::fnSum(inputName, input1, input2);
                func
            }
            _ => panic!("Unknown function name: {:?}", conf.name())
        }
    }
}
