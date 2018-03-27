#![allow(dead_code)] // TODO: remove

// This is inspired from `synstructure`, but `synstructure` is not adapted in severals ways
// including:
//     * `&mut` everywhere
//     * not generic, we use our own `ast`, `synstructure` only knows about `syn`
//     * missing information (what arm are we in?, what attributes? etc.)

use ast;
use attr;
use quote;

use quote::ToTokens;
use syn;

/// The type of binding to use when generating a pattern.
#[derive(Debug, Copy, Clone)]
pub enum BindingStyle {
    /// `x`
    Move,
    /// `mut x`
    MoveMut,
    /// `ref x`
    Ref,
    /// `ref mut x`
    RefMut,
}

impl quote::ToTokens for BindingStyle {
    fn to_tokens(&self, tokens: &mut quote::Tokens) {
        match *self {
            BindingStyle::Move => (),
            BindingStyle::MoveMut => tokens.append("mut"),
            BindingStyle::Ref => tokens.append("ref"),
            BindingStyle::RefMut => {
                tokens.append("ref");
                tokens.append("mut");
            }
        }
    }
}

#[derive(Debug)]
pub struct BindingInfo<'a> {
    pub ident: syn::Ident,
    pub field: &'a ast::Field<'a>,
}

pub struct Matcher {
    binding_name: String,
    binding_style: BindingStyle,
}

impl Matcher {
    pub fn new(style: BindingStyle) -> Self {
        Matcher {
            binding_name: "__arg".into(),
            binding_style: style,
        }
    }

    pub fn with_name(self, name: String) -> Self {
        Matcher { binding_name: name, ..self }
    }

    pub fn build_arms<F>(self, input: &ast::Input, f: F) -> quote::Tokens
    where F: Fn(syn::Path, &syn::Ident, ast::Style, &attr::Input, Vec<BindingInfo>) -> quote::Tokens
    {
        let ident = &input.ident;
        // Generate patterns for matching against all of the variants
        let variants = match input.body {
            ast::Body::Enum(ref variants) => {
                variants.iter()
                    .map(|variant| {
                        let variant_ident = &variant.ident;
                        let variant_path = syn::aster::path().ids(&[ident, variant_ident]).build();

                        let pat = self.build_match_pattern(
                            &variant_path,
                            variant.style,
                            &variant.fields
                        );

                        (variant_path, variant_ident, variant.style, &variant.attrs, pat)
                    })
                    .collect()
            }
            ast::Body::Struct(style, ref vd) => {
                let path = syn::aster::path().id(ident).build();
                vec![(path, ident, style, &input.attrs, self.build_match_pattern(ident, style, vd))]
            }
        };

        // Now that we have the patterns, generate the actual branches of the match
        // expression
        let mut t = quote::Tokens::new();
        for (path, name, style, attrs, (pat, bindings)) in variants {
            let body = f(path, name, style, attrs, bindings);
            quote!(#pat => { #body }).to_tokens(&mut t);
        }

        t
    }

    pub fn build_match_pattern<'a, N>(
        &self,
        name: &N,
        style: ast::Style,
        fields: &'a [ast::Field<'a>]
    )
    -> (quote::Tokens, Vec<BindingInfo<'a>>)
    where N: quote::ToTokens,
    {
        let mut t = quote::Tokens::new();
        let mut matches = Vec::new();

        let binding = self.binding_style;
        name.to_tokens(&mut t);
        match style {
            ast::Style::Unit => {}
            ast::Style::Tuple => {
                t.append("(");
                for (i, field) in fields.iter().enumerate() {
                    let ident: syn::Ident = format!("{}_{}", self.binding_name, i).into();
                    quote!(#binding #ident ,).to_tokens(&mut t);
                    matches.push(BindingInfo {
                        ident: ident,
                        field: field,
                    });
                }
                t.append(")");
            }
            ast::Style::Struct => {
                t.append("{");
                for (i, field) in fields.iter().enumerate() {
                    let ident: syn::Ident = format!("{}_{}", self.binding_name, i).into();
                    {
                        let field_name = field.ident.as_ref().unwrap();
                        quote!(#field_name : #binding #ident ,).to_tokens(&mut t);
                    }
                    matches.push(BindingInfo {
                        ident: ident,
                        field: field,
                    });
                }
                t.append("}");
            }
        }

        (t, matches)
    }
}
