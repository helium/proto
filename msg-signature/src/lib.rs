// use helium_crypto::PublicKey;
pub use msg_verify_macro::MsgHasSignature;

// #[derive(thiserror::Error, Debug)]
// pub enum MsgVerifyError {
//     #[error("prost encode error: {0}")]
//     Prost(#[from] prost::EncodeError),
//
//     #[error("crypto error: {0}")]
//     Crypto(#[from] helium_crypto::Error),
// }
//
// pub trait MsgVerify {
//     fn verify(&self, verifier: &PublicKey) -> Result<(), MsgVerifyError>;
// }

pub trait MsgHasSignature {
    fn signature(&self) -> &[u8];
    fn without_signature(&self) -> Self;
}