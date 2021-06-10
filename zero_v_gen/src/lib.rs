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

/// This macro generates iterations for traits implementing zero_v iteration
/// and generic bounds for functions taking a collection of objects with those
/// traits. It has some limitations at the moment: if your trait's method 
/// arguments involve lifetimes or generics, the generation is likely to fail.
/// If it doesn't then this macro should generate all your boilerplate for you.
///
/// The macro will generate one iteration method on the collection for each of
/// your methods. If your method has signature:
///
/// ```ignore
/// fn {method_name}(&self, input_1: Type1, input_2: Type2, ...) -> OutType
/// ```
///
/// Then the generated method will have signature:
///
/// ```ignore
/// fn iter_{method_name}(&self, input_1: Type1, input_2: Type2, ...) -> impl Iterator<Item=OutType>
/// ```
///
/// # Interface
/// For traits, the interface is very simple.
///
/// ```ignore
/// use zero_v::zero_v;
///
/// #[zero_v(trait_types)]
/// // ... Define your trait here.
/// ```
///
/// For functions you need to provide two extra details. The name of your trait
/// and the type of the argument which accepts a collection of objects
/// implementing it.
///
/// ```ignore
/// #[zero_v(fn_generics, {YourTraitName} as {YourCollectionName})]
/// fn use_trait_collection(arg1: usize, collection: &{YourCollectionName})
/// ```
///
/// # Usage Example
///
/// So putting that all together, you get something like the following example.
///
/// ```
/// use zero_v::{compose, zero_v};
///
/// #[zero_v(trait_types)]
/// trait IntOp {
///     fn execute(&self, input:usize) -> usize;
/// }
///
/// struct Adder {}
///
/// impl IntOp for Adder {
///     fn execute(&self, input: usize) -> usize {
///         input + 1
///     }
/// }
///
/// #[zero_v(fn_generics, IntOp as IntOps)]
/// fn get_intops_sum(input: usize, collection: IntOps) -> usize {
///     collection.iter_execute(input).sum()
/// }
///
/// fn main() {
///     let collection = compose!(Adder {}, Adder {});
///     let result = get_intops_sum(1, collection);
///     println!("{}", result);
/// }
/// ```
#[proc_macro_attribute]
pub fn zero_v(args: TokenStream, input: TokenStream) -> TokenStream {
    match parse_macro_input!(args as ZeroVGen) {
        ZeroVGen::TraitTypes(t) => t.generate(input),
        ZeroVGen::FnGenerics(g) => g.generate(input),
    }
}
