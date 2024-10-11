use std::{ops::Add, marker::PhantomData};


#[derive(Debug, Clone, Copy, PartialEq)]
enum Inch {}
#[derive(Debug, Clone, Copy, PartialEq)]
enum Mm {}


#[derive(Debug, Clone, Copy, PartialEq)]
struct Length<Unit>(f64, PhantomData<Unit>);


impl<Unit> Add for Length<Unit> {
    type Output = Length<Unit>;

    fn add(self, rhs: Self) -> Self::Output {
        Length(self.0 + rhs.0, PhantomData)
    }
}


fn main() {
    let one_foot = Length::<Inch>(12.0, PhantomData);
    let one_meter = Length::<Mm>(1000.0, PhantomData);

    // `+` calls the `add()` method we implemented for `Length<Unit>`.
    //
    // Since `Length` implements `Copy`, `add()` does not consume
    // `one_foot` and `one_meter` but copies them into `self` and `rhs`.
    let two_feet = one_foot + one_foot;
    let two_meters = one_meter + one_meter;

    println!("two feet: {:?}", two_feet);
    println!("two meters: {:?}", two_meters);

    // Nonsensical operations fail as they should:
    // Compile-time Error: type mismatch.    
    // let compare = two_feet == two_meters;
}