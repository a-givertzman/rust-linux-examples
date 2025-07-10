use std::fmt::Debug;


pub struct TestResult {
    pub name: String,
    pub events: usize,
    pub total_elapsed: std::time::Duration,
}
impl Debug for TestResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = format!(r#"
test: {}
---------------------------
Events: {:?}
---------------------------
Total elapsed: {:?}
            "#, 
            self.name,
            self.events,
            self.total_elapsed
        );
        write!(f, "{result}")
    }
}