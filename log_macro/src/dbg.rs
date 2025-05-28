
pub(crate) fn log_duration_impl(_args: TokenStream, input: TokenStream) -> TokenStream {
    // Parse the input as `ItemFn` which is a type provided
    // by `syn` to represent a function.
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
    println!("function: {}", stringify!(#function_identifier));

    // Reconstruct the function as output using parsed input
    // quote!(
    //     // Reapply all the other attributes on this function.
    //     // The compiler doesn't include the macro we are
    //     // currently working in this list.
    //     #(#attrs)*
    //     // Reconstruct the function declaration
    //     #vis #sig {
    //         #(#statements)*

    //         // At the beginning of the function, create an instance of `Instant`
    //         // let __dbg = std::time::Instant::now();

    //         // Create a new block, the body of which is the body of the function.
    //         // Store the return value of this block as a variable so that we can
    //         // return it later from the parent function.
    //         // println!("function: {}", stringify!(#function_identifier));
            
    //         // Log the duration information for this function
    //         // println!("{} took {}μs", stringify!(#function_identifier), __start.elapsed().as_micros());
            
    //         // let __result = {
    //         //     #(#statements)*
    //         // };
    //         // Return the result (if any)
    //         // return __result;
    //     }
    // ).into()
    input
}

