mod macro_input;
use macro_input::MacroInput;
use proc_macro::TokenStream;
use quote::quote;
use syn::ItemFn;

///
/// Define this attribute above the method,
/// to make possible using a log macros as
/// 
/// ```ignore
/// struct MyStruct {
///     dbg: String,    // any type implements Display, the name of this field must be `dbg`
/// }
/// impl MyStruct {
///     #[dbg]
///     pub fn show(&self, val: usize) {
///         log::debug!("val: {}", val);
///     }
/// }
/// fn main() {
///     let my_struct = MyStruct { dbg: "MyStruct".into() };
///     my_stryct.show(12);       // MyStruct.show | val: 12
/// }
/// ```
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
    let value = syn::parse_macro_input!(tokens as MacroInput);
    let f = &value.fmt;
    let vals = &value.values;
    quote!(
        log::debug!(
            "{}.{} | {:?}",
            self.dbg,
            __fn_name,
            format!(
                #f,
                #(#vals)*
            )
        );
    ).into()
}
