use proc_macro::TokenStream;

#[proc_macro]
pub fn nested(input: TokenStream) -> TokenStream {
    for i in input.clone().into_iter() {
        println!("{:#?}", i);
    }
    input
}