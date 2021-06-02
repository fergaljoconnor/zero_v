use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{
    parse_macro_input, parse_quote, FnArg, ItemFn, ItemTrait, Pat, PatType, ReturnType, Token,
    TraitItem, Type,
};

struct Idents {
    main: Ident,
    main_methods: Vec<Ident>,
}

impl Idents {
    pub fn from_trait(main: ItemTrait) -> Self {
        let main_methods = main.items.into_iter().filter_map(|i| match i {
            TraitItem::Method(m) => Some(m.sig.ident),
            _ => None,
        });

        Self {
            main: main.ident,
            main_methods: main_methods.collect(),
        }
    }

    pub fn from_ident(main: Ident) -> Self {
        Self {
            main,
            main_methods: vec![],
        }
    }

    pub fn level_trait(&self) -> Ident {
        format_ident!("{}AtLevel", self.main)
    }

    pub fn level_methods<'a>(&'a self) -> impl Iterator<Item = Ident> + 'a {
        self.main_methods
            .iter()
            .map(|m| format_ident!("{}_at_level", m))
    }

    pub fn iter_trait(&self) -> Ident {
        format_ident!("Iter{}", self.main)
    }

    pub fn iter_methods<'a>(&'a self) -> impl Iterator<Item = Ident> + 'a {
        self.main_methods
            .iter()
            .map(|m| format_ident!("iter_{}", m))
    }

    pub fn composite_iters<'a>(&'a self) -> impl Iterator<Item = Ident> + 'a {
        self.main_methods.iter().map(|m| {
            format_ident!(
                "CompositeIterator{}",
                m.to_string().to_case(Case::UpperCamel)
            )
        })
    }
}

#[proc_macro_attribute]
pub fn zero_v_gen(_args: TokenStream, input: TokenStream) -> TokenStream {
    let trait_type = parse_macro_input!(input as ItemTrait);
    let idents = Idents::from_trait(trait_type.clone());
    let trait_ident = &trait_type.ident;
    let trait_methods = || {
        trait_type.items.iter().filter_map(|i| match i {
            TraitItem::Method(m) => Some(m),
            _ => None,
        })
    };

    let trait_method_idents: Vec<Ident> = trait_methods().map(|m| m.sig.ident.clone()).collect();
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
    let iter_methods: Vec<Ident> = idents.iter_methods().collect();
    let composite_iters: Vec<Ident> = idents.composite_iters().collect();

    let tokens = quote! {
        use zero_v::{Composite, NextNode, Node};
        #trait_type

        trait #level_trait {
            #(
                fn #level_methods(#level_method_inputs, level: usize) -> #level_method_outputs;
            )*
        }

        impl #level_trait for () {
            #(
                #[allow(unused)]
                fn #level_methods(#level_method_inputs, level: usize) -> #level_method_outputs {
                    None
                }
            )*
        }

        impl<A: #trait_ident, B: NextNode + #level_trait> #level_trait for Node<A, B> {
            #(
                fn #level_methods(#level_method_inputs, level: usize) -> #level_method_outputs {
                    if level != 0 {
                        self.next.#level_methods(#trait_method_args, level - 1)
                    } else {
                        Some(self.data.#trait_method_idents(#trait_method_args))
                    }
                }
            )*
        }

        trait #iter_trait<NodeType: NextNode + #level_trait> {
            #(
                fn #iter_methods(#level_method_inputs) -> #composite_iters<'_, NodeType>;
            )*
        }

        impl<Nodes: NextNode + #level_trait> #iter_trait<Nodes> for Composite<Nodes> {
            #(
                fn #iter_methods(#level_method_inputs) -> #composite_iters<'_, Nodes> {
                    #composite_iters::new(&self.head, #trait_method_args)
                }
            )*
        }

        #(
            struct #composite_iters<'a, Nodes: NextNode + #level_trait> {
                level: usize,
                #trait_method_inputs,
                parent: &'a Nodes,
            }


            impl<'a, Nodes: NextNode + #level_trait> #composite_iters<'a, Nodes> {
                fn new(parent: &'a Nodes, #trait_method_inputs) -> Self {
                    Self {
                        parent,
                        #trait_method_args,
                        level: 0,
                    }
                }
            }

            impl<'a, Nodes: NextNode + #level_trait> Iterator for #composite_iters<'a, Nodes> {
                type Item = #trait_method_outputs;

                #[inline]
                fn next(&mut self) -> Option<Self::Item> {
                    let result = self.parent.#level_methods(#trait_method_self_args, self.level);
                    self.level += 1;
                    result
                }
            }
        )*
    };

    TokenStream::from(tokens)
}

struct Composed {
    trait_ident: Ident,
    _as: Token![as],
    type_name: Ident,
}

impl Parse for Composed {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            trait_ident: input.parse()?,
            _as: input.parse()?,
            type_name: input.parse()?,
        })
    }
}

#[proc_macro_attribute]
pub fn composed(args: TokenStream, input: TokenStream) -> TokenStream {
    let composed = parse_macro_input!(args as Composed);
    let type_name = &composed.type_name;
    let idents = Idents::from_ident(composed.trait_ident.clone());
    let mut f = parse_macro_input!(input as ItemFn);

    let level_trait = idents.level_trait();
    let iter_trait = idents.iter_trait();

    f.sig.generics.params.push(parse_quote! { NodeType });
    f.sig.generics.params.push(parse_quote! { #type_name });
    f.sig.generics.where_clause = Some(parse_quote! {
        where NodeType: #level_trait + NextNode,
              #type_name: #iter_trait<NodeType>
    });

    TokenStream::from(quote! { #f })
}
