use std::collections::HashMap;


#[derive(Clone)]
pub enum Initial {
    Bool(bool),
    Int(i64),
    Float(f64),
    None,
}

pub struct Conf {
    pub id: String,
    pub name: String,
    pub initial: Initial,
    pub nested: HashMap<String, Conf>,
}

impl Conf {
    pub fn name(&self) -> String {
        self.name.clone()
    }
    pub fn get(&self, key: &str) -> String {
        self.name.clone()
    }
    pub fn nested(&mut self, key: &str) -> &mut Conf {
        self.nested.get_mut(key).unwrap()
    }
}

