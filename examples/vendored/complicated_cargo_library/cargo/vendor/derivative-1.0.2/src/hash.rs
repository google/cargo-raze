use proc_macro2;

use ast;
use attr;
use matcher;
use syn;
use utils;

pub fn derive(input: &ast::Input) -> proc_macro2::TokenStream {
    let hasher_trait_path = hasher_trait_path();
    let hash_trait_path = hash_trait_path();

    let body = matcher::Matcher::new(matcher::BindingStyle::Ref).build_arms(
        input,
        |arm_path, _, _, _, bis| {
            let field_prints = bis.iter().filter_map(|bi| {
                if bi.field.attrs.ignore_hash() {
                    return None;
                }

                let arg = &bi.ident;

                if let Some(hash_with) = bi.field.attrs.hash_with() {
                    Some(quote! {
                        #hash_with(#arg, __state);
                    })
                } else {
                    Some(quote! {
                        #hash_trait_path::hash(#arg, __state);
                    })
                }
            });

            let variant = if let ast::Body::Enum(_) = input.body {
                Some(quote!(
                    #hash_trait_path::hash(&(#arm_path as u64), __state);
                ))
            } else {
                None
            };

            quote! {
                #variant
                #(#field_prints)*
            }
        },
    );

    let name = &input.ident;
    let generics = utils::build_impl_generics(
        input,
        &hash_trait_path,
        needs_hash_bound,
        |field| field.hash_bound(),
        |input| input.hash_bound(),
    );
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let hasher_ty_parameter = utils::hygienic_type_parameter(input, "__H");
    quote! {
        #[allow(unused_qualifications)]
        impl #impl_generics #hash_trait_path for #name #ty_generics #where_clause {
            fn hash<#hasher_ty_parameter>(&self, __state: &mut #hasher_ty_parameter)
                where #hasher_ty_parameter: #hasher_trait_path
            {
                match *self {
                    #body
                }
            }
        }
    }
}

fn needs_hash_bound(attrs: &attr::Field) -> bool {
    !attrs.ignore_hash() && attrs.hash_bound().is_none()
}

/// Return the path of the `Hash` trait, that is `::std::hash::Hash`.
fn hash_trait_path() -> syn::Path {
    if cfg!(feature = "use_core") {
        parse_quote!(::core::hash::Hash)
    } else {
        parse_quote!(::std::hash::Hash)
    }
}

/// Return the path of the `Hasher` trait, that is `::std::hash::Hasher`.
fn hasher_trait_path() -> syn::Path {
    if cfg!(feature = "use_core") {
        parse_quote!(::core::hash::Hasher)
    } else {
        parse_quote!(::std::hash::Hasher)
    }
}
