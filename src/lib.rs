pub mod blockchain {
    include!(concat!(env!("OUT_DIR"), "/helium.txns.rs"));
    pub use blockchain_txn::Txn;
}

pub mod packet {
    include!(concat!(env!("OUT_DIR"), "/helium.packet.rs"));
}

pub use prost::Message;
