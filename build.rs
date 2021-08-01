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
                "src/service/gateway.proto",
            ],
            &["src/"],
        )?;

    Ok(())
}

#[cfg(feature = "json_serde")]
macro_rules! json_serde {
    ($base:tt) => {
        $base
            .type_attribute(
                ".helium",
                "#[derive(::serde::Serialize, ::serde::Deserialize)]",
            )
            // .type_attribute(
            //     "blockchain_txn",
            //     "#[serde(tag = \"type\", rename_all = \"snake_case\")]",
            // )
            .field_attribute("address", "#[serde(with = \"crate::base58\")]")
            .field_attribute(
                "client_pubkeybin",
                "#[serde(default, with = \"crate::base58\")]",
            )
            .field_attribute("payee", "#[serde(with = \"crate::base58\")]")
            .field_attribute("account", "#[serde(with = \"crate::base58\")]")
            .field_attribute("payer", "#[serde(with = \"crate::base58\")]")
            .field_attribute("owner", "#[serde(with = \"crate::base58\")]")
            .field_attribute("closer", "#[serde(with = \"crate::base58\")]")
            .field_attribute("opener", "#[serde(with = \"crate::base58\")]")
            .field_attribute("public_key", "#[serde(with = \"crate::base58\")]")
            .field_attribute("gateway_owner", "#[serde(with = \"crate::base58\")]")
            .field_attribute("gateway", "#[serde(with = \"crate::base58\")]")
            .field_attribute("challenger_owner", "#[serde(with = \"crate::base58\")]")
            .field_attribute("challengee", "#[serde(with = \"crate::base58\")]")
            .field_attribute("challenger", "#[serde(with = \"crate::base58\")]")
            .field_attribute("buyer", "#[serde(with = \"crate::base58\")]")
            .field_attribute("seller", "#[serde(with = \"crate::base58\")]")
            .field_attribute("multi_keys", "#[serde(with = \"crate::multikeys\")]")
            .field_attribute("id", "#[serde(with = \"crate::base64_url\")]")
            .field_attribute("block_hash", "#[serde(with = \"crate::base64_url\")]")
            .field_attribute("memo", "#[serde(with = \"crate::u64_base64\")]")
            .field_attribute("secret", "#[serde(with = \"crate::base64_url\")]")
            .field_attribute("secret_hash", "#[serde(with = \"crate::base64_url\")]")
            .field_attribute("onion_key_hash", "#[serde(with = \"crate::base64_url\")]")
            .field_attribute("packet_hash", "#[serde(with = \"crate::base64_url\")]")
            .field_attribute("preimage", "#[serde(with = \"crate::base64\")]")
            .field_attribute("hashlock", "#[serde(with = \"crate::base64_url\")]")
            .field_attribute("root_hash", "#[serde(with = \"crate::base64_url\")]")
            .field_attribute("unsets", "#[serde(with = \"crate::str_list\")]")
            .field_attribute("cancels", "#[serde(with = \"crate::str_list\")]")
            .field_attribute(
                "key_proof",
                "#[serde(default, with = \"crate::base64_url\")]",
            )
            .field_attribute(
                "master_key",
                "#[serde(default, with = \"crate::base64_url\")]",
            )
            .field_attribute(
                "request_block_hash",
                "#[serde(with = \"crate::base64_url\")]",
            )
            .field_attribute(
                "multi_proofs",
                "#[serde(with = \"crate::base64_url_list\")]",
            )
            .field_attribute(
                "multi_key_proofs",
                "#[serde(with = \"crate::base64_url_list\")]",
            )
            .field_attribute("fee", "#[serde(default)]")
            .field_attribute(
                "signature",
                "#[serde(default, with = \"crate::base64_url\")]",
            )
            .field_attribute(
                "gateway_signature",
                "#[serde(default, with = \"crate::base64_url\")]",
            )
            .field_attribute(
                "owner_signature",
                "#[serde(default, with = \"crate::base64_url\")]",
            )
            .field_attribute(
                "payer_signature",
                "#[serde(default, with = \"crate::base64_url\")]",
            )
            .field_attribute(
                "buyer_signature",
                "#[serde(default, with = \"crate::base64_url\")]",
            )
            .field_attribute(
                "seller_signature",
                "#[serde(default, with = \"crate::base64_url\")]",
            )
            .field_attribute("filter", "#[serde(default, with = \"crate::base64_url\")]")
            .field_attribute(
                "gateway_owner_signature",
                "#[serde(default, with = \"crate::base64_url\")]",
            )
            .field_attribute(
                "oui_owner_signature",
                "#[serde(default, with = \"crate::base64_url\")]",
            )
            .field_attribute(
                "blockchain_poc_receipt_v1.origin",
                "#[serde(with = \"crate::origin\")]",
            )
            .field_attribute(
                "blockchain_txn_reward_v1.type",
                "#[serde(with = \"crate::reward_type\")]",
            )
            .field_attribute("blockchain_var_v1.type", "#[serde(skip_serializing)]")
            // we ignore data for now. i'm unsure how to parse
            .field_attribute("data", "#[serde(skip_deserializing)]")
            // we ignore oui. it is not returned by the API
            .field_attribute("height", "#[serde(skip_deserializing)]")
            .field_attribute("proof", "#[serde(skip_deserializing)]")
            .field_attribute("oui", "#[serde(skip_deserializing)]")
            .field_attribute("state", "#[serde(skip_deserializing)]")
            .field_attribute("credits", "#[serde(skip_deserializing)]")
            .field_attribute("skewed", "#[serde(skip_deserializing)]")
            .field_attribute("members", "#[serde(skip_deserializing)]")
            .field_attribute("signature", "#[serde(skip_deserializing)]")
            .field_attribute("staking_fee", "#[serde(skip_deserializing)]")
            .field_attribute("datarate", "#[serde(skip_deserializing)]")
            .field_attribute("addr_hash", "#[serde(skip_deserializing)]")
            .field_attribute("tx_power", "#[serde(skip_deserializing)]")
    };
}

#[cfg(not(feature = "services"))]
fn main() -> Result<()> {
    let mut config = prost_build::Config::new();
    #[cfg(feature = "json_serde")]
    json_serde!(config);

    config.compile_protos(&["src/blockchain_txn.proto"], &["src/"])?;
    Ok(())
}
