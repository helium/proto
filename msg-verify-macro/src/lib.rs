use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(MsgVerify)]
pub fn msg_verify_derive(input: TokenStream) -> TokenStream {
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
            impl msg_verify::MsgVerify for #name {
                fn verify(&self, verifier: &helium_crypto::PublicKey) -> Result<(), msg_verify::MsgVerifyError> {
                    let mut buf = vec![];
                    let mut msg = self.clone();
                    msg.signature = vec![];
                    prost::Message::encode(&msg, &mut buf)?;
                    helium_crypto::Verify::verify(verifier, &buf, &self.signature)?;
                    Ok(())
                }
            }
        };

        return TokenStream::from(expanded);
    }

    TokenStream::new()
}