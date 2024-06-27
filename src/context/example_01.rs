use std::{cell::RefCell, rc::Rc};
use add_field1::AddField1;
use api_tools::{api::reply::api_reply::ApiReply, client::{api_query::{ApiQuery, ApiQueryKind, ApiQuerySql}, api_request::ApiRequest}};
use calc_eval::CalcEval;
use display_results::DisplayCalcResults;
use display_src::DisplaySrc;
use indexmap::IndexMap;
use mul2::Mul2;
use nested_value::{const_value::ConstValue, fetch_value::FetchValue, mut_value::MutValue};
use calc_context::{AddField1Results, CalcContext, CalcSrc, Mul2Results, Results};
use set_context::SetContext;
mod calc_eval;
mod calc_context;
mod set_context;
mod mul2;
mod add_field1;
mod display_src;
mod display_results;
///
/// 
fn main() {
    let self_id = "main";
    let calc_context = prepare_task1_context(self_id);
    let mut calc = DisplayCalcResults::new(
        "~~~~~~~~~~   Results after AddField1",
        SetContext::new(
            |context, result| {
                context.borrow_mut().results.add_field1 = result;
            },
            AddField1::new(
                DisplayCalcResults::new(
                    "~~~~~~~~~~   Results after Mul2",
                    SetContext::new(
                        |context, result| {
                            let context: Rc<RefCell<CalcContext>> = context;
                            context.borrow_mut().results.mul2 = result;
                        },
                        Mul2::new(
                            DisplayCalcResults::new(
                                "~~~~~~~~~~   Results before calc",
                                DisplaySrc::new(
                                    "~~~~~~~~~~   Src   ~~~~~~~~~~",
                                    Start::new()
                                )
                            ),
                        ),
                    ),
                ),
            )
        ),
    );
    let calc_context = calc.eval(calc_context);
    _ = calc_context;
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
            Mul2Results::new(
                MutValue::new(0.0),
                MutValue::new(vec![]),
                MutValue::new(0.0),
            ),
            AddField1Results::new(
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
/// Start calculations, just returns the context as it
struct Start {}
//
//
impl Start {
    fn new() -> Self {
        Self {}
    }
    ///
    /// Retirns the context as it
    pub fn eval<T>(&mut self, context: T) -> T {
        context
    }
}
//
//
impl CalcEval<Rc<RefCell<CalcContext>>, Rc<RefCell<CalcContext>>> for Start {
    fn eval(&mut self, context: Rc<RefCell<CalcContext>>) -> Rc<RefCell<CalcContext>> {
        Start::eval(self, context)
    }
}