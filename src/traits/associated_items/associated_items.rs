#[derive(Debug)]
struct Container(i32, i32);

trait Contains<A, B> {
    fn contains(&self, first: &A, last: &B) -> bool;
    fn first(&self) -> i32;
    fn last(&self) -> i32;
}


impl Contains<i32, i32> for Container {
    fn contains(&self, first: &i32, last: &i32) -> bool {
        (&self.0 == first) && (&self.1 == last)
    }
    fn first(&self) -> i32 {
        self.0
    }
    fn last(&self) -> i32 {
        self.1
    }
}


fn diff<A, B, C>(container: &C) -> i32 where
    C: Contains<A, B> {
    container.last() - container.first()
}

trait Diff {
    fn diff(&self) -> i32;
}

impl Diff for Container {
    fn diff(&self) -> i32 {
        self.1 - self.0
    }
}

fn main() {
    let first = 12;
    let last = 24;
    let container = Container(first, last);
    println!("container {:?}", container);
    println!("container.first {} ", container.first());
    println!("container.last {} ", container.last());
    println!("contains {} and {} - {}", first, last, container.contains(&first, &last));
    println!("contains {} and {} - {}", 11, last, container.contains(&11, &last));
    println!("contains {} and {} - {}", first, 22, container.contains(&first, &22));
    let diff = diff(&container);
    println!("difference( container ) {} ", diff);
    let diff = container.diff();
    println!("container.difference() {} ", diff);
}