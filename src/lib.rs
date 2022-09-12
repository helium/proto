#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(non_camel_case_types)]

include!(concat!(env!("OUT_DIR"), "/helium.rs"));

pub use blockchain_txn::Txn;
pub use prost::{DecodeError, EncodeError, Message};

#[cfg(feature = "services")]
pub mod services {
    use crate::{BlockchainTokenTypeV1, BlockchainTxn, RoutingAddress};

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

    pub mod poc_mobile {
        include!(concat!(env!("OUT_DIR"), "/helium.poc_mobile.rs"));
        pub use poc_mobile_client::PocMobileClient as Client;
        pub use poc_mobile_server::{PocMobile, PocMobileServer as Server};

        impl Share {
            pub fn is_valid(&self) -> bool {
                self.validity == 0
            }
        }

        impl SpeedShare {
            pub fn is_valid(&self) -> bool {
                self.validity == 0
            }
        }
    }

    pub mod follower {
        include!(concat!(env!("OUT_DIR"), "/helium.follower.rs"));
        pub use follower_client::FollowerClient as Client;
        pub use follower_server::FollowerServer as Server;
    }

    pub mod transaction {
        include!(concat!(env!("OUT_DIR"), "/helium.transaction.rs"));
        pub use transaction_client::TransactionClient as Client;
        pub use transaction_server::TransactionServer as Server;
    }

    pub use tonic::transport::*;
}
