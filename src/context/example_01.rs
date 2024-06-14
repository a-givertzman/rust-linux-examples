use api_tools::{api::reply::api_reply::ApiReply, client::{api_query::{ApiQuery, ApiQueryKind, ApiQuerySql}, api_request::ApiRequest}};
use indexmap::IndexMap;
use nested_value::{const_value::ConstValue, fetch_value::FetchValue, mut_value::MutValue};
use task1_context::{Task1Context, Task1Results, Task1Src};

mod task1_context;

fn main() {
    let self_id = "main";
    let mut task1_context = prepare_task1_context(self_id);
    println!("task1 field1: {}", task1_context.src.const_f64_field1.get());
    println!("task1 field2: {:#?}", task1_context.src.const_map_field2.get());
    println!("task1 field3: {}", task1_context.src.mut_f64_field3.get());
    task1_context.src.mut_f64_field3.store(&self_id, 24.16).unwrap();
    println!("task1 field3: {}", task1_context.src.mut_f64_field3.get());
}
///
/// The constructor of the Task1 context
fn prepare_task1_context(parent: &str) -> Task1Context {
    let self_id = parent.to_owned() + "prepare_task1_context";
    let address = "127.0.0.1:8080";
    let auth_token = "";
    let database = "nested_value";
    Task1Context::new(
        Task1Src::new(
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
            MutValue::new(12.32),
        ),
        Task1Results::new(
            MutValue::new(0.0),
            MutValue::new(vec![]),
            MutValue::new(0.0),
        ),
    )
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
