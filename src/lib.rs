include!(concat!(env!("OUT_DIR"), "/helium.rs"));

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
        include!(concat!(env!("OUT_DIR"), "/helium.local.rs"));
        pub use api_client::ApiClient as Client;
        pub use api_server::{Api, ApiServer as Server};

        impl From<crate::BlockchainVarV1> for ConfigValue {
            fn from(v: crate::BlockchainVarV1) -> Self {
                Self {
                    name: v.name,
                    r#type: v.r#type,
                    value: v.value,
                }
            }
        }
    }

    pub use tonic::transport::*;
}
