# IDCHAIN - A Substrate-Based Blockchain

![Version](https://img.shields.io/badge/version-1.0.0-blue)
![License](https://img.shields.io/badge/license-Apache%202.0-green)
![Substrate](https://img.shields.io/badge/substrate-v1.7.2-brightgreen)

> **Repository Notice**
> This is a public fork of [IDCHAIN Gitlab](https://git.247.or.id/pandi/idchain-testnet/-/tree/feat/async-backing?ref_type=heads) (private). To comply with licensing and confidentiality requirements, this fork contains a limited, sanitized subset of the source code and history. Some features are intentionally omitted or replaced with placeholders.

## Overview

IDCHAIN is a blockchain infrastructure built with [Substrate](https://substrate.io/), designed as a Layer-1 solution for decentralized identity and digital credentials. The project leverages cutting-edge blockchain technology to provide a scalable, secure, and decentralized network.

### Project Components
- **Relay Chain Support**: Full integration with Polkadot SDK for relay chain validators
- **DID System**: Decentralized Identifier (DID) capabilities for identity management
- **Credentials**: Public credential management and verification
- **Delegation**: Delegation logic for identity operations

For complete project details and architecture, refer to the [IDCHAIN Relaychain Deployment Handbook](https://hackmd.io/Z6f3ZRJBTbaXffKNshFhDQ?view).

## Quick Start

### System Requirements

Before starting, ensure your system meets these requirements:

**Hardware (Recommended for Production)**
- **CPU**: 8 physical cores @ 3.4GHz (Intel Ice Lake or newer, AMD Zen3 or newer)
- **Memory**: 32 GB DDR4 ECC
- **Storage**: 1 TB NVMe SSD
- **Network**: Minimum 500 Mbit/s symmetric bandwidth
- **OS**: Linux Kernel 5.16 or newer (Ubuntu 24.04 recommended)

**Development Requirements**
- Rust 1.74.0 or newer
- Git, Clang, and build essentials
- Optional: Docker for containerized deployment

### Prerequisites Setup

#### Install Rust and Dependencies

```sh
sudo apt install --assume-yes git clang curl libssl-dev protobuf-compiler make pkg-config build-essential
```

```sh
curl https://sh.rustup.rs -sSf | sh -s -- -y
source $HOME/.cargo/env
```

Install specific Rust version:
```sh
rustup install 1.74.0
rustup default 1.74.0
cargo --version  # Verify installation
```

**For macOS users:**
```sh
brew install cmake pkg-config openssl git llvm
```

#### Configure NTP (Critical for Network Nodes)

```sh
timedatectl  # Check if NTP is synchronized
sudo apt-get install ntp  # If not installed
sudo ntpq -p  # Verify NTP status
```

> **Warning**: NTP synchronization is critical for block authorship. Nodes with unsynchronized clocks may miss block opportunities.

#### Verify Landlock is Enabled

```sh
sudo dmesg | grep landlock || sudo journalctl -kg landlock
```

### Build the Project

```sh
cargo build --release
```

The build process may take 30-80 minutes depending on your hardware. After completion, verify the installation:

```sh
./target/release/did-node --version
```

### Generate Documentation

```sh
cargo +nightly doc --open
```

## Running Nodes

### Development Mode

#### Single-Node Development Chain (DID)

```sh
./target/release/did-node --dev
```

To start with detailed logging:
```sh
RUST_BACKTRACE=1 ./target/release/did-node -ldebug --dev
```

To persist state between runs:
```sh
mkdir -p ~/my-chain-state
./target/release/did-node --dev --base-path ~/my-chain-state/
```

To purge the development chain's state:
```sh
./target/release/did-node purge-chain --dev
```

### Multi-Node Local Testnet

Refer to [Simulate a Network](https://docs.substrate.io/tutorials/get-started/simulate-network/) in the Substrate documentation.

### Production Deployment

For complete production deployment instructions, refer to the [IDCHAIN Relaychain Deployment Handbook](https://hackmd.io/Z6f3ZRJBTbaXffKNshFhDQ?view), which includes:

- Hardware requirements and benchmarks
- Validator key generation and management
- Chain specification configuration
- Systemd service setup for automated management
- RPC endpoint setup with Nginx for WebSocket access
- Network bootstrapping and monitoring

**Quick Start:**
```sh
git clone https://git.247.or.id/munir/idchain-polkadot-sdk.git
cd idchain-polkadot-sdk
cargo build --release
./target/release/subkey generate --scheme Sr25519  # Generate validator keys
```

#### Parachain Collator Setup

Parachain deployment is now handled through standardized infrastructure. For complete details, refer to the [IDCHAIN Parachain Deployment Handbook](https://hackmd.io/8TOVne-uQoCYHWVTykn2CA?view).

#### Relay Chain Validator Setup

Complete deployment instructions are available in the [IDCHAIN Relaychain Deployment Handbook](https://hackmd.io/Z6f3ZRJBTbaXffKNshFhDQ?view).

**Quick Summary:**
1. Build the relay chain binaries from [idchain-polkadot-sdk](https://git.247.or.id/munir/idchain-polkadot-sdk.git)
2. Generate validator keys using `subkey`
3. Create and configure custom chain specification files
4. Configure systemd services for automated management
5. Set up RPC endpoints with Nginx for WebSocket access

**Key Steps:**
```sh
git clone https://git.247.or.id/munir/idchain-polkadot-sdk.git
cd idchain-polkadot-sdk
cargo build --release
./target/release/subkey generate --scheme Sr25519  # Generate validator keys
```

## Chain Specifications

A chain specification (chain spec) defines the initial state (genesis) of your blockchain, including network name, boot nodes, validator set, genesis accounts, and runtime parameters.

### Chain Spec Files

Chain specifications are available in the `chainspecs/` directory:
- **Mainnet**: `chainspecs/parachain-mainnet/parachain-mainnet.json`
- **Testnet**: `chainspecs/parachain-testnet/`
- **Staging**: `chainspecs/parachain-testnet-stg/`
- **Standalone**: `chainspecs/standalone/standalone-testnet.json`

### Generating Chain Specifications

#### DID Node
```bash
./target/release/did-node build-spec --chain dev > did-dev-spec.json
```

#### Raw Format (for production)
```bash
./target/release/did-node build-spec --chain=did-dev-spec.json --raw --disable-default-bootnode > did-dev-spec-raw.json
```

### Chain Spec Customization

Modify the JSON chain spec file to:
1. Update `name`, `id`, and `protocolId` fields
2. Configure consensus parameters (`aura`, `grandpa`)
3. Set initial validators and their session keys
4. Define genesis balances

For detailed chain spec guidelines, refer to:
- [IDCHAIN Relaychain Chainspec Guide](https://hackmd.io/Z6f3ZRJBTbaXffKNshFhDQ?view#IDCHAIN-Relaychain-Chainspec-Guide)

## Interacting with the Network

### Polkadot.js Apps

Connect to the running node using the Polkadot.js Apps interface:

1. **Local Development Node**: [http://polkadot.js.org/apps/?rpc=ws://127.0.0.1:9944](http://polkadot.js.org/apps/?rpc=ws://127.0.0.1:9944)
2. **Alternative**: [IPFS Version](ipns://dotapps.io/?rpc=ws%3A%2F%2F127.0.0.1%3A9944#/explorer)

### WebSocket RPC Setup

For production nodes, expose the RPC endpoint through Nginx with WebSocket support:

```nginx
server {
    server_name node-rpc.idchain.id;

    location / {
        proxy_pass http://localhost:9944;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "Upgrade";
        proxy_set_header Host $host;
    }

    listen 80;
}
```

Then enable SSL:
```bash
sudo apt install certbot python3-certbot-nginx
sudo certbot --nginx -d node-rpc.idchain.id
```

Access via: `https://polkadot.js.org/apps/?rpc=wss://node-rpc.idchain.id`

## Project Architecture

### Network Structure

IDCHAIN follows a modular architecture designed for scalability and interoperability:

```
IDCHAIN Ecosystem
├── Relay Chain (Validator Network)
│   ├── Validators (block production & finality)
│   └── Authority Discovery
│
```

### Validator Node

A validator node participates in consensus by:
- Following the consensus mechanism (Aura/BABE for block production, GRANDPA for finality)
- Validating blocks from other network participants
- Maintaining the blockchain state
- Exposing RPC endpoints for external access

**Validator Requirements:**
- Reference hardware specs (see System Requirements above)
- Synchronized clock (NTP)
- Stable, high-bandwidth internet connection
- Generated validator keys and session keys

### Runtime Implementation

The runtime is the core logic of the blockchain responsible for:
- Validating blocks
- Executing state transitions
- Processing extrinsics (transactions)

IDCHAIN uses [FRAME](https://docs.substrate.io/fundamentals/runtime-development/#frame) to construct the runtime, allowing flexible composition of domain-specific logic through **pallets**.

### Pallets (Runtime Modules)

Custom pallets are located in the `pallets/` directory:

| Pallet | Purpose |
|--------|---------|
| `delegation` | Delegation logic for governance |
| `pallet-dip-consumer` | DIP (Decentralized Identity Provider) consumer pallet |
| `pallet-dip-provider` | DIP provider functionality |
| `pallet-did-lookup` | DID to address mapping |
| `pallet-web3-names` | Web3 name registration and resolution |
| `public-credentials` | Credential management and verification |
| `verification` | Identity verification mechanisms |
| `uid-core` | Universal Identity core functionality |
| `uid-credential` | Universal Identity credential management |
| `pallet-asset-switch` | Asset switching capabilities |
| `pallet-configuration` | Configuration management |
| `pallet-deposit-storage` | Deposit and storage management |
| `pallet-migration` | Migration utilities |
| `pallet-relay-store` | Relay chain state store |

Each pallet includes:
- **Storage**: Key-value data structures for chain state
- **Dispatchables**: Callable functions (extrinsics) to modify state
- **Events**: Notifications for state changes
- **Errors**: Error handling and reporting

### Runtime APIs

Located in `runtime-api/`, these provide read-only access to runtime state:
- **`asset-switch`**: Query asset switching functionality
- **`did`**: DID lookup and management queries
- **`dip-provider`**: DIP provider information and queries
- **`public-credentials`**: Credential and verification queries
- **`staking`**: Staking information and rewards

## Alternative Installations

### Nix

Install [Nix](https://nixos.org/), and optionally [direnv](https://github.com/direnv/direnv) and [lorri](https://github.com/nix-community/lorri) for a plug-and-play development environment:

```sh
direnv allow
lorri shell
```

### Docker

For containerized deployment, refer to [Substrate Docker instructions](https://github.com/paritytech/substrate/blob/master/docker/README.md).

## Documentation & Resources

### Official Handbooks

Comprehensive deployment and configuration guides:

- **[IDCHAIN Relaychain Deployment Handbook](https://hackmd.io/Z6f3ZRJBTbaXffKNshFhDQ?view)**
  - Complete validator node setup guide
  - Hardware requirements and benchmarks
  - Chain specification configuration
  - Systemd service management
  - RPC endpoint setup with Nginx
  - Polkadot.js integration

### External Resources

- **[Substrate Documentation](https://docs.substrate.io/)**
- **[Polkadot Wiki](https://wiki.polkadot.network/)**
- **[FRAME Developer Guide](https://docs.substrate.io/fundamentals/runtime-development/)**
- **[Polkadot JS Apps](https://polkadot.js.org/apps/)**

## Testing & Benchmarking

### Running Tests

```sh
# Run all tests
cargo test --release

# Run tests for a specific pallet
cargo test -p pallet-dip-consumer --release
```

### Benchmarking

Benchmark your pallets to optimize performance:

```sh
cargo build --package frame-benchmarking-cli --release

./target/release/substrate benchmark pallet \
  --chain dev \
  --pallet '*' \
  --extrinsic '*'
```

### Zombienet Testing

For complex network simulations, use Zombienet:

1. Download the [Zombienet binary](https://github.com/paritytech/zombienet/releases)
2. Clone the [Polkadot SDK](https://github.com/paritytech/polkadot-sdk) (same branch)
3. Generate plain chain spec:
   ```bash
   ./target/release/idchain-parachain build-spec --disable-default-bootnode --dev > chainspecs/plain-idchain-chainspec.json
   ```
4. Configure `scripts/zombienet/config.toml` and run:
   ```bash
   cd scripts/zombienet
   zombienet -p native spawn config.toml
   ```

## Common Issues & Troubleshooting

### Build Fails

**Solution**: Ensure Rust version is correct and dependencies are installed:
```sh
rustup update
rustup default 1.74.0
rustc --version
```

### Node Won't Start

**Solution**: Check ports aren't already in use:
```sh
lsof -i :9944  # Check RPC port
lsof -i :30333  # Check P2P port
```

### Keys Not Injecting

**Solution**: Verify the key format and use the correct RPC methods:
```sh
curl -H "Content-Type: application/json" -d '{
  "jsonrpc": "2.0",
  "method": "author_insertKey",
  "params": ["aura", "<seed_phrase>", "<public_key_hex>"],
  "id": 1
}' http://localhost:9944
```

### Performance Issues

**Checklist:**
- Verify system clock is synchronized (NTP)
- Check network connectivity to peers
- Monitor CPU, memory, and disk I/O
- Increase `--wasm-execution` optimization if needed
- Consider upgrading hardware

## Security

### Key Management

⚠️ **Critical Security Practices**:
- Never share your seed phrases or secret keys
- Store sudo keys in a secure, offline location
- Use hardware wallets for production validator keys
- Rotate keys periodically
- Use environment variables or secure vaults for sensitive data

### Validator Security

- Keep your node software up to date
- Firewall RPC ports appropriately
- Use VPN for remote management
- Monitor node logs for suspicious activity
- Enable Nginx SSL/TLS for all RPC endpoints

## License

This project is licensed under the terms specified in the `LICENSE` file. Refer to the [IDCHAIN private repository](https://git.247.or.id/pandi/idchain-testnet/) for the complete licensing information.

## Support & Community

For issues and discussions:
- Check existing documentation in the [handbooks](#documentation--resources)
- Review the [Substrate Stack Exchange](https://substrate.stackexchange.com/)
- Engage with the [Polkadot community](https://polkadot.network/community/)

---
