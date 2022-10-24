#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(non_camel_case_types)]

include!(concat!(env!("OUT_DIR"), "/helium.rs"));

pub use blockchain_txn::Txn;
pub use prost::{DecodeError, EncodeError, Message};

#[cfg(feature = "services")]
pub mod services {
    use crate::{
        BlockchainTokenTypeV1, BlockchainTxn, DataRate, GatewayStakingMode, Region, RoutingAddress,
    };

    pub mod router {
        pub use crate::router_client::RouterClient;
        pub use crate::state_channel_client::StateChannelClient;

        include!(concat!(env!("OUT_DIR"), "/helium.packet_router.rs"));
        pub use packet_client::PacketClient as PacketRouterClient;
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

impl std::fmt::Display for DataRate {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            DataRate::Sf12bw125 => f.write_str("SF12BW125"),
            DataRate::Sf11bw125 => f.write_str("SF11BW125"),
            DataRate::Sf10bw125 => f.write_str("SF10BW125"),
            DataRate::Sf9bw125 => f.write_str("SF9BW125"),
            DataRate::Sf8bw125 => f.write_str("SF8BW125"),
            DataRate::Sf7bw125 => f.write_str("SF7BW125"),
            DataRate::Sf12bw250 => f.write_str("SF12BW250"),
            DataRate::Sf11bw250 => f.write_str("SF11BW250"),
            DataRate::Sf10bw250 => f.write_str("SF10BW250"),
            DataRate::Sf9bw250 => f.write_str("SF9BW250"),
            DataRate::Sf8bw250 => f.write_str("SF8BW250"),
            DataRate::Sf7bw250 => f.write_str("SF7BW250"),
            DataRate::Sf12bw500 => f.write_str("SF12BW500"),
            DataRate::Sf11bw500 => f.write_str("SF11BW500"),
            DataRate::Sf10bw500 => f.write_str("SF10BW500"),
            DataRate::Sf9bw500 => f.write_str("SF9BW500"),
            DataRate::Sf8bw500 => f.write_str("SF8BW500"),
            DataRate::Sf7bw500 => f.write_str("SF7BW500"),
            DataRate::Lrfhss1bw137 => f.write_str("LRFHSS1BW137"),
            DataRate::Lrfhss2bw137 => f.write_str("LRFHSS2BW137"),
            DataRate::Lrfhss1bw336 => f.write_str("LRFHSS1BW336"),
            DataRate::Lrfhss2bw336 => f.write_str("LRFHSS2BW336"),
            DataRate::Lrfhss1bw1523 => f.write_str("LRFHSS1BW1523"),
            DataRate::Lrfhss2bw1523 => f.write_str("LRFHSS2BW1523"),
            DataRate::Fsk50 => f.write_str("FSK50"),
        }
    }
}

impl std::str::FromStr for Region {
    type Err = prost::DecodeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_uppercase().as_str() {
            "US915" => Ok(Region::Us915),
            "EU868" => Ok(Region::Eu868),
            "EU433" => Ok(Region::Eu433),
            "CN470" => Ok(Region::Cn470),
            "CN779" => Ok(Region::Cn779),
            "AU915" => Ok(Region::Au915),
            "AS923_1" => Ok(Region::As9231),
            "AS923_1B" => Ok(Region::As9231b),
            "AS923_2" => Ok(Region::As9232),
            "AS923_3" => Ok(Region::As9233),
            "AS923_4" => Ok(Region::As9234),
            "KR920" => Ok(Region::Kr920),
            "IN865" => Ok(Region::In865),
            "CD900_1A" => Ok(Region::Cd9001a),
            unsupported => Err(prost::DecodeError::new(format!(
                "unknown region: {unsupported}"
            ))),
        }
    }
}

impl std::fmt::Display for Region {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Region::Us915 => f.write_str("US915"),
            Region::Eu868 => f.write_str("EU868"),
            Region::Eu433 => f.write_str("EU433"),
            Region::Cn470 => f.write_str("CN470"),
            Region::Cn779 => f.write_str("CN779"),
            Region::Au915 => f.write_str("AU915"),
            Region::As9231 => f.write_str("AS923_1"),
            Region::As9231b => f.write_str("AS923_1B"),
            Region::As9232 => f.write_str("AS923_2"),
            Region::As9233 => f.write_str("AS923_3"),
            Region::As9234 => f.write_str("AS923_4"),
            Region::Kr920 => f.write_str("KR920"),
            Region::In865 => f.write_str("IN865"),
            Region::Cd9001a => f.write_str("CD900_1A"),
        }
    }
}
