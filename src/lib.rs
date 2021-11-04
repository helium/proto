include!(concat!(env!("OUT_DIR"), "/helium.rs"));

#[cfg(feature = "services")]
include!(concat!(env!("OUT_DIR"), "/helium.local.rs"));

pub use blockchain_txn::Txn;
pub use prost::{DecodeError, EncodeError, Message};

#[cfg(feature = "services")]
pub mod services {
    pub mod router {
        pub use crate::router_client::RouterClient;
        pub use crate::state_channel_client::StateChannelClient;
    }
    pub mod gateway {
        pub use crate::gateway_client::GatewayClient as Client;
    }

    pub mod local {
        pub use crate::api_client::ApiClient as Client;
        pub use crate::api_server::ApiServer as Server;
    }

    pub use tonic::transport::*;
}
