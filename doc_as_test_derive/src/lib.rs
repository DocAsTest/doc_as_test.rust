use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn doc_as_test(attr: TokenStream, stream: TokenStream) -> TokenStream {
    let input = parse_macro_input!(stream as ItemFn);

    let ItemFn {
        attrs,
        vis,
        sig,
        block,
    } = input;

    let test_name = sig.ident.to_string();
    let title = attr.to_string();
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
