#![recursion_limit="256"]

extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::{Body, Field, Ident, Lifetime, LifetimeDef, MacroInput, Ty, TyParam, VariantData,
          WhereClause};
use quote::Tokens;

/// Used to `#[derive]` the trait
/// `SystemData`.
#[proc_macro_derive(SystemData)]
pub fn system_data(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_macro_input(&s).unwrap();

    let gen = impl_system_data(&ast);

    gen.parse().expect("Invalid")
}

fn impl_system_data(ast: &MacroInput) -> Tokens {
    let name = &ast.ident;
    let lifetime_defs = &ast.generics.lifetimes;
    let ty_params = &ast.generics.ty_params;
    let where_clause = &ast.generics.where_clause;

    let (fetch_return, tys) = gen_from_body(&ast.body, name);
    let tys = &tys;
    // Assumes that the first lifetime is the fetch lt
    let def_fetch_lt = lifetime_defs
        .iter()
        .next()
        .expect("There has to be at least one lifetime");
    let ref impl_fetch_lt = def_fetch_lt.lifetime;
    let def_lt_tokens = gen_def_lt_tokens(lifetime_defs);
    let impl_lt_tokens = gen_impl_lt_tokens(lifetime_defs);
    let def_ty_params = gen_def_ty_params(ty_params);
    let impl_ty_params = gen_impl_ty_params(ty_params);
    let where_clause = gen_where_clause(where_clause, impl_fetch_lt, tys);
    // Reads and writes are taken from the same types,
    // but need to be cloned before.

    quote! {
        impl< #def_lt_tokens , #def_ty_params >
            ::shred::SystemData< #impl_fetch_lt >
            for #name< #impl_lt_tokens , #impl_ty_params >
            where #where_clause
        {
            fn fetch(res: & #impl_fetch_lt ::shred::Resources, id: usize) -> Self {
                #fetch_return
            }

            fn reads(id: usize) -> Vec<::shred::ResourceId> {
                let mut r = Vec::new();

                #( {
                        let mut reads = <#tys as ::shred::SystemData> :: reads(id);
                        r.append(&mut reads);
                    } )*

                r
            }

            fn writes(id: usize) -> Vec<::shred::ResourceId> {
                let mut r = Vec::new();

                #( {
                        let mut writes = <#tys as ::shred::SystemData> :: writes(id);
                        r.append(&mut writes);
                    } )*

                r
            }
        }
    }
}

fn collect_field_types(fields: &Vec<Field>) -> Vec<Ty> {
    fields.iter().map(|x| x.ty.clone()).collect()
}

fn gen_identifiers(fields: &Vec<Field>) -> Vec<Ident> {
    fields.iter().map(|x| x.ident.clone().unwrap()).collect()
}

fn gen_def_lt_tokens(lifetime_defs: &Vec<LifetimeDef>) -> Tokens {
    let lts: Vec<Tokens> = lifetime_defs
        .iter()
        .map(|x| {
            let ref lt = x.lifetime;
            let ref bounds = x.bounds;

            if bounds.is_empty() {
                quote! { #lt }
            } else {
                quote! { #lt: #( #bounds )+* }
            }
        })
        .collect();

    quote! { #( #lts ),* }
}

fn gen_impl_lt_tokens(lifetime_defs: &Vec<LifetimeDef>) -> Tokens {
    let lts: Vec<Lifetime> = lifetime_defs.iter().map(|x| x.lifetime.clone()).collect();

    quote! { #( #lts ),* }
}

fn gen_def_ty_params(ty_params: &Vec<TyParam>) -> Tokens {
    let ty_params: Vec<Tokens> = ty_params
        .iter()
        .map(|x| {
            let ref ty = x.ident;
            let ref bounds = x.bounds;

            quote! { #ty: #( #bounds )+* }
        })
        .collect();

    quote! { #( #ty_params ),* }
}

fn gen_impl_ty_params(ty_params: &Vec<TyParam>) -> Tokens {
    let ty_params: Vec<Ident> = ty_params.iter().map(|x| x.ident.clone()).collect();

    quote! { #( #ty_params ),* }
}

fn gen_where_clause(clause: &WhereClause, fetch_lt: &Lifetime, tys: &Vec<Ty>) -> Tokens {
    let user_predicates = clause.predicates.iter().map(|x| quote! { #x });
    let system_data_predicates = tys.iter()
        .map(|ty| {
            quote! { #ty : ::shred::SystemData< #fetch_lt > }
        });

    let mut tokens = Tokens::new();
    tokens.append_separated(user_predicates.chain(system_data_predicates), ",");

    tokens
}

fn gen_from_body(ast: &Body, name: &Ident) -> (Tokens, Vec<Ty>) {
    enum BodyType {
        Struct,
        Tuple,
    }

    let (body, fields) = match *ast {
        Body::Struct(VariantData::Struct(ref x)) => (BodyType::Struct, x),
        Body::Struct(VariantData::Tuple(ref x)) => (BodyType::Tuple, x),
        _ => panic!("Enums are not supported"),
    };

    let tys = collect_field_types(fields);

    let fetch_return = match body {
        BodyType::Struct => {
            let identifiers = gen_identifiers(fields);

            quote! {
                #name {
                    #( #identifiers: ::shred::SystemData::fetch(res, id) ),*
                }
            }
        }
        BodyType::Tuple => {
            let count = tys.len();
            let fetch = vec![quote! { ::shred::SystemData::fetch(res, id) }; count];

            quote! {
                #name ( #( #fetch ),* )
            }
        }
    };

    (fetch_return, tys)
}
