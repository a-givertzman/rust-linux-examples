use api_tools::api::reply::api_reply::ApiReply;
use indexmap::IndexMap;
use crate::value::Value;
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
/// Extract array from the ApiReply
pub fn parse_array(reply: &[u8]) -> Result<Vec<Value>, String> {
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
/// 
pub fn parse_value(reply: &[u8]) -> Result<Value, String> {
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