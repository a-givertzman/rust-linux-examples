#![allow(non_snake_case)]

use log::{
    // info,
    // trace,
    debug,
    // warn,
};
use serde::{
    Serialize,
    Deserialize,
};
use std::time::SystemTime;
use chrono::{
    DateTime,
    Utc,
    SecondsFormat,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct DsPoint<T> {
    pub class: String,
    #[serde(rename(deserialize = "type", serialize = "type"))]
    pub datatype: String,
    pub name: String,
    pub value: T,
    pub status: i64,
    pub timestamp: String,
}
impl<'a, T> DsPoint<T> 
where
    for<'de> T: Deserialize<'de> + 'a,
    T: Serialize + 'a,
    T: Clone + From<u32> + From<f64>,

{
    pub fn fromBytes(bytes: &[u8]) -> Self {
        let string = String::from_utf8_lossy(&bytes).into_owned();
        debug!("[DsPoint] string: {:#?}", string);
        // let eof = String::from_utf8_lossy(&[4]).into_owned();
        // println!("[DsPoint] eof: {:#?}", eof);
        // let parts: Vec<&str> = string.split(&eof).collect();
        // debug!("[DsPoint] parts: {:#?}", parts);
        // let pointJson = parts[0];
        let point: DsPoint<T> = match serde_json::from_str(&string) {
            Ok(value) => {value},
            Err(_) => {
                DsPoint {
                    class: String::from("commonCmd"),
                    datatype: String::from("real"),
                    name: String::new(),
                    value: T::from(0),
                    status: 10,
                    timestamp: DateTime::<Utc>::from(SystemTime::now()).to_rfc3339_opts(SecondsFormat::Micros, true),
                }
            },
        };
        // debug!("[DsPoint] point: {:#?}", point);
        point
    }
    pub fn toJson(&self) -> Result<String, serde_json::error::Error>{
        let result = serde_json::to_string(&self);
        // debug!("[DsPoint] point: {:#?}", result);
        result
    }
}
