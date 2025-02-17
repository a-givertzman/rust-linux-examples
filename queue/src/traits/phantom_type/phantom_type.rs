use std::marker::PhantomData;

#[derive(PartialEq, Debug)]
struct PhuntomTuple<A, B>(A, PhantomData<B>);


#[derive(PartialEq, Debug)]
struct PhantomStruct<A, B> {
    first: A,
    _phantom: PhantomData<B>
}

fn main() {
    println!("PhantomType");
    let tuple_f64_1 = PhuntomTuple("tuple_f64_1", PhantomData::<f64>);
    let tuple_f64_2 = PhuntomTuple("tuple_f64_2", PhantomData::<f64>);
    let tuple_i64 = PhuntomTuple("tuple_i64", PhantomData::<i64>);

    let struct1: PhantomStruct<&str, i64>  = PhantomStruct {
        first: "struct1",
        _phantom: PhantomData
    };
    let struct2: PhantomStruct<&str, f64>  = PhantomStruct {
        first: "struct2",
        _phantom: PhantomData
    };

    println!("tuple_f64_1: {:?}", tuple_f64_1);
    println!("tuple_f64_2: {:?}", tuple_f64_2);
    println!("tuple_i64: {:?}", tuple_i64);
    println!("struct1: {:?}", struct1);
    println!("struct2: {:?}", struct2);

    let comp1 = tuple_f64_1 == tuple_f64_2;
    // Compile-time Error! Type mismatch so these cannot be compared:
    // let comp1 = tuple_f64_1 == tuple_i64;

}