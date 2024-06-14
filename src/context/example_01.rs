use std::{cell::RefCell, rc::Rc};
use api_tools::{api::reply::api_reply::ApiReply, client::{api_query::{ApiQuery, ApiQueryKind, ApiQuerySql}, api_request::ApiRequest}};
use calc_eval::CalcEval;
use indexmap::IndexMap;
use nested_value::{const_value::ConstValue, fetch_value::FetchValue, mut_value::MutValue};
use calc_context::{CalcContext, CalcResults, CalcSrc, Results};
use set_context::SetContext;
mod calc_eval;
mod calc_context;
mod set_context;
///
/// 
fn main() {
    let self_id = "main";
    let calc_context = prepare_task1_context(self_id);
    let mut calc = DisplayCalcResults::new(
        "Results after calc",
        Box::new(SetContext::new(
            Box::new(|context, result| {
                context.borrow_mut().results = result;
            }),
            Box::new(Calc::new(
                Box::new(DisplayCalcResults::new(
                    "Results before calc",
                    Box::new(SetContext::new(
                        Box::new(|context, result| {
                            // context.borrow_mut().results.
                        }),
                        Box::new(DisplayCalcSrc::new(
                            "~~~~~~~~~~   Src   ~~~~~~~~~~",
                            Box::new(Start::new())
                        )),
                    ))
                )),
            )),
        )),
    );
    let calc_context = calc.eval(calc_context);
    _ = calc_context;
}
///
/// 
struct DisplayCalcSrc<T> {
    id: String,
    label: String,
    exp: Box<dyn CalcEval<T>>,
}
//
//
impl<T> DisplayCalcSrc<T> {
    fn new(
        label: impl Into<String>,
        exp: Box<dyn CalcEval<T>>,
    ) -> Self {
        Self {
            id: format!("DisplayCalcSrc"),
            label: label.into(),
            exp,
        }
    }
    ///
    /// 
    pub fn eval(&mut self, context: Rc<RefCell<CalcContext>>) -> T {
        let context_ref = self.exp.eval(context.clone());
        // let context = context_ref.clone();
        let context = context.borrow_mut();
        println!("\n{}", self.label);
        println!("{}.eval src/field1: {}", self.id, context.src.field1.get());
        println!("{}.eval src/field2: {:#?}", self.id, context.src.field2.get());
        println!("{}.eval src/field3: {}", self.id, context.src.field3.get());
        context_ref
    }
}
//
//
impl<T> CalcEval<T> for DisplayCalcSrc<T> {
    fn eval(&mut self, context: Rc<RefCell<CalcContext>>) -> T {
        DisplayCalcSrc::eval(self, context)
    }
}
///
/// 
struct DisplayCalcResults<T> {
    id: String,
    label: String,
    exp: Box<dyn CalcEval<T>>,
}
//
//
impl<T> DisplayCalcResults<T> {
    fn new(
        label: impl Into<String>,
        exp: Box<dyn CalcEval<T>>,
    ) -> Self {
        Self {
            id: format!("DisplayCalcResults"),
            label: label.into(),
            exp,
        }
    }
    ///
    /// 
    pub fn eval(&mut self, context: Rc<RefCell<CalcContext>>) -> T {
        let context_ref = self.exp.eval(context.clone());
        let context = context.borrow();
        println!("\n{}", self.label);
        println!("{}.eval results/field1: {}", self.id, context.results.calc.field1.get());
        println!("{}.eval results/field2: {:#?}", self.id, context.results.calc.field2.get());
        println!("{}.eval results/field3: {}", self.id, context.results.calc.field3.get());
        context_ref
    }
}
//
//
impl<T> CalcEval<T> for DisplayCalcResults<T> {
    fn eval(&mut self, context: Rc<RefCell<CalcContext>>) -> T {
        DisplayCalcResults::eval(self, context)
    }
}
///
/// Calculations
struct Calc<CalcResults> {
    id: String,
    exp: Box<dyn CalcEval<CalcResults>>,
}
//
//
impl<T> Calc<T> {
    fn new(
        exp: Box<dyn CalcEval<T>>,
    ) -> Self {
        Self {
            id: format!("Calc"),
            exp,
        }
    }
    ///
    /// 
    pub fn eval(&mut self, context: Rc<RefCell<CalcContext>>) -> CalcResults {
        let _ = self.exp.eval(context.clone());
        let context = context.borrow_mut();
        let field1 = context.src.field1.get();
        let field2 = context.src.field2.get().unwrap();
        let field3 = context.src.field3.get();
        CalcResults::new(
            MutValue::new(field1 * 2.0),
            MutValue::new(field2.iter().map(|(_key, value)| value * 2.0).collect()),
            MutValue::new(field3 * 2.0),
        )
    }
}
//
//
impl CalcEval<CalcResults> for Calc<CalcResults> {
    fn eval(&mut self, context: Rc<RefCell<CalcContext>>) -> CalcResults {
        Calc::eval(self, context)
    }
}
///
/// The constructor of the Task1 context
fn prepare_task1_context(parent: &str) -> Rc<RefCell<CalcContext>> {
    let self_id = parent.to_owned() + "prepare_task1_context";
    let address = "127.0.0.1:8080";
    let auth_token = "";
    let database = "nested_value";
    Rc::new(RefCell::new(CalcContext::new(
        CalcSrc::new(
            ConstValue::new(0.64),
            FetchValue::<IndexMap<String, f64>>::new(
                ApiRequest::new(
                    self_id, address, auth_token,
                    ApiQuery::new(ApiQueryKind::Sql(ApiQuerySql::new(
                        database,
                        "select key, value from map_test;"
                    )), false),
                    false, false,
                ),
                Box::new(|reply| {
                    match parse_map(reply) {
                        Ok(reply) => {
                            Ok(reply)
                        },
                        Err(err) => Err(format!("fetch-vec | parse error: {:#?}", err)),
                    }
                }),
            ),
            ConstValue::new(12.32),
        ),
        Results::new(
            CalcResults::new(
                MutValue::new(0.0),
                MutValue::new(vec![]),
                MutValue::new(0.0),
            ),
        ) 
    )))
}
///
/// Extract array from the ApiReply
pub fn parse_map(reply: &[u8]) -> Result<IndexMap::<String, f64>, String> {
    match serde_json::from_slice(reply) {
        Ok(reply) => {
            let reply: ApiReply = reply;
            // println!("reply: {:#?}", reply);
            let mut map = IndexMap::<String, f64>::from([]);
            for row in reply.data {
                let key = row.get("key").unwrap();
                let value = row.get("value").unwrap();
                map.insert(key.as_str().unwrap().to_owned(), value.as_str().unwrap().parse().unwrap());
            }
            Ok(map)
        },
        Err(err) => Err(format!("parse array error: {:?}", err)),
    }
}


///
/// Start calculations
struct Start {}
//
//
impl Start {
    fn new() -> Self {
        Self {}
    }
    ///
    /// 
    pub fn eval(&mut self, context: Rc<RefCell<CalcContext>>) -> Rc<RefCell<CalcContext>> {
        context
    }
}
//
//
impl CalcEval<Rc<RefCell<CalcContext>>> for Start {
    fn eval(&mut self, context: Rc<RefCell<CalcContext>>) -> Rc<RefCell<CalcContext>> {
        Start::eval(self, context)
    }
}