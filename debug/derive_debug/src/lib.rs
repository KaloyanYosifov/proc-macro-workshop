use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(CustomDebug)]
pub fn derive(input: TokenStream) -> TokenStream {
    let parsed_ast = parse_macro_input!(input as DeriveInput);
    let struct_structure = &parsed_ast.ident;
    let struct_name = struct_structure.to_string();

    //
    // let fields = if let Data::Struct(ref data_struct) = data {} else {
    //     todo!();
    // };

    let returned_token = quote! {
        impl std::fmt::Debug for #struct_structure {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct(#struct_name)
                    .field("name", &self.name)
                    .field("bitmask", &self.bitmask)
                    .finish()
            }
        }
    };

    returned_token.into()
}
