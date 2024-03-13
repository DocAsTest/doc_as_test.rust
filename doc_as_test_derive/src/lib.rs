use std::collections::HashMap;

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::Expr;
use syn::{parse_macro_input, punctuated::Punctuated, ItemFn, MetaNameValue};

fn extract_params(params: Punctuated<MetaNameValue, syn::Token![,]>) -> HashMap<String, String> {
    let mut hashmap = HashMap::new();
    for param in params {
        let key: String = param.path.to_token_stream().to_string();

        let value = match param.value {
            Expr::Lit(expr) => match &expr.lit {
                syn::Lit::Str(ls) => {
                    println!("Str: {:?}", ls.to_token_stream().to_string());
                    println!("Str: {:?}", ls.value());
                    ls.value()
                }
                _ => expr.to_token_stream().to_string(),
            },
            content => content.to_token_stream().to_string(),
        };

        println!("key: {}, value: {}", key, value);
        hashmap.insert(key, value);
    }
    hashmap
}

#[proc_macro_attribute]
pub fn doc_as_test(attr: TokenStream, stream: TokenStream) -> TokenStream {
    // println!(">>> stream: {}", stream);
    // println!(">>> attr: {}", &attr);
    let input = parse_macro_input!(stream as ItemFn);

    let params = extract_params(
        parse_macro_input!(attr with Punctuated::<MetaNameValue, syn::Token![,]>::parse_terminated),
    );

    let ItemFn {
        attrs,
        vis,
        sig,
        block,
    } = input;

    let test_name = sig.ident.to_string();
    let title = params.get("title").unwrap_or(&test_name);

    let before_stmts = quote! {
        let mut doc = DocAsTest::new(#title, &format!("{}::{}", module_path!(), #test_name));
    };

    let after_stmts = quote! {
        doc.approve();
    };
    let stmts = &block.stmts;
    quote! {
        #[test]
        #(#attrs)* #vis #sig {
            #before_stmts
            #(#stmts)*;
            #after_stmts
        }
    }
    .into()
}

#[cfg(test)]
mod tests {

    use super::*;

    use syn::parse::Parser;

    #[test]
    fn test_extract() {
        let code = "title = \"My title\", value = \"abc\"";
        let attr: proc_macro2::TokenStream = code.parse().unwrap();

        let parsed = Punctuated::<syn::MetaNameValue, syn::Token![,]>::parse_terminated
            .parse2(attr)
            .unwrap();
        let params = extract_params(parsed);
        assert_eq!("My title", params.get("title").unwrap());
        assert_eq!("abc", params.get("value").unwrap());
        assert_eq!(2, params.len());
    }
}
