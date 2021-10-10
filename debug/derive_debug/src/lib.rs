#![feature(entry_insert)]

use quote::quote;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, Ident, Data, Fields, Field};
use std::collections::HashMap;
use syn::punctuated::Punctuated;

type DebugFields = Punctuated<Field, syn::token::Comma>;

fn get_fields_attribute_values(fields: &DebugFields) -> HashMap<String, String> {
    let mut hasher = HashMap::new();

    fields.iter().for_each(|field| {
        field.attrs.iter()
            .for_each(|e| {
                if let syn::Meta::NameValue(name_value) = e.parse_meta().unwrap() {
                    let name = name_value.path.segments[0].ident.to_string();

                    if name != "debug" {
                        todo!();
                    }

                    let value: String = if let syn::Lit::Str(lit) = name_value.lit {
                        lit.value()
                    } else {
                        todo!()
                    };

                    hasher.insert(field.ident.as_ref().unwrap().to_string(), value);
                } else {
                    todo!();
                }
            });
    });

    hasher
}

fn get_fields_to_show_in_debug(fields: &DebugFields, fields_with_attributes: &HashMap<String, String>) -> (String, Vec<proc_macro2::TokenStream>) {
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

            if fields_with_attributes.contains_key(&stringified_ident) {
                let formatting = fields_with_attributes.get(&stringified_ident).unwrap();
                quote! {
                    format!("{}: {}", #stringified_ident, format!(#formatting, self.#ident))
                }
            } else {
                quote! {
                    format!("{}: \"{}\"", #stringified_ident, &self.#ident)
                }
            }
        })
        .collect();

    formatter.push_str(" }}");

    (formatter, debug_fields)
}

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
    let fields_with_attributes = get_fields_attribute_values(fields);
    let (formatter, debug_fields) = get_fields_to_show_in_debug(fields, &fields_with_attributes);

    let returned_token = quote! {
        impl std::fmt::Debug for #struct_structure {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, #formatter, #struct_name, #(#debug_fields,)*)
            }
        }
    };

    returned_token.into()
}
