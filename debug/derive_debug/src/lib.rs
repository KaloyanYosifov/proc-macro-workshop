use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Ident, Data, Fields};

#[proc_macro_derive(CustomDebug, attributes(debug))]
pub fn derive(input: TokenStream) -> TokenStream {
    let parsed_ast = parse_macro_input!(input as DeriveInput);
    let struct_structure = &parsed_ast.ident;
    let struct_name = struct_structure.to_string();
    let fields = if let Data::Struct(ref data_struct) = parsed_ast.data {
        if let Fields::Named(ref fields) = data_struct.fields {
            &fields.named
        } else {
            todo!();
        }
    } else {
        todo!();
    };
    let field_names = fields
        .iter()
        .map(|field| field.ident.as_ref().unwrap().to_string());
    let debug_fields = fields
        .iter()
        .map(|field| {
            let ident: &Ident = field.ident.as_ref().unwrap();

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
