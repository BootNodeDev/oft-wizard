Set a global version is required.

`svm use 0.8.22`

Hardhat folder structure uses `contracts` folder for contracts; on `compiler.rs` we indicate the root directory.
Remapping are taken from the `remappings` field in the `foundry.toml` file.

```sh
cargo run -p cli -- --chain base_sepolia compile
```
