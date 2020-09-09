# Install rust
```bash
# Install
curl https://sh.rustup.rs -sSf | sh
source ~/.cargo/env

# Update Rust
rustup update nightly
rustup update stable

# Add Wasm target
rustup target add wasm32-unknown-unknown --toolchain nightly
```

# Install substrate dependancy
Install all the required dependencies with a single command (be patient, this can take up to 30
minutes).

```bash
curl https://getsubstrate.io -sSf | bash -s -- --fast
```

### Build
Once the development environment is set up, change directory to lesson7 and build the node template:
```bash
cd lesson7
cargo build
```

## Run

### Single Node Development Chain

Purge any existing dev chain state:

```bash
./target/release/node-template purge-chain --dev
```

Start a dev chain:

```bash
./target/release/node-template --dev
```

Or, start a dev chain with detailed logging:

```bash
RUST_LOG=debug RUST_BACKTRACE=1 ./target/release/node-template -lruntime=debug --dev
```
