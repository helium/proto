use prost_build;
use std::io::Result;

fn main() -> Result<()> {
    let txn_proto = ["blockchain_txn.proto"];
    prost_build::compile_protos(&txn_proto, &["../src/"]).unwrap();

    Ok(())
}
