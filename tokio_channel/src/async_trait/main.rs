mod request;
use std::{future::Future, pin::Pin};
use request::{Link, Request};
// use request::AsyncFn;
///
/// Async trait defined
pub trait Eval<Out> {
    fn eval(&mut self) -> Pin<Box<dyn Future<Output = Out> + '_>>;
}
///
/// The Context to be returned from `Eval`
#[derive(Debug, Clone)]
pub struct Context {
    result: String,
}
///
/// Some struct implements async trai `Eval`
pub struct EvaluationStep {
    req: Request,
    ctx: Box<dyn Eval<Context> + Send>,
}
impl EvaluationStep {
    ///
    /// New instance [EvaluationStep]
    /// - `ctx` - [Eval]
    pub fn new(
        req: Request,
    // req: impl AsyncFn + Send + Sync + 'static,
        ctx: impl Eval<Context> + Send + 'static
    ) -> Self {
        Self {
            req,
            // req: Box::new(req),
            ctx: Box::new(ctx)
        }
    }
}
impl Eval<Context> for EvaluationStep {
    fn eval(&mut self) -> Pin<Box<dyn Future<Output = Context> + '_>> {
        Box::pin(async move {
            // let reply = self.req.eval("Query".to_owned()).await;
            let reply = self.req.fetch("Query".to_owned()).await;
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
    fn eval(&mut self) -> Pin<Box<dyn Future<Output = Context> + '_>> {
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
        Request::new(
            Link {},
            async |val, link| {
                (format!("{:?}/{}/Reply", link, val), link)
            },
        ),
        FakeStep { ctx: Context { result: "0.0".to_owned() } }
    );
    let result = step.eval().await;
    log::info!("Result: {:?}", result);
    // tokio::task::spawn(future)
}
