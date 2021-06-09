use proc_macro::TokenStream;
use proc_macro2::Ident;
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, Token};

mod fn_generics;
mod idents;
mod trait_types;

pub(crate) use idents::Idents;

enum ZeroVGen {
    TraitTypes(trait_types::TraitTypes),
    FnGenerics(fn_generics::FnGenerics),
}

impl Parse for ZeroVGen {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident: Ident = input.parse()?;
        let _comma_token: Option<Token![,]> = input.parse()?;

        match ident.to_string().as_str() {
            "trait_types" => input.parse().map(Self::TraitTypes),
            "fn_generics" => input.parse().map(Self::FnGenerics),
            _ => Err(syn::Error::new(
                ident.span(),
                "expected one of `trait_types` | `fn_generics`",
            )),
        }
    }
}

#[proc_macro_attribute]
pub fn zero_v(args: TokenStream, input: TokenStream) -> TokenStream {
    match parse_macro_input!(args as ZeroVGen) {
        ZeroVGen::TraitTypes(t) => t.generate(input),
        ZeroVGen::FnGenerics(g) => g.generate(input),
    }
}
