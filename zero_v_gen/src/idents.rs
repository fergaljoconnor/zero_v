use convert_case::{Case, Casing};
use proc_macro2::Ident;

use quote::format_ident;
use syn::{ItemTrait, TraitItem};

pub(crate) struct Idents {
    main: Ident,
    main_methods: Vec<Ident>,
}

impl Idents {
    pub(crate) fn from_trait(main: ItemTrait) -> Self {
        let main_methods = main.items.into_iter().filter_map(|i| match i {
            TraitItem::Method(m) => Some(m.sig.ident),
            _ => None,
        });

        Self {
            main: main.ident,
            main_methods: main_methods.collect(),
        }
    }

    pub(crate) fn from_ident(main: Ident) -> Self {
        Self {
            main,
            main_methods: vec![],
        }
    }

    pub(crate) fn level_trait(&self) -> Ident {
        format_ident!("{}AtLevel", self.main)
    }

    pub(crate) fn level_methods<'a>(&'a self) -> impl Iterator<Item = Ident> + 'a {
        self.main_methods
            .iter()
            .map(|m| format_ident!("{}_at_level", m))
    }

    pub(crate) fn iter_trait(&self) -> Ident {
        format_ident!("Iter{}", self.main)
    }

    pub(crate) fn iter_methods<'a>(&'a self) -> impl Iterator<Item = Ident> + 'a {
        self.main_methods
            .iter()
            .map(|m| format_ident!("iter_{}", m))
    }

    pub(crate) fn composite_iters<'a>(&'a self) -> impl Iterator<Item = Ident> + 'a {
        self.main_methods.iter().map(|m| {
            format_ident!(
                "CompositeIterator{}",
                m.to_string().to_case(Case::UpperCamel)
            )
        })
    }
}
