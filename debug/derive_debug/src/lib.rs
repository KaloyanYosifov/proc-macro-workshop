use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Ident, Data, Fields};

#[proc_macro_derive(CustomDebug)]
pub fn derive(input: TokenStream) -> TokenStream {
    let parsed_ast = parse_macro_input!(input as DeriveInput);
    let struct_structure = &parsed_ast.ident;
    let struct_name = struct_structure.to_string();
    let field_names = if let Data::Struct(ref data_struct) = parsed_ast.data {
        if let Fields::Named(ref fields) = data_struct.fields {
            fields
                .named
                .iter()
                .map(|field| field.ident.as_ref().unwrap().to_string())
                .collect::<Vec<String>>()
        } else {
            todo!();
        }
    } else {
        todo!();
    };
    let debug_fields = field_names
        .iter()
        .map(|name| {
            let ident = Ident::new(&name, proc_macro2::Span::call_site());

            quote! {
                &self.#ident
            }
        });
    let returned_token = quote! {
        impl std::fmt::Debug for #struct_structure {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct(#struct_name)
                    #(.field(#field_names, #debug_fields))*
                    .finish()
            }
        }
    };

    returned_token.into()
}
