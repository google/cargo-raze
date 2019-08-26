use proc_macro2;

use ast;
use attr;
use matcher;
use syn;
use utils;

pub fn derive(input: &ast::Input) -> proc_macro2::TokenStream {
    let debug_trait_path = debug_trait_path();
    let fmt_path = fmt_path();

    let body = matcher::Matcher::new(matcher::BindingStyle::Ref).build_arms(
        input,
        |_, arm_name, style, attrs, bis| {
            let field_prints = bis.iter().filter_map(|bi| {
                if bi.field.attrs.ignore_debug() {
                    return None;
                }

                if attrs.debug_transparent() {
                    return Some(quote!{
                        #debug_trait_path::fmt(__arg_0, __f)
                    });
                }

                let arg = &bi.ident;

                let dummy_debug = bi.field.attrs.debug_format_with().map(|format_fn| {
                    format_with(bi.field, &arg, format_fn, input.generics.clone())
                });

                let builder = if let Some(ref name) = bi.field.ident {
                    let name = name.to_string();
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
            let method = syn::Ident::new(method, proc_macro2::Span::call_site());

            if attrs.debug_transparent() {
                quote! {
                    #(#field_prints)*
                }
            } else {
                let name = arm_name.to_string();
                quote! {
                    let mut builder = __f.#method(#name);
                    #(#field_prints)*
                    builder.finish()
                }
            }
        },
    );

    let name = &input.ident;

    let generics = utils::build_impl_generics(
        input,
        &debug_trait_path,
        needs_debug_bound,
        |field| field.debug_bound(),
        |input| input.debug_bound(),
    );
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    quote! {
        #[allow(unused_qualifications)]
        impl #impl_generics #debug_trait_path for #name #ty_generics #where_clause {
            fn fmt(&self, __f: &mut #fmt_path::Formatter) -> #fmt_path::Result {
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
    if cfg!(feature = "use_core") {
        parse_quote!(::core::fmt::Debug)
    } else {
        parse_quote!(::std::fmt::Debug)
    }
}

/// Return the path of the `fmt` module, that is `::std::fmt`.
fn fmt_path() -> syn::Path {
    if cfg!(feature = "use_core") {
        parse_quote!(::core::fmt)
    } else {
        parse_quote!(::std::fmt)
    }
}

/// Return the path of the `PhantomData` type, that is `::std::marker::PhantomData`.
fn phantom_path() -> syn::Path {
    if cfg!(feature = "use_core") {
        parse_quote!(::core::marker::PhantomData)
    } else {
        parse_quote!(::std::marker::PhantomData)
    }
}

fn format_with(
    f: &ast::Field,
    arg_n: &syn::Ident,
    format_fn: &syn::Path,
    mut generics: syn::Generics,
) -> proc_macro2::TokenStream {
    let debug_trait_path = debug_trait_path();
    let fmt_path = fmt_path();
    let phantom_path = phantom_path();

    let ctor_generics = generics.clone();
    let (_, ctor_ty_generics, _) = ctor_generics.split_for_impl();

    generics
        .make_where_clause()
        .predicates
        .extend(f.attrs.debug_bound().unwrap_or(&[]).iter().cloned());

    generics
        .params
        .push(syn::GenericParam::Lifetime(syn::LifetimeDef::new(
            parse_quote!('_derivative),
        )));
    let where_predicates = generics
        .type_params()
        .map(|ty| {
            let mut bounds = syn::punctuated::Punctuated::new();
            bounds.push(syn::TypeParamBound::Lifetime(syn::Lifetime::new(
                "'_derivative",
                proc_macro2::Span::call_site(),
            )));

            let path = syn::Path::from(syn::PathSegment::from(ty.ident.clone()));

            syn::WherePredicate::Type(syn::PredicateType {
                lifetimes: None,
                bounded_ty: syn::Type::Path(syn::TypePath { qself: None, path: path }),
                colon_token: Default::default(),
                bounds: bounds,
            })
        })
        .collect::<Vec<_>>();
    generics
        .make_where_clause()
        .predicates
        .extend(where_predicates);

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let ty = f.ty;

    // Leave off the type parameter bounds, defaults, and attributes
    let phantom = generics.type_params().map(|tp| &tp.ident);

    quote!(
        let #arg_n = {
            struct Dummy #ty_generics (&'_derivative #ty, #phantom_path <(#(#phantom),*)>) #where_clause;

            impl #impl_generics #debug_trait_path for Dummy #ty_generics #where_clause {
                fn fmt(&self, __f: &mut #fmt_path::Formatter) -> #fmt_path::Result {
                    #format_fn(&self.0, __f)
                }
            }

            Dummy:: #ctor_ty_generics (#arg_n, #phantom_path)
        };
    )
}
