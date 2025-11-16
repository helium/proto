pub use msg_signature_macro::MsgHasSignature;

pub trait MsgHasSignature {
    fn signature(&self) -> &[u8];
    fn without_signature(&self) -> Self;
}
