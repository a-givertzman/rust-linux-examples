use std::fmt::Debug;


pub struct TestResult {
    pub name: String,
    pub events: usize,
    pub producers: usize,
    pub total_produced: usize,
    pub receivers: usize,
    pub total_received: usize,
    pub loads: usize,
    pub total_elapsed: std::time::Duration,
}
impl Debug for TestResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = format!(r#"
test: {}
---------------------------
Events: {:?}
---------------------------
Producers: {:?}
Total produced: {:?}
---------------------------
Receivers: {:?}
Total received: {:?}
---------------------------
Loads: {:?}
---------------------------
Total elapsed: {:?}
            "#, 
            self.name,
            self.events,
            self.producers,
            self.total_produced,
            self.receivers,
            self.total_received,
            self.loads,
            self.total_elapsed
        );
        write!(f, "{result}")
    }
}