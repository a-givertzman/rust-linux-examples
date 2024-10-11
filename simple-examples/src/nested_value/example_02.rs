//!
//! #### Demo version of the nested value structure
//! 
//! - ConstValue - contains simple constant value (bool / i64 / u64 / f64, String, Vec<Value>, Map<Value, Value>)
//! - MutValue - contains simple mutable value (bool / i64 / u64 / f64, String, Vec<Value>, Map<Value, Value>)
//! - MultiValue - contains map of nested values
//! - FetchValue - contains cached data fetched using ApiRequest
//! 
//! **For example constants & mutable value:**
//! 
//! ```rust
//! let mut flags = MultiValue::new([
//!     ("bool-flags", Box::new(MultiValue::new([
//!         ("flag-true", Box::new(ConstValue::new(true))),
//!         ("flag-false", Box::new(MutValue::new(false))),
//!     ]))),
//! ]);
//! let key = "bool-flags/flag-true";
//! println!("multi value {}: {:?}", key, flags.get(key));
//! ```
//! 
//! **Example with fetched values:**
//! 
//! ```rust
//! pub fn parse_value(reply: &[u8]) -> Result<serde_json::Value, String> {
//!     match serde_json::from_slice(reply) {
//!         Ok(reply) => {
//!             let reply: ApiReply = reply;
//!             match reply.data.first() {
//!                 Some(row) => {
//!                     match row.values().next() {
//!                         Some(value) => Ok(value.to_owned()),
//!                         None => Err(format!("request_value | value not present in the reply: {:?}", reply)),
//!                     }
//!                 }
//!                 None => Err(format!("request_value | value not present in the reply: {:?}", reply)),
//!             }
//!         },
//!         Err(err) => Err(format!("parse array error: {:?}", err)),
//!     }
//! }
//! let mut value = FetchValue::new(
//!     ApiRequest::new(
//!         self_id, address, auth_token,
//!         ApiQuery::new(ApiQueryKind::Sql(ApiQuerySql::new(database, "select 222;")), false),
//!         false, false,
//!     ),
//!     Box::new(|reply| {
//!         parse_value(reply)
//!     }),
//! ),
//! println!("multi value {}: {:?}", key, value.get(""));
//! ```
//! 
//! **Try example using command:**
//! 
//! ```
//!     cargo run --bin nested_value --release -- --nocapture
//! ```
mod value;
mod parse_example;

use std::time::Instant;
use api_tools::client::{api_query::{ApiQuery, ApiQueryKind, ApiQuerySql}, api_request::ApiRequest};
use debugging::session::debug_session::{Backtrace, DebugSession, LogLevel};
use indexmap::IndexMap;
use log::error;
use nested_value::{const_value::ConstValue, fetch_value::FetchValue, multi_value::MultiValue, mut_value::MutValue, nested_value::NestedValue};
use parse_example::{parse_array, parse_map, parse_value};
use value::Value;
//
//
fn main() {
    DebugSession::init(LogLevel::Debug, Backtrace::Short);
    let self_id = "main";
    let value = ConstValue::new(Value::Null);
    println!("const value: {:#?}", value);
    println!("const value: {:?}", value.get());
    println!();
    let value = ConstValue::new(12345.6789012345);
    println!("const value: {:#?}", value);
    println!("const value: {:?}", value.get());
    println!();
    let value = ConstValue::new(12345);
    println!("const value: {:#?}", value);
    println!("const value: {:?}", value.get());
    println!();

    let mut flags = MultiValue::new([
        ("bool-flags", Box::new(MultiValue::new([
            ("flag-true", Box::new(ConstValue::new(true))),
            ("flag-false", Box::new(MutValue::new(false))),
        ]))),
    ]);
    let key = "bool-flags/flag-true";
    println!("multi value {}: {:?}", key, flags.get(key));
    let key = "bool-flags/flag-false";
    println!("multi value {}: {:?}", key, flags.get(key));
    println!("multi value {}: {:?}", key, flags.get(key));
    let key = "bool-flags/flag-falsed";
    flags.store(self_id, key, true).unwrap_or_else(|err| error!("main | Store error: {}", err));
    println!("multi value {}: {:?}", key, flags.get(key));
    let key = "bool-flags/flag-true";
    let result = flags.store(self_id, key, true);
    println!("multi result {}: {:?}", key, result);
    println!("multi value {}: {:?}", key, flags.get(key));
    println!();

    let mut flags = MultiValue::new([
        ("int-flags", Box::new(MultiValue::new([
            ("flag-1", Box::new(ConstValue::new(1))),
            ("flag-5", Box::new(ConstValue::new(5))),
            ("flag-876", Box::new(MutValue::new(876))),
        ]))),
    ]);
    let key = "int-flags/flag-1";
    println!("multi value '{}': {:?}", key, flags.get(key));
    let key = "int-flags/flag-5";
    println!("multi value '{}': {:?}", key, flags.get(key));
    let key = "int-flags/flag-876";
    println!("multi value '{}': {:?}", key, flags.get(key));
    flags.store(self_id, key, 888).unwrap();
    println!("multi value '{}': {:?}", key, flags.get(key));
    println!();
    let address = "127.0.0.1:8080";
    let auth_token = "";
    let database = "nested_value";
    let mut value = MultiValue::new([
        ("u64", Box::new(ConstValue::new(Value::U64(1234567890)))),
        ("i64", Box::new(ConstValue::new(Value::I64(-1234567890)))),
        ("f64", Box::new(ConstValue::new(Value::F64(12345.6789012345)))),
        ("v1", Box::new(MultiValue::new([
            ("u64", Box::new(ConstValue::new(Value::U64(111)))),
            ("i64", Box::new(ConstValue::new(Value::I64(-111)))),
            ("f64", Box::new(ConstValue::new(Value::F64(111.111111)))),
            ("v2", Box::new(MultiValue::new([
                ("u64", Box::new(ConstValue::new(Value::U64(222)))),
                ("i64", Box::new(ConstValue::new(Value::I64(-222)))),
                ("f64", Box::new(MutValue::new(Value::F64(222.222222)))),

                ("vec", Box::new(ConstValue::new(Value::Vec(vec![Value::U64(222), Value::I64(-222), Value::F64(222.222222)])))),

                ("map", Box::new(ConstValue::new(Value::Map(IndexMap::from([
                    (Value::from("222"), Value::U64(222)),
                    (Value::from("-222"), Value::I64(-222)),
                    (Value::from("222.222"), Value::F64(222.222222)),
                ]))))),

                ("fetch-222", Box::new(FetchValue::new(
                    ApiRequest::new(
                        self_id, address, auth_token,
                        ApiQuery::new(ApiQueryKind::Sql(ApiQuerySql::new(database, "select 222;")), false),
                        false, false,
                    ),
                    Box::new(|reply| {
                        parse_value(reply)
                    }),
                ))),
                ("fetch-222.222", Box::new(FetchValue::new(
                    ApiRequest::new(
                        self_id, address, auth_token,
                        ApiQuery::new(ApiQueryKind::Sql(ApiQuerySql::new(database, "select 222.222;")), false),
                        false, false,
                    ),
                    Box::new(|reply| {
                        parse_value(reply)
                    }),
                ))),

            ]))),
            ("fetch-vec", Box::new(FetchValue::new(
                ApiRequest::new(
                    self_id, address, auth_token,
                    ApiQuery::new(ApiQueryKind::Sql(ApiQuerySql::new(
                        database,
                        "select value from array_test;"
                    )), false),
                    false, false,
                ),
                Box::new(|reply| {
                    match parse_array(reply) {
                        Ok(v) => Ok(Value::Vec(v)),
                        Err(err) => Err(format!("fetch-vec | error: {:#?}", err)),
                    }
                }),
            ))),
            ("fetch-map", Box::new(FetchValue::new(
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
                            let reply = reply
                                .into_iter().map(|(key, value)| (Value::String(key), Value::F64(value)))
                                .collect();
                            Ok(Value::Map(reply))
                        },
                        Err(err) => Err(format!("fetch-vec | parse error: {:#?}", err)),
                    }
                }),
            ))),

        ]))),
    ]);
    let keys = [
        "u64",
        "i64",
        "f64",
        "v1/f64",
        "v1/v2/f64",
        "v1/v2/vec",
        "v1/v2/map",
        "v1/v2/fetch-222",
        "v1/v2/fetch-222.222",
        "v1/fetch-vec",
        "v1/fetch-map",
        "v1/fetch-map",
    ];
    let time = Instant::now();
    for key in keys {
        println!("multi value '{}': {:#?}", key, value.get(key));
        println!();
    }
    let key = "v1/v2/f64";
    println!("multi value '{}': {:#?}", key, value.get(key));
    value.store(self_id, key, Value::F64(222.222222 + 222.222)).unwrap();
    value.store(self_id, key, Value::F64(222.222222 + 333.333)).unwrap();
    println!("multi value edited '{}': {:#?}", key, value.edited(key));
    println!("multi value '{}': {:#?}", key, value.get(key));
    println!("Total elapse: {:#?}", time.elapsed());
}
