use proc_macro::TokenStream;

use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input, Type, PathSegment};
use proc_macro2::TokenTree;

fn get_type_without_ident_wrapped(ty: &Type, skipped_ident: String) -> Option<&Type> {
    if let Type::Path(path) = ty {
        let segments = &path.path.segments;
        let segment = segments.last().unwrap();
        let ident = get_first_level_indent_of_type(ty).unwrap();

        if ident == &skipped_ident {
            if let syn::PathArguments::AngleBracketed(field) = &segment.arguments {
                if let syn::GenericArgument::Type(nested_type) = field.args.last().unwrap() {
                    return get_type_without_ident_wrapped(nested_type, skipped_ident);
                }
            }
        }

        Some(ty)
    } else {
        None
    }
}

fn get_first_segment_from_path(path: &syn::Path) -> &PathSegment {
    let segments = &path.segments;

    segments.first().unwrap()
}

fn get_first_level_indent_of_type(ty: &Type) -> Option<&syn::Ident> {
    if let Type::Path(path) = ty {
        Some(&get_first_segment_from_path(&path.path).ident)
    } else {
        None
    }
}

fn parse_field_attribute_and_get_identifier(field: &syn::Field) -> Option<TokenTree> {
    if field.attrs.len() <= 0 {
        return None;
    }

    let attribute = field.attrs.get(0).unwrap();
    let segment = get_first_segment_from_path(&attribute.path);

    if segment.ident != "builder" {
        panic!("Only builder attribute supported!");
    }

    if let proc_macro2::TokenTree::Group(group) = attribute.tokens.clone().into_iter().next().unwrap() {
        let mut tokens = group.stream().into_iter();

        if tokens.next().unwrap().to_string() != "each" {
            proc_macro_error::abort!(attribute, "expected `builder(each = \"...\")`");
        }

        assert_eq!(tokens.next().unwrap().to_string(), "=");

        Some(tokens.next().unwrap())
    } else {
        None
    }
}

#[proc_macro_error::proc_macro_error]
#[proc_macro_derive(Builder, attributes(builder))]
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

    let extra_builder_methods = fields
        .iter()
        .filter(|field| field.attrs.len() > 0)
        .map(|field| {
            if let Some(argument) = parse_field_attribute_and_get_identifier(&field) {
                let field_name = field.ident.as_ref().unwrap();
                let field_type = get_type_without_ident_wrapped(&field.ty, "Option".to_string()).unwrap();

                match get_first_level_indent_of_type(field_type) {
                    Some(value) => assert_eq!("Vec", value.to_string()),
                    _ => panic!("Only vectors are allowed with each macro!"),
                };

                let field_type = get_type_without_ident_wrapped(field_type, "Vec".to_string()).unwrap();
                let parsed_argument = syn::Ident::new(
                    &argument.to_string().replace('"', ""),
                    proc_macro2::Span::call_site(),
                );

                quote! {
                    pub fn #parsed_argument(&mut self, value: #field_type) -> &mut Self {
                        self.#field_name
                            .get_or_insert(vec![])
                            .push(value);

                        self
                    }
                }
            } else {
                quote! {}
            }
        });
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
            #field_name: std::option::Option::None
        }
    });
    let builder_fields_methods = fields.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let field_type = get_type_without_ident_wrapped(&field.ty, "Option".to_string()).unwrap();

        if let Some(argument) = parse_field_attribute_and_get_identifier(&field) {
            if field_name == &argument.to_string().replace('"', "") {
                return quote! {};
            }
        }

        quote! {
            pub fn #field_name(&mut self, #field_name: #field_type) -> &mut Self {
                self.#field_name = std::option::Option::Some(#field_name);

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
        } else if field_ident == "Vec" {
            quote! {
                #field_name: self.#field_name.take().or(std::option::Option::Some(vec![])).unwrap()
            }
        } else {
            let error = format!("{} is required", field_name);

            quote! {
                #field_name: self.#field_name.take().ok_or(#error)?
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
            #(#extra_builder_methods)*

            pub fn build(&mut self) -> std::result::Result<#name, std::boxed::Box<dyn Error>> {
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
