use syn::{parse::Parse};

pub(crate) struct MacroInput {
    pub fmt: syn::PatLit,
    // comma: syn::Token![,],
    pub values: Vec<syn::Expr>,
}

impl Parse for MacroInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let fmt = input.parse()?;
        let mut values = vec![];
        while !input.is_empty() {
            let _comma: syn::Token![,] = input.parse()?;
            let val: syn::Expr = input.parse()?;
            values.push(val);
        }
        Ok(Self {
            fmt,
            values,
        })
    }
}
