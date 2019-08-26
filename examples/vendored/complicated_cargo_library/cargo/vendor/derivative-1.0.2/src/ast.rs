use attr;
use syn;

#[derive(Debug)]
pub struct Input<'a> {
    pub attrs: attr::Input,
    pub body: Body<'a>,
    pub generics: &'a syn::Generics,
    pub ident: syn::Ident,
}

#[derive(Debug)]
pub enum Body<'a> {
    Enum(Vec<Variant<'a>>),
    Struct(Style, Vec<Field<'a>>),
}

#[derive(Debug)]
pub struct Variant<'a> {
    pub attrs: attr::Input,
    pub fields: Vec<Field<'a>>,
    pub ident: syn::Ident,
    pub style: Style,
}

#[derive(Debug)]
pub struct Field<'a> {
    pub attrs: attr::Field,
    pub ident: Option<syn::Ident>,
    pub ty: &'a syn::Type,
}

#[derive(Clone, Copy, Debug)]
pub enum Style {
    Struct,
    Tuple,
    Unit,
}

impl<'a> Input<'a> {
    pub fn from_ast(item: &'a syn::DeriveInput) -> Result<Input<'a>, String> {
        let attrs = try!(attr::Input::from_ast(&item.attrs));

        let body = match item.data {
            syn::Data::Enum(syn::DataEnum { ref variants, .. }) => {
                Body::Enum(try!(enum_from_ast(variants)))
            }
            syn::Data::Struct(syn::DataStruct { ref fields, .. }) => {
                let (style, fields) = try!(struct_from_ast(fields));
                Body::Struct(style, fields)
            }
            _ => panic!("Unsupported data type"),
        };

        Ok(Input {
            attrs: attrs,
            body: body,
            generics: &item.generics,
            ident: item.ident.clone(),
        })
    }
}

impl<'a> Body<'a> {
    pub fn all_fields(&self) -> Vec<&Field> {
        match *self {
            Body::Enum(ref variants) => variants
                .iter()
                .flat_map(|variant| variant.fields.iter())
                .collect(),
            Body::Struct(_, ref fields) => fields.iter().collect(),
        }
    }
}

fn enum_from_ast<'a>(
    variants: &'a syn::punctuated::Punctuated<syn::Variant, syn::token::Comma>,
) -> Result<Vec<Variant<'a>>, String> {
    variants
        .iter()
        .map(|variant| {
            let (style, fields) = try!(struct_from_ast(&variant.fields));
            Ok(Variant {
                attrs: try!(attr::Input::from_ast(&variant.attrs)),
                fields: fields,
                ident: variant.ident.clone(),
                style: style,
            })
        })
        .collect()
}

fn struct_from_ast<'a>(fields: &'a syn::Fields) -> Result<(Style, Vec<Field<'a>>), String> {
    match *fields {
        syn::Fields::Named(ref fields) => Ok((Style::Struct, try!(fields_from_ast(&fields.named)))),
        syn::Fields::Unnamed(ref fields) => {
            Ok((Style::Tuple, try!(fields_from_ast(&fields.unnamed))))
        }
        syn::Fields::Unit => Ok((Style::Unit, Vec::new())),
    }
}

fn fields_from_ast<'a>(
    fields: &'a syn::punctuated::Punctuated<syn::Field, syn::token::Comma>,
) -> Result<Vec<Field<'a>>, String> {
    fields
        .iter()
        .map(|field| {
            Ok(Field {
                attrs: try!(attr::Field::from_ast(field)),
                ident: field.ident.clone(),
                ty: &field.ty,
            })
        })
        .collect()
}
