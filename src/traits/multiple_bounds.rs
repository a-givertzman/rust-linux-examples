use std::fmt::Debug;

trait Compare<T> where
    T: PartialEq + Debug + PartialOrd {
    fn compare_debug(&self, other: &T);
}


fn compare_debug<T>(target: &T, other: &T) where
    T: PartialEq + Debug + PartialOrd {
    let comp = target > other;
    println!("target: {:?} > other: {:?}    {:?}", target, other, comp);
}


#[derive(Debug)]
struct A<T> where 
    T: PartialEq + Debug + PartialOrd {
    inner: T
}

impl<T> Compare<T> for A<T> where 
    T: PartialEq + Debug + PartialOrd {
    fn compare_debug(&self, other: &T) {
        let inner = &self.inner;
        let comp = inner > other;
        println!("self: {:?} > other: {:?}    {:?}", &self, other, comp);
    }
}

fn main() {
    let string1 = "A string value l";
    let string2 = "A string value";
    let array = [1,2,3];
    let vec = vec![1,2,3];

    compare_debug(&string1, &string2);
    compare_debug(&string2, &string1);
    compare_debug(&array, &[1,2,0]);
    compare_debug(&vec![1,2,0], &vec);

    let a = A{ inner: string1 };
    a.compare_debug(&"A string");
    a.compare_debug(&"A string value long long");
    // compare_debug(&array);
    // compare_debug(&vec);
}