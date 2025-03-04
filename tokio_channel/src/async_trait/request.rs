use std::{future::Future, pin::Pin};
use coco::Stack;
///
/// 
#[derive(Debug)]
pub struct Link {}
///
/// Async callback
pub struct Request {
    link: Stack<Link>,
    op: Box<dyn AsyncFn + Send + Sync>,
}
//
//
impl Request {
    pub fn new(link: Link, op: impl AsyncFn + Send + Sync + 'static) -> Self {
        let stack = Stack::new();
        stack.push(link);
        Self {
            link: stack,
            op: Box::new(op),
        }
    }
    ///
    /// Performs the request
    pub async fn fetch(&self, val: String) -> String {
        let link = self.link.pop().unwrap();
        let (result, link) = self.op.eval(val, link).await;
        self.link.push(link);
        result
    }
}
///
/// 
pub trait AsyncFn {
    fn eval(&self, ctx: String, link: Link) -> Pin<Box<dyn Future<Output = (String, Link)> + Send + '_>>;
}
impl<T, F> AsyncFn for T
where
    T: Fn(String, Link) -> F,
    F: std::future::Future<Output = (String, Link)> + Send + 'static,
{
    fn eval(&self, val: String, link: Link) -> Pin<Box<dyn Future<Output = (String, Link)> + Send + '_>> {
        Box::pin(self(val, link))
    }
}
