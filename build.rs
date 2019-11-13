use prost_build;
use std::{
    fs::{self},
    io::Result,
    path::Path,
};

fn file_names(path: &str) -> Result<Vec<String>> {
    let read_dir = fs::read_dir(Path::new(path))?;
    let mut names = Vec::new();
    for dir_entry in read_dir {
        let file_name = dir_entry?.path().to_string_lossy().into_owned();
        names.push(file_name);
    }
    Ok(names)
}

fn main() -> Result<()> {
    let txn_v1_names = file_names("src/blockchain/transactions/v1")?;
    prost_build::compile_protos(&txn_v1_names, &["src".to_string()]).unwrap();
    Ok(())
}
