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
    "src/service/packet_router.proto",
    "src/service/iot_config.proto",
    "src/service/downlink.proto",
];

const MESSAGES: &[&str] = &[
    "src/blockchain_txn.proto",
    "src/entropy.proto",
    "src/data_rate.proto",
    "src/region.proto",
    "src/mapper.proto",
    "src/reward_manifest.proto",
    "src/blockchain_region_param_v1.proto",
];

#[cfg(feature = "services")]
fn main() -> Result<()> {
    std::env::set_var("PROTOC", protoc_bin_vendored::protoc_bin_path().unwrap());

    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .type_attribute(
            ".",
            "#[derive(serde_derive::Serialize, serde_derive::Deserialize)]",
        )
        .type_attribute(".helium.config", "#[derive(serde_derive::Deserialize)]")
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
    std::env::set_var("PROTOC", protoc_bin_vendored::protoc_bin_path().unwrap());
    prost_build::Config::new()
        .type_attribute(
            ".",
            "#[derive(serde_derive::Serialize, serde_derive::Deserialize)]",
        )
        .compile_protos(MESSAGES, &["src"])?;
    Ok(())
}
