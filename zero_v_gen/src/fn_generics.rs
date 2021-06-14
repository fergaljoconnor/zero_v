use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, parse_quote, ItemFn, Token, WherePredicate};

use crate::Idents;

pub(crate) struct FnGenerics {
    trait_ident: Ident,
    _as: Token![as],
    type_name: Ident,
}

impl FnGenerics {
    pub(crate) fn generate(&self, input: TokenStream) -> TokenStream {
        let type_name = &self.type_name;
        let idents = Idents::from_ident(self.trait_ident.clone());
        let mut f = parse_macro_input!(input as ItemFn);

        let level_trait = idents.level_trait();
        let iter_trait = idents.iter_trait();

        let generics = f.sig.generics.clone();
        let mut iter_generics = generics.clone();
        iter_generics.params.push(parse_quote! { NodeType });

        f.sig.generics.params.push(parse_quote! { NodeType });
        f.sig.generics.params.push(parse_quote! { #type_name });
        f.sig
            .generics
            .make_where_clause()
            .predicates
            .extend::<Vec<WherePredicate>>(vec![
                parse_quote! { NodeType: NextNode + #level_trait #generics },
                parse_quote! { #type_name: #iter_trait #iter_generics },
            ]);

        TokenStream::from(quote! { #f })
    }
}

impl Parse for FnGenerics {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            trait_ident: input.parse()?,
            _as: input.parse()?,
            type_name: input.parse()?,
        })
    }
}
