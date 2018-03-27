/* This file incorporates work covered by the following copyright and
 * permission notice:
 *   Copyright 2016 The serde Developers. See
 *   https://github.com/serde-rs/serde/blob/3f28a9324042950afa80354722aeeee1a55cbfa3/README.md#license.
 *
 *   Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 *   http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 *   <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
 *   option. This file may not be copied, modified, or distributed
 *   except according to those terms.
 */

use ast;
use attr;
use std::collections::HashSet;
use syn::{self, aster, visit};

// use internals::ast::Item;
// use internals::attr;

/// Remove the default from every type parameter because in the generated `impl`s
/// they look like associated types: "error: associated type bindings are not
/// allowed here".
pub fn without_defaults(generics: &syn::Generics) -> syn::Generics {
    syn::Generics {
        ty_params: generics.ty_params
            .iter()
            .map(|ty_param| syn::TyParam { default: None, ..ty_param.clone() })
            .collect(),
        ..generics.clone()
    }
}

pub fn with_where_predicates(
    generics: &syn::Generics,
    predicates: &[syn::WherePredicate],
) -> syn::Generics {
    aster::from_generics(generics.clone())
        .with_predicates(predicates.to_vec())
        .build()
}

pub fn with_where_predicates_from_fields<F>(
    item: &ast::Input,
    generics: &syn::Generics,
    from_field: F,
) -> syn::Generics
    where F: Fn(&attr::Field) -> Option<&[syn::WherePredicate]>,
{
    aster::from_generics(generics.clone())
        .with_predicates(
            item.body
                .all_fields()
                .iter()
                .flat_map(|field| from_field(&field.attrs))
                .flat_map(|predicates| predicates.to_vec()))
        .build()
}

/// Puts the given bound on any generic type parameters that are used in fields
/// for which filter returns true.
///
/// For example, the following structure needs the bound `A: Debug, B: Debug`.
///
/// ```ignore
/// struct S<'b, A, B: 'b, C> {
///     a: A,
///     b: Option<&'b B>
///     #[derivative(Debug="ignore")]
///     c: C,
/// }
/// ```
pub fn with_bound<F>(
    item: &ast::Input,
    generics: &syn::Generics,
    filter: F,
    bound: &syn::Path,
) -> syn::Generics
    where F: Fn(&attr::Field) -> bool,
{
    #[derive(Debug)]
    struct FindTyParams {
        /// Set of all generic type parameters on the current struct (A, B, C in
        /// the example). Initialized up front.
        all_ty_params: HashSet<syn::Ident>,
        /// Set of generic type parameters used in fields for which filter
        /// returns true (A and B in the example). Filled in as the visitor sees
        /// them.
        relevant_ty_params: HashSet<syn::Ident>,
    }
    impl visit::Visitor for FindTyParams {
        fn visit_path(&mut self, path: &syn::Path) {
            if let Some(seg) = path.segments.last() {
                if seg.ident == "PhantomData" {
                    // Hardcoded exception, because `PhantomData<T>` implements
                    // most traits whether or not `T` implements it.
                    return;
                }
            }
            if !path.global && path.segments.len() == 1 {
                let id = path.segments[0].ident.clone();
                if self.all_ty_params.contains(&id) {
                    self.relevant_ty_params.insert(id);
                }
            }
            visit::walk_path(self, path);
        }
    }

    let all_ty_params: HashSet<_> = generics.ty_params
        .iter()
        .map(|ty_param| ty_param.ident.clone())
        .collect();

    let relevant_tys = item.body
        .all_fields()
        .into_iter()
        .filter(|field| filter(&field.attrs))
        .map(|field| &field.ty);

    let mut visitor = FindTyParams {
        all_ty_params: all_ty_params,
        relevant_ty_params: HashSet::new(),
    };
    for ty in relevant_tys {
        visit::walk_ty(&mut visitor, ty);
    }

    aster::from_generics(generics.clone())
        .with_predicates(generics.ty_params
            .iter()
            .map(|ty_param| ty_param.ident.clone())
            .filter(|id| visitor.relevant_ty_params.contains(id))
            .map(|id| {
                aster::where_predicate()
                    // the type parameter that is being bounded e.g. `T`
                    .bound().build(aster::ty().id(id))
                    // the bound e.g. `Debug`
                    .bound().trait_(bound.clone()).build()
                    .build()
            }))
        .build()
}
