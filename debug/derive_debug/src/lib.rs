use quote::quote;
use proc_macro::TokenStream;
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
    let mut formatter = String::from("{} {{ ");
    let debug_fields: Vec<_> = fields
        .iter()
        .enumerate()
        .map(|(index, field)| {
            let ident: &Ident = field.ident.as_ref().unwrap();
            let stringified_ident = ident.to_string();

            if index == 0 {
                formatter.push_str("{}");
            } else {
                formatter.push_str(", {}");
            }

            quote! {
                format!("{}: \"{}\"", #stringified_ident, &self.#ident)
            }
        })
        .collect();

    formatter.push_str(" }}");

    let returned_token = quote! {
        impl std::fmt::Debug for #struct_structure {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, #formatter, #struct_name, #(#debug_fields,)*)
            }
        }
    };

    returned_token.into()
}
