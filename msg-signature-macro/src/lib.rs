use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(MsgHasSignature)]
pub fn msg_signature_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let has_signature = match &input.data {
        syn::Data::Struct(data_struct) => match &data_struct.fields {
            syn::Fields::Named(fields_named) => fields_named
                .named
                .iter()
                .any(|field| field.ident.as_ref().unwrap() == "signature"),
            _ => false,
        },
        _ => false,
    };

    if has_signature {
        let expanded = quote! {
            impl msg_signature::MsgHasSignature for #name {
                fn signature(&self) -> &[u8] {
                    &self.signature
                }

                fn without_signature(&self) -> Self {
                    let mut clone = self.clone();
                    clone.signature = vec![];
                    clone
                }

                fn set_signature(&mut self, signature: &[u8]) {
                    self.signature = signature.to_vec();
                }
            }
        };

        return TokenStream::from(expanded);
    }

    TokenStream::new()
}
