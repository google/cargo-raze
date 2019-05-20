#![allow(dead_code)] // TODO: remove

// This is inspired from `synstructure`, but `synstructure` is not adapted in severals ways
// including:
//     * `&mut` everywhere
//     * not generic, we use our own `ast`, `synstructure` only knows about `syn`
//     * missing information (what arm are we in?, what attributes? etc.)

use proc_macro2;
use quote::ToTokens;
use syn;

use ast;
use attr;
use quote;

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
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match *self {
            BindingStyle::Move => (),
            BindingStyle::MoveMut => tokens.extend(quote!(mut)),
            BindingStyle::Ref => tokens.extend(quote!(ref)),
            BindingStyle::RefMut => {
                tokens.extend(quote!(ref mut));
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
        Matcher {
            binding_name: name,
            ..self
        }
    }

    pub fn build_arms<F>(self, input: &ast::Input, f: F) -> proc_macro2::TokenStream
    where
        F: Fn(syn::Path, &syn::Ident, ast::Style, &attr::Input, Vec<BindingInfo>)
            -> proc_macro2::TokenStream,
    {
        let ident = &input.ident;
        // Generate patterns for matching against all of the variants
        let variants = match input.body {
            ast::Body::Enum(ref variants) => variants
                .iter()
                .map(|variant| {
                    let variant_ident = &variant.ident;
                    let variant_path = parse_quote!(#ident::#variant_ident);

                    let pat =
                        self.build_match_pattern(&variant_path, variant.style, &variant.fields);

                    (
                        variant_path,
                        variant_ident,
                        variant.style,
                        &variant.attrs,
                        pat,
                    )
                })
                .collect(),
            ast::Body::Struct(style, ref vd) => {
                let path = parse_quote!(#ident);
                vec![(
                    path,
                    ident,
                    style,
                    &input.attrs,
                    self.build_match_pattern(ident, style, vd),
                )]
            }
        };

        // Now that we have the patterns, generate the actual branches of the match
        // expression
        let mut t = proc_macro2::TokenStream::new();
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
        fields: &'a [ast::Field<'a>],
    ) -> (proc_macro2::TokenStream, Vec<BindingInfo<'a>>)
    where
        N: quote::ToTokens,
    {
        let binding = self.binding_style;
        let (stream, matches) = match style {
            ast::Style::Unit => (proc_macro2::TokenStream::new(), Vec::new()),
            ast::Style::Tuple => {
                let (stream, matches) = fields.iter().enumerate().fold(
                    (proc_macro2::TokenStream::new(), Vec::new()),
                    |(mut stream, mut matches), (i, field)| {
                        let ident: syn::Ident = syn::Ident::new(
                            &format!("{}_{}", self.binding_name, i),
                            proc_macro2::Span::call_site(),
                        );
                        quote!(#binding #ident ,).to_tokens(&mut stream);
                        matches.push(BindingInfo { ident: ident, field: field });

                        (stream, matches)
                    },
                );

                (quote! { ( #stream ) }, matches)
            }
            ast::Style::Struct => {
                let (stream, matches) = fields.iter().enumerate().fold(
                    (proc_macro2::TokenStream::new(), Vec::new()),
                    |(mut stream, mut matches), (i, field)| {
                        let ident: syn::Ident = syn::Ident::new(
                            &format!("{}_{}", self.binding_name, i),
                            proc_macro2::Span::call_site(),
                        );
                        {
                            let field_name = field.ident.as_ref().unwrap();
                            quote!(#field_name : #binding #ident ,).to_tokens(&mut stream);
                        }
                        matches.push(BindingInfo {
                            ident: ident,
                            field: field,
                        });

                        (stream, matches)
                    },
                );

                (quote! { { #stream } }, matches)
            }
        };

        let mut all_tokens = proc_macro2::TokenStream::new();
        name.to_tokens(&mut all_tokens);
        all_tokens.extend(stream);

        (all_tokens, matches)
    }
}
