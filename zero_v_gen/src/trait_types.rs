use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{
    parse_macro_input, parse_quote, FnArg, GenericParam, ItemTrait, Pat, PatType, ReturnType,
    TraitItem, Type, WherePredicate,
};

use crate::Idents;

pub(crate) struct TraitTypes;

impl TraitTypes {
    pub(crate) fn generate(&self, input: TokenStream) -> TokenStream {
        let trait_type = parse_macro_input!(input as ItemTrait);
        let trait_generics = &trait_type.generics;
        let (impl_generics, ty_generics, where_clause) = trait_type.generics.split_for_impl();
        let idents = Idents::from_trait(trait_type.clone());
        let trait_ident = &trait_type.ident;
        let trait_methods = || {
            trait_type.items.iter().filter_map(|i| match i {
                TraitItem::Method(m) => Some(m),
                _ => None,
            })
        };

        let trait_method_idents: Vec<Ident> =
            trait_methods().map(|m| m.sig.ident.clone()).collect();
        let trait_method_inputs = trait_methods()
            .map(|m| {
                m.sig
                    .inputs
                    .iter()
                    .filter_map(|arg| match arg {
                        FnArg::Typed(_) => Some(arg.clone()),
                        _ => None,
                    })
                    .collect::<Punctuated<FnArg, Comma>>()
            })
            .collect::<Vec<_>>();
        let trait_method_args = trait_methods()
            .map(|m| {
                m.sig
                    .inputs
                    .iter()
                    .filter_map(|arg| match arg {
                        FnArg::Typed(PatType { pat, .. }) => match **pat {
                            Pat::Ident(ref i) => Some(i.ident.clone()),
                            _ => None,
                        },
                        _ => None,
                    })
                    .collect::<Punctuated<Ident, Comma>>()
            })
            .collect::<Vec<_>>();

        let trait_method_self_args = trait_method_args
            .iter()
            .map(|args| {
                let iter = args.iter();
                quote! { #(self.#iter),* }
            })
            .collect::<Vec<_>>();

        let trait_method_outputs: Vec<Type> = trait_methods()
            .map(|m| match &m.sig.output {
                ReturnType::Default => parse_quote! { () },
                ReturnType::Type(_, ty) => *ty.clone(),
            })
            .collect();

        let level_trait = idents.level_trait();
        let mut level_generics = trait_generics.clone();
        let zv_trait_type: GenericParam = parse_quote! { TraitType };
        let zv_trait_type_pred: WherePredicate =
            parse_quote! { TraitType: #trait_ident #ty_generics };
        let zv_node_type: GenericParam = parse_quote! { NodeType };
        let zv_node_type_pred: WherePredicate =
            parse_quote! { NodeType: NextNode + #level_trait #ty_generics };

        let zv_generics = vec![zv_trait_type.clone(), zv_node_type.clone()];
        let zv_where = vec![zv_trait_type_pred.clone(), zv_node_type_pred.clone()];

        level_generics.params.extend(zv_generics);
        level_generics
            .make_where_clause()
            .predicates
            .extend(zv_where);

        let (level_impl_generics, _, level_where_clause) = level_generics.split_for_impl();
        let level_methods: Vec<Ident> = idents.level_methods().collect();
        let level_method_inputs = trait_methods()
            .map(|m| m.sig.inputs.clone())
            .collect::<Vec<_>>();

        let level_method_outputs: Vec<Type> = trait_methods()
            .map(|m| match &m.sig.output {
                ReturnType::Default => parse_quote! { Option<()> },
                ReturnType::Type(_, ty) => parse_quote! { Option<#ty> },
            })
            .collect();

        let iter_trait = idents.iter_trait();
        let mut iter_generics = trait_generics.clone();
        iter_generics.params.push(zv_node_type.clone());
        iter_generics
            .make_where_clause()
            .predicates
            .push(zv_node_type_pred.clone());

        let (iter_impl_generics, iter_ty_generics, iter_where_clause) =
            iter_generics.split_for_impl();
        let iter_methods: Vec<Ident> = idents.iter_methods().collect();

        let composite_iters: Vec<Ident> = idents.composite_iters().collect();
        let mut composite_generics = trait_generics.clone();
        let mut composite_lifetime_generics = composite_generics.clone();
        composite_generics
            .params
            .extend(vec![parse_quote! { '_ }, zv_node_type.clone()]);

        composite_lifetime_generics
            .params
            .extend(vec![parse_quote! { 'zero_v }, zv_node_type.clone()]);

        composite_lifetime_generics
            .make_where_clause()
            .predicates
            .push(zv_node_type_pred.clone());
        let (_, composite_ty_generics, _) = composite_generics.split_for_impl();

        let (composite_impl_generics, composite_lifetime_ty_generics, composite_where_clause) =
            composite_lifetime_generics.split_for_impl();

        let composite_phantom_types = trait_generics
            .params
            .iter()
            .filter_map(|p| match p {
                GenericParam::Type(t) => Some(t.ident.clone()),
                _ => None,
            })
            .collect::<Vec<_>>();

        let composite_phantom_names = composite_phantom_types
            .iter()
            .enumerate()
            .map(|(i, _)| format_ident!("_phantom_{}", i))
            .collect::<Vec<_>>();

        let composite_phantom_fields = quote! {
            #(
                #composite_phantom_names: PhantomData<#composite_phantom_types>,
            )*
        };
        let composite_phantom_vals = quote! {
            #(
                #composite_phantom_names: PhantomData,
            )*
        };
        let tokens = quote! {
            use zero_v::{Composite, NextNode, Node};
            use std::marker::PhantomData;
            #trait_type

            trait #level_trait #trait_generics #where_clause {
                #(
                    fn #level_methods(#level_method_inputs, level: usize) -> #level_method_outputs;
                )*
            }

            impl #impl_generics #level_trait #ty_generics for () #where_clause {
                #(
                    #[allow(unused)]
                    fn #level_methods(#level_method_inputs, level: usize) -> #level_method_outputs {
                        None
                    }
                )*
            }

            impl #level_impl_generics #level_trait #ty_generics
                for Node<#zv_trait_type, #zv_node_type>
            #level_where_clause
            {
                #(
                    fn #level_methods(#level_method_inputs, level: usize)
                        -> #level_method_outputs
                    {
                        if level != 0 {
                            self.next.#level_methods(#trait_method_args, level - 1)
                        } else {
                            Some(self.data.#trait_method_idents(#trait_method_args))
                        }
                    }
                )*
            }

            trait #iter_trait #iter_generics #iter_where_clause {
                #(
                    fn #iter_methods(#level_method_inputs)
                        -> #composite_iters #composite_ty_generics;
                )*
            }

            impl #iter_impl_generics #iter_trait #iter_ty_generics for Composite<#zv_node_type>
            #iter_where_clause
            {
                #(
                    fn #iter_methods(#level_method_inputs)
                        -> #composite_iters #composite_ty_generics
                    {
                        #composite_iters::new(&self.head, #trait_method_args)
                    }
                )*
            }

            #(
                struct #composite_iters #composite_lifetime_generics
                #composite_where_clause
                {
                    level: usize,
                    #trait_method_inputs,
                    parent: &'zero_v #zv_node_type,
                    #composite_phantom_fields
                }

                impl #composite_impl_generics
                     #composite_iters #composite_lifetime_ty_generics
                #composite_where_clause
                {
                    fn new(parent: &'zero_v #zv_node_type, #trait_method_inputs) -> Self {
                        Self {
                            parent,
                            #trait_method_args,
                            level: 0,
                            #composite_phantom_vals
                        }
                    }
                }

                impl #composite_impl_generics Iterator for
                     #composite_iters #composite_lifetime_ty_generics
                #composite_where_clause
                {
                    type Item = #trait_method_outputs;

                    #[inline]
                    fn next(&mut self) -> Option<Self::Item> {
                        let result = self.parent.#level_methods(
                            #trait_method_self_args,
                            self.level
                        );
                        self.level += 1;
                        result
                    }
                }
            )*
        };

        TokenStream::from(tokens)
    }
}

impl Parse for TraitTypes {
    fn parse(_input: ParseStream) -> syn::Result<Self> {
        Ok(Self {})
    }
}
