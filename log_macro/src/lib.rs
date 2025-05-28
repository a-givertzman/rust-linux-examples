mod macro_input;

use macro_input::MacroInput;
use proc_macro::TokenStream;
use quote::quote;
use syn::ItemFn;
use strfmt::strfmt;


// #[proc_macro_derive(Dbg)]
// pub fn derive_answer_fn(input: TokenStream) -> TokenStream {
//     let input = syn::parse_macro_input!(input as DeriveInput);
//     if let syn::Data::Struct(ref data) = input.data {
//     }
//     TokenStream::from(
//         syn::Error::new(
//             input.ident.span(),
//             "Only structs with named fields can derive `FromRow`"
//         ).to_compile_error()
//     )
// }


#[proc_macro_attribute]
pub fn dbg(args: TokenStream, input: TokenStream) -> TokenStream {
    println!("attrs: {:#?}", args);
    // log_duration_impl(args, item)
    let item = input.clone();
    let item = syn::parse_macro_input!(item as ItemFn);

    let ItemFn {
        // The function signature
        sig,
        // The visibility specifier of this function
        vis,
        // The function block or body
        block,
        // Other attributes applied to this function
        attrs,
    } = item;

    // Extract statements in the body of the functions
    let statements = block.stmts;

    // Store the function identifier for logging
    let function_identifier = sig.ident.clone();
    // println!("function: {}", function_identifier);
    // println!("statements: {}", stringify!(#statements));
    // Reconstruct the function as output using parsed input
    quote!(
        // Reapply all the other attributes on this function.
        // The compiler doesn't include the macro we are
        // currently working in this list.
        #(#attrs)*
        // Reconstruct the function declaration
        #vis #sig {
            // At the beginning of the function, create an instance of `Instant`
            // let __dbg = std::time::Instant::now();
            let __fn_name = stringify!(#function_identifier);

            // Create a new block, the body of which is the body of the function.
            // Store the return value of this block as a variable so that we can
            // return it later from the parent function.
            println!("function: {}", stringify!(#function_identifier));
            
            // Log the duration information for this function
            // println!("{} took {}μs", stringify!(#function_identifier), __start.elapsed().as_micros());
            
            let __result = {
                #(#statements)*
            };
            // Return the result (if any)
            return __result;
        }
    ).into()
}

#[proc_macro]
pub fn debug(tokens: TokenStream) -> TokenStream {
    println!("tokens: {:#?}", tokens);
    // Parse input as a string literal
    let value = syn::parse_macro_input!(tokens as MacroInput);
    // let value = syn::parse_macro_input!(tokens as syn::Expr);
    // let value = syn::punctuated::Punctuated::<syn::Expr, syn::Token![,]>::parse_terminated
    //     .parse(item)
    //     .unwrap();
    let f = &value.fmt;
    let vals = &value.values;
    let vals = strfmt(f, vals);
    quote!(
        // println!("{}.{} | {:?}", self.dbg, __fn_name, #value.);
    ).into()
}
