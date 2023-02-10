use std::path::PathBuf;

use ipfs_eth::{eth, ipfs};

#[tokio::test]
async fn integration() {
    let cid = ipfs::upload_file(asset("img.jpg")).await.expect("ipfs::upload_file");
    let contract = eth::DeployContract {
        src: asset("contract.sol"),
        url: "http://127.0.0.1:8545".parse().expect("Valid Url"),
        constructor_args: cid.clone(),
        keystore_path: asset("keystore"),
        keystore_password: "1".to_string().into(),
    }
    .exec()
    .await
    .expect("eth::DeployContract");

    let stored_cid = contract
        .method::<_, String>("get_cid", ())
        .expect("Valid method")
        .call()
        .await
        .expect("get_cid()");

    assert_eq!(cid, stored_cid);
}

fn asset(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/assets").join(name)
}
