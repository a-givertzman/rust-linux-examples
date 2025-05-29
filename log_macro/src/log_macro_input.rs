use syn::punctuated::Punctuated;

///
/// ### Extracts `TokenStream` for function-like proc macro
/// 
/// Used for passing function arguments into `println(fmt, vals)` or `log::info(fmt, vals)`
/// 
/// Where function signature:
/// ```ignore
/// fn debug("{}: {}", a, b)
/// ```
/// 
/// **`"{}: {}"`** - will be extracted into `self.fmt`
/// 
/// **`a`, `b`** - will be extracted into `self.values`
pub(super) struct LogMacroInput {
    pub fmt: syn::PatLit,
    // pub vals: Vec<syn::Expr>,
    pub vals: Punctuated<syn::Expr, syn::Token![,]>,
}
//
//
impl syn::parse::Parse for LogMacroInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let fmt: syn::PatLit = input.parse()?;
        let _comma: Result<syn::Token![,], syn::Error> = input.parse();
        Ok(Self {
            fmt,
            vals: input.parse_terminated(syn::Expr::parse, syn::Token![,])?,
        })
    }
}
