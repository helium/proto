use std::io::Result;

#[cfg(feature = "services")]
fn main() -> Result<()> {
    tonic_build::configure()
        .build_server(false)
        .type_attribute(".", "#[derive(serde_derive::Serialize)]")
        .compile(
            &[
                "src/blockchain_txn.proto",
                "src/service/router.proto",
                "src/service/validator.proto",
            ],
            &["src/"],
        )?;
    Ok(())
}

#[cfg(not(feature = "services"))]
fn main() -> Result<()> {
    prost_build::Config::new()
        .type_attribute(".", "#[derive(serde_derive::Serialize)]")
        .compile_protos(&["src/blockchain_txn.proto"], &["src/"])?;
    Ok(())
}
