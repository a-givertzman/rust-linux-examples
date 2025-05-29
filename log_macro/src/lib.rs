mod log_macro_input;
use log_macro_input::LogMacroInput;
use proc_macro::TokenStream;
use quote::{quote, IdentFragment};
use syn::{spanned::Spanned, ItemFn};

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
    let function_identifier = sig.ident.span().source_text();
    quote!(
        // Reapply all the other attributes on this function.
        // The compiler doesn't include the macro we are
        // currently working in this list.
        #(#attrs)*
        // Reconstruct the function declaration
        #vis #sig {
            // Defining variable containing the function identifier
            let __fn_label = #function_identifier;

            // println!("function: {}", stringify!(#function_identifier));
            
            #(#statements)*
        }
    ).into()
}
///
/// Logs a message at the info level.
#[proc_macro]
pub fn info(tokens: TokenStream) -> TokenStream {
    let value = syn::parse_macro_input!(tokens as LogMacroInput);
    let f = value.fmt;
    let vals = value.vals.into_pairs();
    quote!(
        log::info!(
            "{}.{} | {:?}",
            self.dbg,
            __fn_label,
            format!(
                #f,
                #(#vals)*
            )
        );
    ).into()
}
///
/// Logs a message at the debug level.
#[proc_macro]
pub fn debug(tokens: TokenStream) -> TokenStream {
    let value = syn::parse_macro_input!(tokens as LogMacroInput);
    let f = value.fmt;
    let vals = value.vals.into_pairs();
    quote!(
        log::debug!(
            "{}.{} | {:?}",
            self.dbg,
            __fn_label,
            format!(
                #f,
                #(#vals)*
            )
        );
    ).into()
}
///
/// Logs a message at the trace level.
#[proc_macro]
pub fn trace(tokens: TokenStream) -> TokenStream {
    let value = syn::parse_macro_input!(tokens as LogMacroInput);
    let f = value.fmt;
    let vals = value.vals.into_pairs();
    quote!(
        log::trace!(
            "{}.{} | {:?}",
            self.dbg,
            __fn_label,
            format!(
                #f,
                #(#vals)*
            )
        );
    ).into()
}
///
/// Logs a message at the warn level.
#[proc_macro]
pub fn warn(tokens: TokenStream) -> TokenStream {
    let value = syn::parse_macro_input!(tokens as LogMacroInput);
    let f = value.fmt;
    let vals = value.vals.into_pairs();
    quote!(
        log::warn!(
            "{}.{} | {:?}",
            self.dbg,
            __fn_label,
            format!(
                #f,
                #(#vals)*
            )
        );
    ).into()
}
///
/// Logs a message at the error level.
#[proc_macro]
pub fn error(tokens: TokenStream) -> TokenStream {
    let value = syn::parse_macro_input!(tokens as LogMacroInput);
    let f = value.fmt;
    let vals = value.vals.into_pairs();
    quote!(
        log::error!(
            "{}.{} | {:?}",
            self.dbg,
            __fn_label,
            format!(
                #f,
                #(#vals)*
            )
        );
    ).into()
}
