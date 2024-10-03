use std::env;
use nestify::nest;
use struct_iterable::Iterable;


macro_rules! nested {
    //
    // input is empty: time to output
    (@munch () -> {$(#[$attr:meta])* $name:ident $(($ty:ty))*}) => {
        $(#[$attr])*
        struct $name {
            path: String,
            $(child: $ty,),*
            // $($id: $ty,),*
        }
    };
    //    
    // branch off to generate an inner struct
    (@munch ($name:ident {$($inner:tt)*} $($next:tt)*) -> {$(#[$attr:meta])* struct $($output:tt)*}) => {
        nested!(@munch ($($inner)*) -> {$(#[$attr])* $name});
        nested!(@munch ($($next)*) -> {$(#[$attr])* $($output)* ($name)});
    };
    //
    // throw on the last field
    (@munch ($ty:ty) -> {$($output:tt)*}) => {
        nested!(@munch () -> {$($output)* ($ty)});
    };
    //
    // throw on another field (not the last one)
    (@munch ($ty:ty, $($next:tt)*) -> {$($output:tt)*}) => {
        nested!(@munch ($($next)*) -> {$($output)* ($id: $ty)});
    };
    //
    // entry point (this is where a macro call starts)
    ($(#[$attr:meta])* $name:ident { $($input:tt)*} ) => {
        nested!(@munch ($($input)*) -> {$(#[$attr])* struct $name});
        //                 ^^^^^^^^^^^^    ^^^^^^^^^^^^^^^^^^^^^^^^^^^
        //                     input       output
    }
}


///
/// Build tree
nested!{
    #[derive(Debug)]
    Parent {
        Child {
        }
    }
}





fn main() {
    env::set_var("RUST_LOG", "debug");
    env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    // let thing = Parent { child: Child {} };
    // let name = name_struct!(thing.child);
    // println!("{name}"); // on playground prints: "playground::Parent"

    // let tree = Parent::from(Child {});
    // Child, };
    let tree = Parent {
        path: "/parent".to_owned(),
        child: Child {
            path: "/child".to_owned(),
        }
    };
    println!("tree: {:#?}", tree);
    println!("tree.path: {:#?}", tree.path);
    println!("tree.child.path: {:#?}", tree.child.path);
}

