use std::env;
use nestify::nest;
use regex::Regex;
use struct_iterable::Iterable;


fn to_snack_case(val: &str) -> String {
    let re = Regex::new(r"[A-Z]").unwrap();
    val.chars().fold(String::new(), |mut acc, char| {
        let char = char.to_string();
        let is_caps = re.is_match(&char);
        if is_caps {
            if !acc.is_empty() {
                acc.push_str("_");
                // acc.push_str(&char.to_ascii_lowercase());
            }
            acc.push_str(&char.to_ascii_lowercase());
        } else {
            acc.push_str(&char);
        }
        acc
    })
}
///
/// 
fn just_method() -> String {
    "child".to_owned()
}
///
///
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
    // just_method,
}





fn main() {
    env::set_var("RUST_LOG", "debug");
    env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let tree = Parent {
        path: "/parent".to_owned(),
        child: Child {
            path: "/child".to_owned(),
        }
    };
    println!("tree: {:#?}", tree);
    println!("tree.path: {:#?}", tree.path);
    println!("tree.child.path: {:#?}", tree.child.path);
    let cam = "SomeCamelCaseString";
    println!("'{}': '{}'", cam, to_snack_case(cam));
}

