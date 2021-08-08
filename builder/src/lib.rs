use proc_macro::TokenStream;

use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input, Type};

fn get_type_indent_without_option_wrapped(ty: &Type) -> Option<&Type> {
    if let Type::Path(path) = ty {
        let segments = &path.path.segments;
        let segment = segments.last().unwrap();
        let ident = get_first_level_indent_of_type(ty).unwrap();

        if ident == "Option" {
            if let syn::PathArguments::AngleBracketed(field) = &segment.arguments {
                if let syn::GenericArgument::Type(nested_type) = field.args.last().unwrap() {
                    return get_type_indent_without_option_wrapped(nested_type);
                }
            }
        }

        Some(ty)
    } else {
        None
    }
}

fn get_first_level_indent_of_type(ty: &Type) -> Option<&syn::Ident> {
    if let Type::Path(path) = ty {
        let segments = &path.path.segments;
        let segment = segments.last().unwrap();

        Some(&segment.ident)
    } else {
        None
    }
}

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let parsed_ast = parse_macro_input!(input as DeriveInput);
    let name = &parsed_ast.ident;
    let builder_struct_name = format!("{}Builder", name);
    let builder_struct_name = syn::Ident::new(&builder_struct_name, name.span());
    let data = parsed_ast.data;

    let fields = if let Data::Struct(ref data_struct) = data {
        if let Fields::Named(ref named_fields) = data_struct.fields {
            &named_fields.named
        } else {
            todo!();
        }
    } else {
        todo!();
    };

    let builder_fields = fields.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let field_type = &field.ty;
        let ident = get_first_level_indent_of_type(field_type).unwrap();

        if ident == "Option" {
            quote! {
                #field_name: #field_type
            }
        } else {
            quote! {
                #field_name: std::option::Option<#field_type>
            }
        }
    });
    let builder_fields_empty = fields.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();

        quote! {
            #field_name: None
        }
    });
    let builder_fields_methods = fields.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let field_type = get_type_indent_without_option_wrapped(&field.ty).unwrap();

        quote! {
            pub fn #field_name(&mut self, #field_name: #field_type) -> &mut Self {
                self.#field_name = Some(#field_name);

                self
            }
        }
    });
    let constructor_arguments = fields.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let field_ident = get_first_level_indent_of_type(&field.ty).unwrap();

        if field_ident == "Option" {
            quote! {
                #field_name: self.#field_name.take()
            }
        } else {
            quote! {
                #field_name: self.#field_name.take().ok_or("#name is required")?
            }
        }
    });

    let tokens = quote!(
        use std::error::Error;

        pub struct #builder_struct_name {
            #(#builder_fields,)*
        }

        impl #builder_struct_name {
            #(#builder_fields_methods)*

            pub fn build(&mut self) -> Result<#name, Box<dyn Error>> {
                Ok(#name {
                    #(#constructor_arguments,)*
                })
            }
        }

        impl #name {
           pub fn builder() -> #builder_struct_name {
                #builder_struct_name {
                    #(#builder_fields_empty,)*
                }
            }
        }
    );

    tokens.into()
}
