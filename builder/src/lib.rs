use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};
use quote::quote;

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let parsed_ast = parse_macro_input!(input as DeriveInput);
    let name = &parsed_ast.ident;
    let builder_struct_name = format!("{}Builder", name);
    let builder_struct_name = syn::Ident::new(&builder_struct_name, name.span());
    let tokens = quote!(
        struct #builder_struct_name {}
        impl #name {
            fn builder() -> #builder_struct_name {
                #builder_struct_name {}
            }
        }
    );

    tokens.into()
}
