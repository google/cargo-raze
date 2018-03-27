use ast;
use attr;
use quote;
use syn::{self, aster};
use utils;

/// Derive `Default` for `input`.
pub fn derive(input: &ast::Input, default: &attr::InputDefault) -> quote::Tokens {
    fn make_variant_data(
        variant_name: quote::Tokens,
        style: ast::Style,
        fields: &[ast::Field],
    ) -> quote::Tokens {
        match style {
            ast::Style::Struct => {
                let mut defaults = Vec::new();

                for f in fields {
                    let name = f.ident.as_ref().expect("A structure field must have a name");
                    let default = f.attrs.default_value().map_or_else(
                        || quote!(::std::default::Default::default()),
                        |v| quote!(#v),
                    );

                    defaults.push(quote!(#name: #default));
                }

                quote!(#variant_name { #(#defaults),* })
            }
            ast::Style::Tuple => {
                let mut defaults = Vec::new();

                for f in fields {
                    let default = f.attrs.default_value().map_or_else(
                        || quote!(::std::default::Default::default()),
                        |v| quote!(#v),
                    );

                    defaults.push(default);
                }

                quote!(#variant_name ( #(#defaults),* ))
            }
            ast::Style::Unit => quote!(#variant_name),
        }
    }

    let name = &input.ident;
    let default_trait_path = default_trait_path();
    let impl_generics = utils::build_impl_generics(
        input,
        &default_trait_path,
        |attrs| attrs.default_bound().is_none(),
        |field| field.default_bound(),
        |input| input.default_bound(),
    );
    let where_clause = &impl_generics.where_clause;

    let ty = syn::aster::ty()
        .path()
        .segment(name.clone())
        .with_generics(impl_generics.clone())
        .build()
        .build();

    let body = match input.body {
        ast::Body::Enum(ref data) => {
            let arms = data.iter().filter_map(|variant| {
                if variant.attrs.default.is_some() {
                    let vname = &variant.ident;

                    Some(make_variant_data(quote!(#name::#vname), variant.style, &variant.fields))
                } else {
                    None
                }
            });

            quote!(#(#arms),*)
        }
        ast::Body::Struct(style, ref vd) => {
            make_variant_data(quote!(#name), style, vd)
        }
    };

    let new_fn = if default.new {
        Some(quote!(
            #[allow(unused_qualifications)]
            impl #impl_generics #ty #where_clause {
                /// Creates a default value for this type.
                #[inline]
                pub fn new() -> Self {
                    #default_trait_path::default()
                }
            }
        ))
    } else {
        None
    };

    quote!(
        #new_fn

        #[allow(unused_qualifications)]
        impl #impl_generics #default_trait_path for #ty #where_clause {
            fn default() -> Self {
                #body
            }
        }
    )
}

/// Return the path of the `Default` trait, that is `::std::default::Default`.
fn default_trait_path() -> syn::Path {
    aster::path().global().ids(&["std", "default", "Default"]).build()
}
