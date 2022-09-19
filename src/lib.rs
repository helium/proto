#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(non_camel_case_types)]

include!(concat!(env!("OUT_DIR"), "/helium.rs"));

pub use blockchain_txn::Txn;
pub use prost::{DecodeError, EncodeError, Message};

#[cfg(feature = "services")]
pub mod services {
    use crate::{
        BlockchainTokenTypeV1, BlockchainTxn, DataRate, GatewayStakingMode, RoutingAddress,
    };

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

    pub mod poc_lora {
        include!(concat!(env!("OUT_DIR"), "/helium.poc_lora.rs"));
        pub use poc_lora_client::PocLoraClient as Client;
        pub use poc_lora_server::{PocLora, PocLoraServer as Server};
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

impl std::str::FromStr for DataRate {
    type Err = prost::DecodeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_uppercase().as_str() {
            "SF12BW125" => Ok(DataRate::Sf12bw125),
            "SF11BW125" => Ok(DataRate::Sf11bw125),
            "SF10BW125" => Ok(DataRate::Sf10bw125),
            "SF9BW125" => Ok(DataRate::Sf9bw125),
            "SF8BW125" => Ok(DataRate::Sf8bw125),
            "SF7BW125" => Ok(DataRate::Sf7bw125),
            "SF12BW250" => Ok(DataRate::Sf12bw250),
            "SF11BW250" => Ok(DataRate::Sf11bw250),
            "SF10BW250" => Ok(DataRate::Sf10bw250),
            "SF9BW250" => Ok(DataRate::Sf9bw250),
            "SF8BW250" => Ok(DataRate::Sf8bw250),
            "SF7BW250" => Ok(DataRate::Sf7bw250),
            "SF12BW500" => Ok(DataRate::Sf12bw500),
            "SF11BW500" => Ok(DataRate::Sf11bw500),
            "SF10BW500" => Ok(DataRate::Sf10bw500),
            "SF9BW500" => Ok(DataRate::Sf9bw500),
            "SF8BW500" => Ok(DataRate::Sf8bw500),
            "SF7BW500" => Ok(DataRate::Sf7bw500),
            "LRFHSS1BW137" => Ok(DataRate::Lrfhss1bw137),
            "LRFHSS2BW137" => Ok(DataRate::Lrfhss2bw137),
            "LRFHSS1BW336" => Ok(DataRate::Lrfhss1bw336),
            "LRFHSS2BW336" => Ok(DataRate::Lrfhss2bw336),
            "LRFHSS1BW1523" => Ok(DataRate::Lrfhss1bw1523),
            "LRFHSS2BW1523" => Ok(DataRate::Lrfhss2bw1523),
            "FSK50" => Ok(DataRate::Fsk50),
            unknown => Err(prost::DecodeError::new(format!(
                "unknown datarate: {unknown}"
            ))),
        }
    }
}
