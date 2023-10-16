mod drop_trait;

#[derive(Debug)]
struct Sheep;
#[derive(Debug)]
struct Cow;

trait Animal {
    fn noise(&self) -> &'static str;
}

impl Animal for Sheep {
    fn noise(&self) -> &'static str {
        "Beeeeeeh!"
    }
}

impl Animal for Cow {
    fn noise(&self) -> &'static str {
        "Mooooooh!"
    }
}

fn next(animals: &mut Vec<Box<dyn Animal>>) -> Option<Box<dyn Animal>> {
    animals.pop()
}

fn main() {
    let animals: Vec<Box<dyn Animal>> = vec![
        Box::new(Sheep{}),
        Box::new(Sheep{}),
        Box::new(Cow{}),
        Box::new(Sheep{}),
        Box::new(Cow{}),
        Box::new(Cow{}),
    ];
    for animal in animals {
        println!("animal: {:?}", animal.noise());
    }
}

