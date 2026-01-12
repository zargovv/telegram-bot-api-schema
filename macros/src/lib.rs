use quote::{quote, quote_spanned};
use syn::{Attribute, ItemStruct, Type, parse, spanned::Spanned};

struct MethodAttributes {
    name: proc_macro2::Literal,
    response: Type,
}

impl MethodAttributes {
    pub fn collect(
        base_span: proc_macro2::Span,
        attrs: &[Attribute],
    ) -> Result<Self, proc_macro2::TokenStream> {
        use proc_macro2::{Delimiter, Literal, TokenTree};

        let mut name = None::<Literal>;
        let mut response = None::<Type>;

        for attr in attrs {
            if attr.meta.path().get_ident().is_some_and(|v| v == "method") {
                let mut tokens = match attr.meta.require_list() {
                    Ok(v) => v.tokens.clone(),
                    Err(err) => return Err(err.to_compile_error()),
                }
                .into_iter();

                while let Some(token) = tokens.next() {
                    let TokenTree::Ident(ident) = token else {
                        return Err(
                            quote_spanned!(token.span() => compile_error!("invalid attribute")),
                        );
                    };

                    if ident == "name" {
                        let Some(token) = tokens.next() else {
                            return Err(
                                quote_spanned!(ident.span() => compile_error!("attribute `name` expects a value")),
                            );
                        };
                        let TokenTree::Punct(p) = token else {
                            return Err(
                                quote_spanned!(token.span() => compile_error!("attribute `name` is a name-value attribute. `=` expected")),
                            );
                        };
                        if p.as_char() != '=' {
                            return Err(
                                quote_spanned!(p.span() => compile_error!("attribute `name` is a name-value attribute. `=` expected")),
                            );
                        }

                        let Some(token) = tokens.next() else {
                            return Err(
                                quote_spanned!(ident.span() => compile_error!("attribute `name` requires a string literal value")),
                            );
                        };
                        let TokenTree::Literal(lit) = token else {
                            return Err(
                                quote_spanned!(token.span() => compile_error!("attribute `name` requires a string literal value. Unexpected value")),
                            );
                        };
                        name = Some(lit);
                    } else if ident == "response" {
                        let Some(token) = tokens.next() else {
                            return Err(
                                quote_spanned!(ident.span() => compile_error!("attribute `response` expects a value")),
                            );
                        };
                        let TokenTree::Group(g) = token else {
                            return Err(
                                quote_spanned!(token.span() => compile_error!("attribute `response` is a List attribute")),
                            );
                        };
                        if g.delimiter() != Delimiter::Parenthesis {
                            return Err(
                                quote_spanned!(g.delim_span() => compile_error!("attribute `response` expects a paren delimiter")),
                            );
                        }

                        response = Some(
                            match parse::<Type>(proc_macro::TokenStream::from(g.stream())) {
                                Ok(v) => v,
                                Err(err) => return Err(err.to_compile_error()),
                            },
                        );
                    } else {
                        return Err(
                            quote_spanned!(ident.span() => compile_error!("unknown attribute")),
                        );
                    }

                    if let Some(token) = tokens.next()
                        && !matches!(&token, TokenTree::Punct(p) if p.as_char() == ',')
                    {
                        return Err(
                            quote_spanned!(token.span() => compile_error!("unexpected token")),
                        );
                    }
                }
            }
        }

        let Some(name) = name else {
            return Err(quote_spanned!(base_span => compile_error!("no method name provided")));
        };
        let Some(response) = response else {
            return Err(
                quote_spanned!(base_span => compile_error!("no method response type provided")),
            );
        };

        Ok(Self { name, response })
    }
}

#[proc_macro_derive(Method, attributes(method))]
pub fn method(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    fn inner(input: proc_macro::TokenStream) -> proc_macro2::TokenStream {
        let input = match parse::<ItemStruct>(input) {
            Ok(v) => v,
            Err(err) => return err.to_compile_error(),
        };

        let ident = input.ident;
        let field_count = input.fields.len();
        let MethodAttributes { name, response } =
            match MethodAttributes::collect(ident.span(), &input.attrs) {
                Ok(v) => v,
                Err(err) => return err,
            };

        let mut ser_fields = proc_macro2::TokenStream::new();
        for f in &input.fields {
            let Some(ident) = &f.ident else {
                return quote_spanned!(f.span() => "field name expected");
            };
            let name = ident.to_string();
            ser_fields.extend(quote!(ser_struct.serialize_field(#name, &self.#ident)?;));
        }

        quote! {
            impl serde::Serialize for #ident {
                fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
                    use serde::ser::SerializeStruct as _;

                    let mut ser_struct = ser.serialize_struct("SendMessageRequest", #field_count)?;
                    #ser_fields
                    ser_struct.end()
                }
            }

            impl crate::Method<'_> for #ident {
                const NAME: &'static str = #name;
                type Response = #response;
            }
        }
    }

    proc_macro::TokenStream::from(inner(input))
}
