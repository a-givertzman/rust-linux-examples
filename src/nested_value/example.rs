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
use debug_session::debug_session::{DebugSession, LogLevel};
use fetch_value::{ApiRequest, FetchValue};
use indexmap::IndexMap;
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
    let key = "bool-flags/flag-false";
    flags.store(key, true).unwrap();
    println!("multi value {}: {:?}", key, flags.get(key));
    let key = "bool-flags/flag-true";
    let result = flags.store(key, true);
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
    flags.store(key, 888).unwrap();
    println!("multi value '{}': {:?}", key, flags.get(key));
    println!();

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
                    ApiRequest::new(&u32::to_be_bytes(222)),
                    Box::new(|input| {
                        match input.try_into() {
                            Ok(bytes) => Ok(Value::U64(u32::from_be_bytes(bytes) as u64)),
                            Err(_) => Err(format!("fetch-222 | invalid input: {:?}", input)),
                        }
                    }),
                ))),
                ("fetch-222.222", Box::new(FetchValue::new(
                    ApiRequest::new(&f64::to_be_bytes(222.222)),
                    Box::new(|input| {
                        match input.try_into() {
                            Ok(bytes) => Ok(Value::F64(f64::from_be_bytes(bytes))),
                            Err(_) => Err(format!("fetch-222.222 | invalid input: {:?}", input)),
                        }
                    }),
                ))),

            ]))),
            ("fetch-vec", Box::new(FetchValue::new(
                ApiRequest::new(
                    serde_json::to_string(&vec![123, 456, 789])
                        .unwrap_or_else(|err| panic!("fetch-vec | to json error: {:#?}", err)).as_bytes(),
                ),
                Box::new(|reply| {
                    match serde_json::from_slice(reply) {
                        Ok(reply) => {
                            let reply: Vec<u64> = reply;
                            // Ok(reply)
                            Ok(Value::Vec(reply.into_iter().map(|v| Value::U64(v)).collect()))
                        }
                        Err(err) => Err(format!("fetch-vec | from json error: {:#?}", err)),
                    }
                }),
            ))),
            ("fetch-map", Box::new(FetchValue::new(
                ApiRequest::new(
                    r#"{
                        "key1": 111.111,
                        "key2": 222.222,
                        "key3": 333.333
                    }"#.as_bytes(),
                ),
                Box::new(|reply| {
                    match serde_json::from_slice(reply) {
                        Ok(reply) => {
                            let reply: IndexMap<&str, f64> = reply;
                            // Ok(reply)
                            Ok(Value::Map(reply.into_iter().map(|(key, value)| (Value::from(key), Value::from(value))).collect()))
                        }
                        Err(err) => Err(format!("fetch-vec | from json error: {:#?}", err)),
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
    for key in keys {
        println!("multi value '{}': {:#?}", key, value.get(key));
        println!();
    }
    let key = "v1/v2/f64";
    println!("multi value '{}': {:#?}", key, value.get(key));
    value.store(key, Value::F64(222.222222 + 222.222)).unwrap();
    println!("multi value '{}': {:#?}", key, value.get(key));
}
