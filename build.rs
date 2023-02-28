use std::io::Result;

#[cfg(feature = "services")]
const SERVICES: &[&str] = &[
    "src/service/router.proto",
    "src/service/state_channel.proto",
    "src/service/local.proto",
    "src/service/gateway.proto",
    "src/service/transaction.proto",
    "src/service/follower.proto",
    "src/service/poc_mobile.proto",
    "src/service/poc_lora.proto",
    "src/service/poc_entropy.proto",
    "src/service/packet_router.proto",
    "src/service/iot_config.proto",
    "src/service/downlink.proto",
    "src/service/packet_verifier.proto",
];

const MESSAGES: &[&str] = &[
    "src/blockchain_txn.proto",
    "src/entropy.proto",
    "src/data_rate.proto",
    "src/region.proto",
    "src/mapper.proto",
    "src/reward_manifest.proto",
    "src/blockchain_region_param_v1.proto",
    "src/price_report.proto",
];

macro_rules! config {
    ($config:expr) => {
        $config
            .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
            .field_attribute(
                ".helium.tagged_spreading.region_spreading",
                "#[serde(with = \"serde_region_spreading\" )]",
            )
    };
}

#[cfg(feature = "services")]
fn main() -> Result<()> {
    config!(tonic_build::configure())
        .build_server(true)
        .build_client(true)
        .compile(
            &MESSAGES
                .iter()
                .chain(SERVICES)
                .map(|str| *str)
                .collect::<Vec<&str>>(),
            &["src"],
        )?;
    Ok(())
}

#[cfg(not(feature = "services"))]
fn main() -> Result<()> {
    config!(prost_build::Config::new()).compile_protos(MESSAGES, &["src"])?;
    Ok(())
}
