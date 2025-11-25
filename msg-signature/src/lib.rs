pub use msg_signature_macro::MsgHasSignature;

pub trait MsgHasSignature {
    fn signature(&self) -> &[u8];
    fn without_signature(&self) -> Self;
    fn set_signature(&mut self, signature: Vec<u8>);
    fn clear_signature(&mut self);
}
