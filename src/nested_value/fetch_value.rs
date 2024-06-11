use std::{cell::RefCell, fmt::Debug};
use log::debug;
use crate::nested_value::NestedValue;
///
/// Returns the data fetched from the ApiRequest on the first call get() method.
/// Next time returns cached value.
pub struct FetchValue<R> {
    id: String,
    inited: bool,
    value: RefCell<Option<R>>,
    request: ApiRequest,
    parser: Box<dyn Fn(&[u8]) -> Result<R, String>>,
}
//
//
impl<R> FetchValue<R> {
    ///
    /// Returns new instance of the [FetchedValue]
    /// - request: ApiRequest - fetches data from the API Server
    /// - parser: closure receives raw API result, returns parsed data
    pub fn new(request: ApiRequest, parser: Box<dyn Fn(&[u8]) -> Result<R, String>>) -> Self {
        Self {
            id: std::any::type_name::<Self>().to_owned(),
            inited: false,
            value: RefCell::new(None),
            request,
            parser,
        }
    }
}
//
//
impl<T: Clone> NestedValue<T> for FetchValue<T> {
    //
    //
    fn id(&self) -> String {
        self.id.clone()
    }
    //
    //
    fn init(&mut self, parent: &str) {
        self.id = format!("{}/{}", parent, self.id);
        self.inited = true;
    }
    //
    //
    fn get(&self, _: &str) -> Result<T, String> {
        if self.value.borrow().is_none() {
            match self.request.fetch() {
                Ok(reply) => {
                    match (self.parser)(&reply) {
                        Ok(reply) => {
                            self.value
                                .borrow_mut()
                                .replace(reply.clone());
                        }
                        Err(err) => return Err(format!("{}.get | Parser returns error: {:#?}", self.id, err)),
                    }
                }
                Err(err) => return Err(format!("{}.get | Request returns error: {:#?}", self.id, err)),
            }
        }
        Ok(self.value
            .borrow()
            .clone()
            .unwrap_or_else(|| panic!("{}.get | Internal error - cache not initialised", self.id))
        )
    }
}
//
//
impl<R: Debug> std::fmt::Debug for FetchValue<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FetchValue").field("id", &self.id).field("value", &self.value).finish()
    }
}

///
/// Demo Mok instead real ApiRequest
pub struct ApiRequest {
    value: Vec<u8>,
}
impl ApiRequest {
    pub fn new(value: &[u8]) -> Self {
        Self {
            value: value.to_owned(),
        }
    }
    pub fn fetch(&self) -> Result<Vec<u8>, String> {
        debug!("ApiRequest.fetch | fetched value: {:?}", self.value);
        Ok(self.value.clone())
    }
}