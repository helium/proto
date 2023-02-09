#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(non_camel_case_types)]

include!(concat!(env!("OUT_DIR"), "/helium.rs"));

pub use blockchain_txn::Txn;
pub use prost::{DecodeError, EncodeError, Message};

#[cfg(feature = "services")]
pub mod services {
    use crate::{
        BlockchainRegionParamsV1, BlockchainTokenTypeV1, BlockchainTxn, DataRate, EntropyReportV1,
        GatewayStakingMode, MapperAttach, Region, RoutingAddress,
    };

    pub mod iot_config {
        include!(concat!(env!("OUT_DIR"), "/helium.iot_config.rs"));
        pub use gateway_client::GatewayClient;
        pub use gateway_server::{Gateway, GatewayServer};
        pub use org_client as config_org_client;
        pub use org_server::{Org, OrgServer};
        pub use route_client as config_route_client;
        pub use route_server::{Route, RouteServer};
        pub use session_key_filter_client as config_session_key_filter_client;
        pub use session_key_filter_server::{SessionKeyFilter, SessionKeyFilterServer};
    }

    pub mod downlink {
        include!(concat!(env!("OUT_DIR"), "/helium.downlink.rs"));
        pub use http_roaming_server::{HttpRoaming, HttpRoamingServer as Server};
    }

    pub mod router {
        pub use crate::router_client::RouterClient;

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
    }

    pub mod packet_verifier {
        include!(concat!(env!("OUT_DIR"), "/helium.packet_verifier.rs"));
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

    pub mod poc_entropy {
        include!(concat!(env!("OUT_DIR"), "/helium.poc_entropy.rs"));
        pub use poc_entropy_client::PocEntropyClient as Client;
        pub use poc_entropy_server::{PocEntropy, PocEntropyServer as Server};
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
            "EU868_A" => Ok(Region::Eu868A),
            "EU868_B" => Ok(Region::Eu868B),
            "EU868_C" => Ok(Region::Eu868C),
            "EU868_D" => Ok(Region::Eu868D),
            "EU868_E" => Ok(Region::Eu868E),
            "EU868_F" => Ok(Region::Eu868F),
            "EU433" => Ok(Region::Eu433),
            "CN470" => Ok(Region::Cn470),
            "CN779" => Ok(Region::Cn779),
            "AU915" => Ok(Region::Au915),
            "AU915_SB1" => Ok(Region::Au915Sb1),
            "AU915_SB2" => Ok(Region::Au915Sb2),
            "AS923_1" => Ok(Region::As9231),
            "AS923_1A" => Ok(Region::As9231a),
            "AS923_1B" => Ok(Region::As9231b),
            "AS923_1C" => Ok(Region::As9231c),
            "AS923_1D" => Ok(Region::As9231d),
            "AS923_1E" => Ok(Region::As9231e),
            "AS923_1F" => Ok(Region::As9231f),
            "AS923_2" => Ok(Region::As9232),
            "AS923_3" => Ok(Region::As9233),
            "AS923_4" => Ok(Region::As9234),
            "KR920" => Ok(Region::Kr920),
            "IN865" => Ok(Region::In865),
            "CD900_1A" => Ok(Region::Cd9001a),
            "RU864" => Ok(Region::Ru864),
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
            Region::Eu868A => f.write_str("EU868_A"),
            Region::Eu868B => f.write_str("EU868_B"),
            Region::Eu868C => f.write_str("EU868_C"),
            Region::Eu868D => f.write_str("EU868_D"),
            Region::Eu868E => f.write_str("EU868_E"),
            Region::Eu868F => f.write_str("EU868_F"),
            Region::Eu433 => f.write_str("EU433"),
            Region::Cn470 => f.write_str("CN470"),
            Region::Cn779 => f.write_str("CN779"),
            Region::Au915 => f.write_str("AU915"),
            Region::Au915Sb1 => f.write_str("AU915_SB1"),
            Region::Au915Sb2 => f.write_str("AU915_SB2"),
            Region::As9231 => f.write_str("AS923_1"),
            Region::As9231a => f.write_str("AS923_1A"),
            Region::As9231b => f.write_str("AS923_1B"),
            Region::As9231c => f.write_str("AS923_1C"),
            Region::As9231d => f.write_str("AS923_1D"),
            Region::As9231e => f.write_str("AS923_1E"),
            Region::As9231f => f.write_str("AS923_1F"),
            Region::As9232 => f.write_str("AS923_2"),
            Region::As9233 => f.write_str("AS923_3"),
            Region::As9234 => f.write_str("AS923_4"),
            Region::Kr920 => f.write_str("KR920"),
            Region::In865 => f.write_str("IN865"),
            Region::Cd9001a => f.write_str("CD900_1A"),
            Region::Ru864 => f.write_str("RU864"),
        }
    }
}

impl std::str::FromStr for RegionSpreading {
    type Err = prost::DecodeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_uppercase().as_str() {
            "SF_INVALID" => Ok(RegionSpreading::SfInvalid),
            "SF7" => Ok(RegionSpreading::Sf7),
            "SF8" => Ok(RegionSpreading::Sf8),
            "SF9" => Ok(RegionSpreading::Sf9),
            "SF10" => Ok(RegionSpreading::Sf10),
            "SF11" => Ok(RegionSpreading::Sf11),
            "SF12" => Ok(RegionSpreading::Sf12),
            _ => Ok(RegionSpreading::SfInvalid),
        }
    }
}

impl std::fmt::Display for RegionSpreading {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            RegionSpreading::SfInvalid => f.write_str("SF_INVALID"),
            RegionSpreading::Sf7 => f.write_str("SF7"),
            RegionSpreading::Sf8 => f.write_str("SF8"),
            RegionSpreading::Sf9 => f.write_str("SF9"),
            RegionSpreading::Sf10 => f.write_str("SF10"),
            RegionSpreading::Sf11 => f.write_str("SF11"),
            RegionSpreading::Sf12 => f.write_str("SF12"),
        }
    }
}

macro_rules! serde_enum {
    ($type:ident, $name:ident) => {
        mod $name {
            use super::$type;
            use std::str::FromStr;

            pub fn serialize<S>(v: &i32, serializer: S) -> std::result::Result<S::Ok, S::Error>
            where
                S: serde::ser::Serializer,
            {
                let v = $type::from_i32(*v)
                    .ok_or_else(|| serde::ser::Error::custom(format!("invalid enum value: {v}")))?;
                serializer.serialize_str(&v.to_string())
            }

            pub fn deserialize<'de, D>(deserializer: D) -> std::result::Result<i32, D::Error>
            where
                D: serde::de::Deserializer<'de>,
            {
                struct _Visitor;

                impl<'de> serde::de::Visitor<'de> for _Visitor {
                    type Value = i32;
                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str("string value")
                    }
                    fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        $type::from_str(value).map(|v| v as i32).map_err(|_| {
                            serde::de::Error::custom(format!("invalid string value \"{value}\""))
                        })
                    }
                }

                deserializer.deserialize_str(_Visitor)
            }
        }
    };
}

serde_enum!(RegionSpreading, serde_region_spreading);
