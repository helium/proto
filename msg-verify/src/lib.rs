use helium_crypto::PublicKey;
pub use msg_verify_macro::MsgVerify;

#[derive(thiserror::Error, Debug)]
pub enum MsgVerifyError {
    #[error("prost encode error: {0}")]
    Prost(#[from] prost::EncodeError),

    #[error("crypto error: {0}")]
    Crypto(#[from] helium_crypto::Error),
}

pub trait MsgVerify {
    fn verify(&self, verifier: &PublicKey) -> Result<(), MsgVerifyError>;
}
