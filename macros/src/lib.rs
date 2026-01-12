use quote::{quote, quote_spanned};
use syn::{ItemStruct, Type, parse};

#[proc_macro_derive(Method, attributes(method))]
pub fn method(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    fn inner(input: proc_macro::TokenStream) -> proc_macro2::TokenStream {
        use proc_macro2::{Delimiter, Literal, TokenTree};

        let input = match parse::<ItemStruct>(input) {
            Ok(v) => v,
            Err(err) => return err.to_compile_error(),
        };

        let ident = input.ident;
        let mut name = None::<Literal>;
        let mut response = None::<Type>;

        for attr in input.attrs {
            if attr.meta.path().get_ident().is_some_and(|v| v == "method") {
                let mut tokens = match attr.meta.require_list() {
                    Ok(v) => v.tokens.clone(),
                    Err(err) => return err.to_compile_error(),
                }
                .into_iter();

                while let Some(token) = tokens.next() {
                    let TokenTree::Ident(ident) = token else {
                        return quote_spanned!(ident.span() => compile_error!("invalid attribute"));
                    };

                    if ident == "name" {
                        let Some(token) = tokens.next() else {
                            return quote_spanned!(ident.span() => compile_error!("attribute `name` expects a value"));
                        };
                        let TokenTree::Punct(p) = token else {
                            return quote_spanned!(token.span() => compile_error!("attribute `name` is a name-value attribute. `=` expected"));
                        };
                        if p.as_char() != '=' {
                            return quote_spanned!(p.span() => compile_error!("attribute `name` is a name-value attribute. `=` expected"));
                        }

                        let Some(token) = tokens.next() else {
                            return quote_spanned!(ident.span() => compile_error!("attribute `name` requires a string literal value"));
                        };
                        let TokenTree::Literal(lit) = token else {
                            return quote_spanned!(token.span() => compile_error!("attribute `name` requires a string literal value. Unexpected value"));
                        };
                        name = Some(lit);
                    } else if ident == "response" {
                        let Some(token) = tokens.next() else {
                            return quote_spanned!(ident.span() => compile_error!("attribute `response` expects a value"));
                        };
                        let TokenTree::Group(g) = token else {
                            return quote_spanned!(token.span() => compile_error!("attribute `response` is a List attribute"));
                        };
                        if g.delimiter() != Delimiter::Parenthesis {
                            return quote_spanned!(g.delim_span() => compile_error!("attribute `response` expects a paren delimiter"));
                        }

                        response = Some(
                            match parse::<Type>(proc_macro::TokenStream::from(g.stream())) {
                                Ok(v) => v,
                                Err(err) => return err.to_compile_error(),
                            },
                        );
                    } else {
                        return quote_spanned!(ident.span() => compile_error!("unknown attribute"));
                    }

                    if let Some(token) = tokens.next()
                        && !matches!(&token, TokenTree::Punct(p) if p.as_char() == ',')
                    {
                        return quote_spanned!(token.span() => compile_error!("unexpected token"));
                    }
                }
            }
        }

        let Some(name) = name else {
            return quote_spanned!(ident.span() => compile_error!("no method name provided"));
        };
        let Some(response) = response else {
            return quote_spanned!(ident.span() => compile_error!("no method response type provided"));
        };

        quote!(impl crate::Method<'_> for #ident {
            const NAME: &'static str = #name;
            type Response = #response;
        })
    }

    proc_macro::TokenStream::from(inner(input))
}
