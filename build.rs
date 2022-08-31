use std::io::Result;
#[cfg(feature = "services")]
fn main() -> Result<()> {
    #[cfg(target_family = "windows")]
    std::env::set_var("PROTOC", protoc_bin_vendored::protoc_bin_path().unwrap());
    #[cfg(not(target_family = "windows"))]
    std::env::set_var("PROTOC", protobuf_src::protoc());

    tonic_build::configure()
        .build_server(false)
        .type_attribute(".", "#[derive(serde_derive::Serialize)]")
        .compile(
            &[
                "src/blockchain_txn.proto",
                "src/service/router.proto",
                "src/service/state_channel.proto",
                "src/service/gateway.proto",
                "src/service/follower.proto",
                "src/service/transaction.proto",
                "src/service/verifier.proto",
            ],
            &["src/"],
        )?;

    tonic_build::configure()
        .build_server(true)
        .type_attribute(".", "#[derive(serde_derive::Serialize)]")
        .compile(
            &["src/service/local.proto", "src/service/poc_mobile.proto"],
            &["src"],
        )?;
    Ok(())
}

#[cfg(not(feature = "services"))]
fn main() -> Result<()> {
    #[cfg(target_family = "windows")]
    std::env::set_var("PROTOC", protoc_bin_vendored::protoc_bin_path().unwrap());
    #[cfg(not(target_family = "windows"))]
    std::env::set_var("PROTOC", protobuf_src::protoc());
    prost_build::Config::new()
        .type_attribute(".", "#[derive(serde_derive::Serialize)]")
        .compile_protos(&["src/blockchain_txn.proto"], &["src/"])?;
    Ok(())
}
