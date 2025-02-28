use std::{future::Future, pin::Pin};
///
/// Async callback
pub struct Request {
    op: Box<dyn AsyncFn + Send + Sync>,
}
//
//
impl Request {
    pub fn new(op: impl AsyncFn + Send + Sync + 'static) -> Self {
        let request = Self { op: Box::new(op) };
        request
    }
    ///
    /// Performs the request
    pub async fn fetch(&self, val: String) -> String {
        self.op.eval(val).await
    }
}
///
/// 
pub trait AsyncFn {
    fn eval(&self, ctx: String) -> Pin<Box<dyn Future<Output = String> + Send + '_>>;
}
impl<T, F> AsyncFn for T
where
    T: Fn(String) -> F,
    F: std::future::Future<Output = String> + Send + 'static,
{
    fn eval(&self, val: String) -> Pin<Box<dyn Future<Output = String> + Send + '_>> {
        Box::pin(self(val))
    }
}
