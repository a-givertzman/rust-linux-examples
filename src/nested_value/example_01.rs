//!
//! #### Demo version of the nested value structure
//! 
//! - ConstValue - contains simple constant value (bool / i64 / u64 / f64, String, Vec<Value>, Map<Value, Value>)
//! - MutValue - contains simple mutable value (bool / i64 / u64 / f64, String, Vec<Value>, Map<Value, Value>)
//! - MultiValue - contains map of nested values
//! - FetchValue - contains cached data fetched using ApiRequest
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
mod parse_example;

use std::time::Instant;
use api_tools::client::{api_query::{ApiQuery, ApiQueryKind, ApiQuerySql}, api_request::ApiRequest};
use debug_session::debug_session::{DebugSession, LogLevel};
use fetch_value::FetchValue;
use parse_example::{parse_array, parse_map, parse_value};
use crate::nested_value::NestedValue;
/// 
/// Test simple fetched values
fn main() {
    DebugSession::init(LogLevel::Debug);
    let time = Instant::now();
    for _ in 0..1 {
        request_value();
        request_array();
        request_map();
    }
    println!("Total elapse: {:#?}", time.elapsed());

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
