# IPFS-ETH

IPFS/Ethereum integration utility.

## Usage

```
Uploads the provided file to the IPFS network

Usage: ipfs-eth upload-file <PATH>

Arguments:
  <PATH>  Path to the file to upload
```

```
Deploys the provided Ethereum smart-contract using CID to construct it

Usage: ipfs-eth deploy-contract --url <URL> --cid <CID> --keystore-path <KEYSTORE_PATH> <SRC>

Arguments:
  <SRC>  Path to the source .sol file

Options:
  -u, --url <URL>                      URL of the Ethereum JSON-RPC API to use
  -c, --cid <CID>                      CID to pass into the smart-contract constructor
  -k, --keystore-path <KEYSTORE_PATH>  Path to the keystore of the Ethereum wallet to use
```

## Integration testing

Prerequisites:
- available `solc` binary
- running `ganache --wallet.accounts 0x6408e69185db0cd404a2494a8186d6af3426df1ce2ae9eea72511fbddc30833f,10000000000000000`
- running `kubo daemon`

```
cargo test
```