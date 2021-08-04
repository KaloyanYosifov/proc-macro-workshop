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
