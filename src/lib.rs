#![allow(clippy::derive_partial_eq_without_eq)]
include!(concat!(env!("OUT_DIR"), "/helium.rs"));

pub use blockchain_txn::Txn;
pub use prost::{DecodeError, EncodeError, Message};

#[cfg(feature = "services")]
pub mod services {
    pub use crate::DataRate;
    // Referenced with helium.packet_router.rs as super::Region
    pub use crate::Region;
    pub mod router {
        pub use crate::router_client::RouterClient;
        pub use crate::state_channel_client::StateChannelClient;

        include!(concat!(env!("OUT_DIR"), "/helium.packet_router.rs"));
        pub use gateway_client::GatewayClient as PacketRouterClient;
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

    pub mod poc_mobile {
        include!(concat!(env!("OUT_DIR"), "/helium.poc_mobile.rs"));
        pub use poc_mobile_client::PocMobileClient as Client;
        pub use poc_mobile_server::{PocMobile, PocMobileServer as Server};
    }

    pub use tonic::transport::*;
}
