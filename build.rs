use prost_build;
use std::io::Result;

fn main() -> Result<()> {
    let txn_proto = ["src/blockchain/transactions/txn.proto"];
    prost_build::compile_protos(&txn_proto, &["src/blockchain/transactions"]).unwrap();

    Ok(())
}
