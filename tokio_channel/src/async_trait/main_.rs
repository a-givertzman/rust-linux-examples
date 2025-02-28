mod request;
use std::{future::Future, pin::Pin};
///
/// Async trait defined
pub trait Eval<Out> {
    fn eval<'a>(&'a mut self) -> Pin<Box<dyn Future<Output = Out> + 'a>>;
}
///
/// The Context to be returned from `Eval`
#[derive(Debug, Clone)]
pub struct Context {
    result: String,
}
///
/// Some struct implements async trai `Eval`
pub struct EvaluationStep<'a> {
    ctx: Box<dyn Eval<Context> + Send + 'a>,
}
impl<'a> EvaluationStep<'a> {
    ///
    /// New instance [EvaluationStep]
    /// - `ctx` - [Eval]
    pub fn new(ctx: impl Eval<Context> + Send + 'a) -> Self {
        Self {
            // req,
            ctx: Box::new(ctx)
        }
    }
}
impl Eval<Context> for EvaluationStep<'_> {
    fn eval<'a>(&'a mut self) -> Pin<Box<dyn Future<Output = Context> +'a>> {
        Box::pin(async {
            let reply = String::from("Query");
            let mut ctx = self.ctx.eval().await;
            ctx.result = reply;
            ctx
        })
    }
}
///
/// Fake implements async trai `Eval`
pub struct FakeStep {
    pub ctx: Context,
}
impl Eval<Context> for FakeStep {
    fn eval<'a>(&'a mut self) -> Pin<Box<dyn Future<Output = Context> + 'a>> {
        Box::pin(async {
            self.ctx.clone()
        })
    }
}
//
//
#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    let mut step = EvaluationStep::new(
        FakeStep { ctx: Context { result: "0.0".to_owned() } }
    );
    let result = step.eval().await;
    log::info!("Result: {:?}", result);
}
