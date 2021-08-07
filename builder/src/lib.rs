use proc_macro::TokenStream;

use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input, Type};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let parsed_ast = parse_macro_input!(input as DeriveInput);
    let name = &parsed_ast.ident;
    let builder_struct_name = format!("{}Builder", name);
    let builder_struct_name = syn::Ident::new(&builder_struct_name, name.span());
    let data = parsed_ast.data;

    let fields = if let Data::Struct(data_struct) = data {
        if let Fields::Named(named_fields) = data_struct.fields {
            named_fields.named
        } else {
            todo!();
        }
    } else {
        todo!();
    };

    let constructor_arguments = fields.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let field_type = if let Type::Path(path) = &field.ty {
            let segments = &path.path.segments;
            let segment = segments.last().unwrap();

            segment.ident.to_string()
        } else {
            todo!();
        };

        if field_type == "Option" {
            quote! {
                #field_name: self.#field_name.take(),
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
            executable: Option<String>,
            args: Option<Vec<String>>,
            env: Option<Vec<String>>,
            current_dir: Option<String>
        }

        impl #builder_struct_name {
            pub fn executable(&mut self, exe: String) -> &mut Self {
                self.executable = Some(exe);

                self
            }

            pub fn args(&mut self, args: Vec<String>) -> &mut Self {
                self.args = Some(args);

                self
            }

            pub fn env(&mut self, env: Vec<String>) -> &mut Self {
                self.env = Some(env);

                self
            }

            pub fn current_dir(&mut self, dir: String) -> &mut Self {
                self.current_dir = Some(dir);

                self
            }

            pub fn build(&mut self) -> Result<#name, Box<dyn Error>> {
                Ok(#name {
                    #(#constructor_arguments),*
                })
            }
        }

        impl #name {
           pub fn builder() -> #builder_struct_name {
                #builder_struct_name {
                    env: None,
                    args: None,
                    executable: None,
                    current_dir: None
                }
            }
        }
    );

    tokens.into()
}
