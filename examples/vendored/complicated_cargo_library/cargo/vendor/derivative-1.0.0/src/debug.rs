use ast;
use attr;
use matcher;
use quote;
use syn::{self, aster};
use utils;

pub fn derive(input: &ast::Input) -> quote::Tokens {
    let body = matcher::Matcher::new(matcher::BindingStyle::Ref)
        .build_arms(input, |_, arm_name, style, attrs, bis| {
            let field_prints = bis.iter().filter_map(|bi| {
                if bi.field.attrs.ignore_debug() {
                    return None;
                }

                if attrs.debug_transparent() {
                    return Some(quote!{
                        ::std::fmt::Debug::fmt(__arg_0, __f)
                    });
                }

                let arg = &bi.ident;

                let dummy_debug = bi.field.attrs.debug_format_with().map(|format_fn| {
                    format_with(bi.field, &arg, format_fn, input.generics.clone())
                });

                let builder = if let Some(ref name) = bi.field.ident {
                    let name = name.as_ref();
                    quote! {
                        #dummy_debug
                        let _ = builder.field(#name, &#arg);
                    }
                } else {
                    quote! {
                        #dummy_debug
                        let _ = builder.field(&#arg);
                    }
                };

                Some(builder)
            });

            let method = match style {
                ast::Style::Struct => "debug_struct",
                ast::Style::Tuple | ast::Style::Unit => "debug_tuple",
            };
            let method = syn::Ident::new(method);

            let name = arm_name.as_ref();

            if attrs.debug_transparent() {
                quote! {
                    #(#field_prints)*
                }
            } else {
                quote! {
                    let mut builder = __f.#method(#name);
                    #(#field_prints)*
                    builder.finish()
                }
            }
        });

    let name = &input.ident;

    let debug_trait_path = debug_trait_path();
    let impl_generics = utils::build_impl_generics(
        input,
        &debug_trait_path,
        needs_debug_bound,
        |field| field.debug_bound(),
        |input| input.debug_bound(),
    );
    let where_clause = &impl_generics.where_clause;

    let ty = syn::aster::ty()
        .path()
        .segment(name.clone())
        .with_generics(impl_generics.clone())
        .build()
        .build();

    quote! {
        #[allow(unused_qualifications)]
        impl #impl_generics #debug_trait_path for #ty #where_clause {
            fn fmt(&self, __f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                match *self {
                    #body
                }
            }
        }
    }
}

fn needs_debug_bound(attrs: &attr::Field) -> bool {
    !attrs.ignore_debug() && attrs.debug_bound().is_none()
}

/// Return the path of the `Debug` trait, that is `::std::fmt::Debug`.
fn debug_trait_path() -> syn::Path {
    aster::path().global().ids(&["std", "fmt", "Debug"]).build()
}

fn format_with(
    f: &ast::Field,
    arg_n: &syn::Ident,
    format_fn: &syn::Path,
    mut generics: syn::Generics,
) -> quote::Tokens {
    let debug_trait_path = debug_trait_path();

    let ctor_generics = generics.clone();
    let (_, ctor_ty_generics, _) = ctor_generics.split_for_impl();

    generics.where_clause.predicates.extend(f.attrs.debug_bound().unwrap_or(&[]).iter().cloned());

    generics.lifetimes.push(syn::LifetimeDef::new("'_derivative"));
    for ty in &generics.ty_params {
        let path = aster::path::PathBuilder::new().id(&ty.ident).build();
        generics.where_clause.predicates
            .push(syn::WherePredicate::BoundPredicate(syn::WhereBoundPredicate {
                bound_lifetimes: vec![],
                bounded_ty: syn::Ty::Path(None, path),
                bounds: vec![syn::TyParamBound::Region(syn::Lifetime::new("'_derivative"))],
            }));
    }

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let ty = f.ty;

    // Leave off the type parameter bounds, defaults, and attributes
    let phantom = generics.ty_params.iter().map(|tp| &tp.ident);

    quote!(
        let #arg_n = {
            struct Dummy #ty_generics (&'_derivative #ty, ::std::marker::PhantomData <(#(#phantom),*)>) #where_clause;

            impl #impl_generics #debug_trait_path for Dummy #ty_generics #where_clause {
                fn fmt(&self, __f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    #format_fn(&self.0, __f)
                }
            }

            Dummy:: #ctor_ty_generics (#arg_n, ::std::marker::PhantomData)
        };
    )
}
