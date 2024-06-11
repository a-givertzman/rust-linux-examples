use std::{cell::RefCell, fmt::Debug};
use api_tools::client::api_request::ApiRequest;
use crate::nested_value::NestedValue;
///
/// Returns the data fetched from the ApiRequest on the first call get() method.
/// Next time returns cached value.
pub struct FetchValue<T> {
    id: String,
    inited: bool,
    value: RefCell<Option<T>>,
    request: RefCell<ApiRequest>,
    parser: Box<dyn Fn(&[u8]) -> Result<T, String>>,
}
//
//
impl<T> FetchValue<T> {
    ///
    /// Returns new instance of the [FetchedValue]
    /// - request: ApiRequest - fetches data from the API Server
    /// - parser: closure receives raw API result, returns parsed data
    pub fn new(request: ApiRequest, parser: Box<dyn Fn(&[u8]) -> Result<T, String>>) -> Self {
        Self {
            id: "FetchValue".to_owned(),
            inited: false,
            value: RefCell::new(None),
            request: RefCell::new(request),
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
    fn init(&mut self, key: &str) {
        self.id = key.to_owned();
        self.inited = true;
    }
    //
    //
    fn get(&self, _: &str) -> Result<T, String> {
        if self.value.borrow().is_none() {
            match self.request.borrow_mut().fetch(false) {
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
impl<T: Debug> std::fmt::Debug for FetchValue<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FetchValue").field("id", &self.id).field("value", &self.value).finish()
    }
}
