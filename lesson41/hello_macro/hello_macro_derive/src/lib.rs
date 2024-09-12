extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // construct a representation of Rust code
    // as an abstract syntax tree that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let expanded = quote! {
        impl HelloMacro for #name {
            fn hello_macro(){
                println!(
                    "Hello, Macro! My name is {}!", stringify!(#name)
                );
            }
        }
    };
    TokenStream::from(expanded)
}
