///
/// Contains future callback
pub struct Future<T> {
    exec: Box<dyn Fn() -> T>,
    // then: Box<dyn Fn(T) -> T>,
}
//
//
impl<T> Future<T> {
    ///
    /// 
    pub fn new(exec: Box<dyn Fn() -> T>) -> Self {
        Self {
            exec,
        }
    }
    /// 
    /// 
    pub fn then(&self, on_done: Box<dyn Fn(T) -> T>) -> T {
        let event = (self.exec)();
        (on_done)(event)
    }
}