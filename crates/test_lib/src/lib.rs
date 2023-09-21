use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use strum::VariantNames;

// A function that can take a given enum type and generates a From implementation
// The target is assumed to have the exact matching variant names
pub fn into_matching_enum<E: VariantNames>(target_name: &str) -> TokenStream {
    let from_name = syn::parse_str::<syn::Path>(std::any::type_name::<E>()).unwrap();
    let target_name = format_ident!("{}", target_name);
    let variant_names = E::VARIANTS.iter().map(|item| format_ident!("{}", item));

    quote!(
        impl From<#from_name> for #target_name {
            fn from(value: #from_name) -> Self {
                match value {
                    #(#from_name::#variant_names => Self::#variant_names),*
                }
            }
        }
    )
}
