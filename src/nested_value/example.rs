//!
//! #### Demo version of the nested value structure
//! 
//! - ConstValue - contains simple constant value (bool / i64 / u64 / f64, String, Vec<Value>, Map<Value, Value>)
//! - MutValue - contains simple mutable value (bool / i64 / u64 / f64, String, Vec<Value>, Map<Value, Value>)
//! - MultiValue - contains map of nested values
//! - FetchValue - contains cached data fetched using ApiRequest
//! 
//! **For example constants:**
//! 
//! ```
//! let value = MultiValue::new([
//!     ("bool", Box::new(ConstValue::new(Value::Bool(true)))),
//!     ("u64", Box::new(ConstValue::new(Value::U64(1234567890)))),
//!     ("f64", Box::new(ConstValue::new(Value::F64(12345.6789012345)))),
//! ]);
//! ```
//! 
//! **Example with sollections:**
//! 
//! ```
//! let value = MultiValue::new([
//!     ("collections", Box::new(MultiValue::new([
//!         ("vec", Box::new(ConstValue::new(Value::Vec(vec![
//!             Value::U64(222),
//!             Value::I64(-222),
//!             Value::F64(222.222222),
//!         ])))),
//!         ("map", Box::new(ConstValue::new(Value::Map(IndexMap::from([
//!             (Value::from("222"), Value::U64(222)),
//!             (Value::from("-222"), Value::I64(-222)),
//!             (Value::from("222.222"), Value::F64(222.222222)),
//!         ]))))),
//!         ("fetch-222", Box::new(FetchValue::new(
//!             ApiRequest::new(&u32::to_be_bytes(222)),
//!             Box::new(|input| {
//!                 match input.try_into() {
//!                     Ok(bytes) => Ok(Value::U64(u32::from_be_bytes(bytes) as u64)),
//!                     Err(_) => Err(format!("fetch-222 | invalid input: {:?}", input)),
//!                 }
//!             }),
//!         ))),
//!         ("fetch-222.222", Box::new(FetchValue::new(
//!             ApiRequest::new(&f64::to_be_bytes(222.222)),
//!             Box::new(|input| {
//!                 match input.try_into() {
//!                     Ok(bytes) => Ok(Value::F64(f64::from_be_bytes(bytes))),
//!                     Err(_) => Err(format!("fetch-222.222 | invalid input: {:?}", input)),
//!                 }
//!             }),
//!         ))),
//!     ]))),
//! ]);
//! ```
//! 
//! **Example with fetched values:**
//! 
//! ```
//! let value = MultiValue::new([
//!     ("fetched", Box::new(MultiValue::new([
//!         ("fetch-vec", Box::new(FetchValue::new(
//!             ApiRequest::new(
//!                 serde_json::to_string(&vec![123, 456, 789])
//!                     .unwrap_or_else(|err| panic!("fetch-vec | to json error: {:#?}", err)).as_bytes(),
//!             ),
//!             Box::new(|reply| {
//!                 match serde_json::from_slice(reply) {
//!                     Ok(reply) => {
//!                         let reply: Vec<u64> = reply;
//!                         // Ok(reply)
//!                         Ok(Value::Vec(reply.into_iter().map(|v| Value::U64(v)).collect()))
//!                     }
//!                     Err(err) => Err(format!("fetch-vec | from json error: {:#?}", err)),
//!                 }
//!             }),
//!         ))),
//!         ("fetch-map", Box::new(FetchValue::new(
//!             ApiRequest::new(
//!                 r#"{
//!                     "key1": 111.111,
//!                     "key2": 222.222,
//!                     "key3": 333.333
//!                 }"#.as_bytes(),
//!             ),
//!             Box::new(|reply| {
//!                 match serde_json::from_slice(reply) {
//!                     Ok(reply) => {
//!                         let reply: IndexMap<&str, f64> = reply;
//!                         // Ok(reply)
//!                         Ok(Value::Map(reply.into_iter().map(|(key, value)| (Value::from(key), Value::from(value))).collect()))
//!                     }
//!                     Err(err) => Err(format!("fetch-vec | from json error: {:#?}", err)),
//!                 }
//!             }),
//!         ))),
//!     ]))),
//! ]);
//! ```
//! 
//! **Try example using command:**
//! 
//! ```
//!     cargo run --bin nested_value --release -- --nocapture
//! ```
#[path = "../debug_session/mod.rs"]
mod debug_session;
mod const_value;
mod nested_value;
mod multi_value;
mod fetch_value;
mod mut_value;
mod value;
use std::time::Instant;

use api_tools::{api::reply::api_reply::ApiReply, client::{api_query::{ApiQuery, ApiQueryKind, ApiQuerySql}, api_request::ApiRequest}};
use debug_session::debug_session::{DebugSession, LogLevel};
use fetch_value::FetchValue;
use indexmap::IndexMap;
use log::error;
use multi_value::MultiValue;
use mut_value::MutValue;
use value::Value;
use crate::{
    const_value::ConstValue,
    nested_value::NestedValue,
};
//
//
fn main() {
    DebugSession::init(LogLevel::Debug);
    let self_id = "main";
    let value = ConstValue::new(Value::Null);
    println!("const value: {:#?}", value);
    println!("const value: {:?}", value.get(""));
    println!();
    let value = ConstValue::new(12345.6789012345);
    println!("const value: {:#?}", value);
    println!("const value: {:?}", value.get(""));
    println!();
    let value = ConstValue::new(12345);
    println!("const value: {:#?}", value);
    println!("const value: {:?}", value.get(""));
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
///
///
/// 
/// 
/// 
/// 
/// 
/// Test simple fetched values
fn _main() {
    let time = Instant::now();
    for _ in 0..1 {
        request_value();
        request_array();
        request_map();
    }
    println!("Total elapse: {:#?}", time.elapsed());

}
///
/// 
fn parse_value(reply: &[u8]) -> Result<Value, String> {
    match serde_json::from_slice(reply) {
        Ok(reply) => {
            let reply: ApiReply = reply;
            match reply.data.first() {
                Some(row) => {
                    match row.values().next() {
                        Some(value) => {
                            match value {
                                serde_json::Value::Null => Ok(Value::Null),
                                serde_json::Value::Bool(v) => Ok(Value::Bool(*v)),
                                serde_json::Value::Number(v) => {
                                    if v.is_f64() {
                                        Ok(Value::F64(v.as_f64().unwrap()))
                                    } else if v.is_i64() {
                                        Ok(Value::I64(v.as_i64().unwrap()))
                                    } else if v.is_u64() {
                                        Ok(Value::U64(v.as_u64().unwrap()))
                                    } else {
                                        Err(format!("request_value | Unknown numeric type: {:?}", v))
                                    }
                                }
                                serde_json::Value::String(v) => Ok(Value::String(v.to_owned())),
                                serde_json::Value::Array(_) => Err(format!("request_value | Simple types only supported, but found Array")),
                                serde_json::Value::Object(_) => Err(format!("request_value | Simple types only supported, but found Mapping")),
                            }
                        },
                        None => Err(format!("request_value | value not present in the reply: {:?}", reply)),
                    }
                }
                None => Err(format!("request_value | value not present in the reply: {:?}", reply)),
            }
        },
        Err(err) => Err(format!("parse array error: {:?}", err)),
    }
}
///
/// Request single value
fn request_value() {
    let self_id = "main";
    let address = "0.0.0.0:8080";
    let auth_token = "";
    let database = "nested_value";
    let debug = true;
    let value = FetchValue::new(
        ApiRequest::new(
            self_id, address, auth_token,
            ApiQuery::new(ApiQueryKind::Sql(ApiQuerySql::new(database, "select 222;")), false),
            false, debug,
        ),
        Box::new(|reply| {
            parse_value(reply)
        }),
    );
    let time = Instant::now();
    match value.get("") {
        Ok(value) => {
            println!("reply: {:#?}", value);
        },
        Err(err) => {
            println!("get value error: : {:?}", err);
        },
    }
    println!("Elapse: {:#?}", time.elapsed());
}
///
/// Extract array from the ApiReply
fn parse_array(reply: &[u8]) -> Result<Vec<Value>, String> {
    match serde_json::from_slice(reply) {
        Ok(reply) => {
            let reply: ApiReply = reply;
            let mut array = vec![];
            for row in reply.data {
                let value = row.values().next().unwrap().clone();
                array.push(Value::I64(value.as_i64().unwrap()))
            }
            Ok(array)
        },
        Err(err) => Err(format!("parse array error: {:?}", err)),
    }
}
///
/// Request array
fn request_array() {
    let self_id = "main";
    let address = "0.0.0.0:8080";
    let auth_token = "";
    let database = "nested_value";
    let debug = true;
    let value = FetchValue::new(
        ApiRequest::new(
            self_id, address, auth_token,
            ApiQuery::new(ApiQueryKind::Sql(ApiQuerySql::new(
                database,
                "select value from array_test;"
            )), false),
            false, debug,
        ),
        Box::new(|reply| {
            parse_array(reply)
        }),
    );
    let time = Instant::now();
    match value.get("") {
        Ok(value) => {
            println!("reply: {:#?}", value);
        },
        Err(err) => {
            println!("get value error: : {:?}", err);
        },
    }
    println!("Elapse: {:#?}", time.elapsed());
}
///
/// Extract array from the ApiReply
fn parse_map(reply: &[u8]) -> Result<IndexMap::<String, f64>, String> {
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
/// Request map
fn request_map() {
    let self_id = "main";
    let address = "0.0.0.0:8080";
    let auth_token = "";
    let database = "nested_value";
    let debug = true;
    let value = FetchValue::new(
        ApiRequest::new(
            self_id, address, auth_token,
            ApiQuery::new(ApiQueryKind::Sql(ApiQuerySql::new(
                database,
                "select key, value from map_test;"
            )), false),
            false, debug,
        ),
        Box::new(|reply| {
            parse_map(reply)
        }),
    );
    let time = Instant::now();
    match value.get("") {
        Ok(value) => {
            println!("reply: {:#?}", value);
        },
        Err(err) => {
            println!("get value error: : {:?}", err);
        },
    }
    println!("Elapse: {:#?}", time.elapsed());
}
