# basin-provider

[![Test](https://github.com/tablelandnetwork/basin-provider/actions/workflows/test.yml/badge.svg?branch=main)](https://github.com/tablelandnetwork/basin-provider/actions/workflows/test.yml)
[![License](https://img.shields.io/github/license/tablelandnetwork/basin-provider.svg)](./LICENSE)
[![standard-readme compliant](https://img.shields.io/badge/standard--readme-OK-green.svg)](https://github.com/RichardLitt/standard-readme)

> The Tableland Basin Provider

# Table of Contents

- [basin-provider](#basin-provider)
- [Table of Contents](#table-of-contents)
- [Background](#background)
- [Usage](#usage)
- [Development](#development)
- [Contributing](#contributing)
- [License](#license)

# Background
_To-do_

# Usage

```
> basin_worker --help
Ingests publications to Basin

Usage: basin_worker [OPTIONS] --database-url <DATABASE_URL>

Options:
      --evm-type <EVM_TYPE>
          EVM type (other EVM flags are ignored when this is 'mem')
          
          [env: EVM_TYPE=mem]
          [default: remote]

          Possible values:
          - mem:    Use an in-memory ephemeral EVM w/ random wallet (a BasinStorage contract will be deployed)
          - remote: Requires wallet private key, contract address, provider URL, and chain ID

      --evm-wallet-pk <EVM_WALLET_PK>
          Wallet private key (ECDSA, secp256k1) to use with the EVM (must have PUB_ADMIN_ROLE)
          
          [env: EVM_WALLET_PK=]

      --evm-contract-address <EVM_CONTRACT_ADDRESS>
          BasinStorage EVM contract address (ECDSA, secp256k1)
          
          [env: EVM_CONTRACT_ADDRESS=]

      --evm-provider-url <EVM_PROVIDER_URL>
          EVM provider URL
          
          [env: EVM_PROVIDER_URL=]
          [default: http://127.0.0.1:8545]

      --evm-chain-id <EVM_CHAIN_ID>
          EVM chain ID
          
          [env: EVM_CHAIN_ID=]
          [default: 31337]

      --database-url <DATABASE_URL>
          Postgres-style database URL
          
          [env: DATABASE_URL=]

      --bind-address <BIND_ADDRESS>
          Host and port to bind the RPC API to
          
          [env: BIND_ADDRESS=]
          [default: 127.0.0.1:3000]

      --bind-health-address <BIND_HEALTH_ADDRESS>
          Host and port to bind the Health API to
          
          [env: BIND_HEALTH_ADDRESS=]
          [default: 127.0.0.1:3001]

  -v, --verbosity...
          Logging verbosity (repeat for more verbose logging)
          
          [env: VERBOSITY=5]

  -q, --quiet
          Silence logging
          
          [env: QUIET=]

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

# Development
_To-do_

# Contributing

PRs accepted.

Small note: If editing the README, please conform to the
[standard-readme](https://github.com/RichardLitt/standard-readme) specification.

# License

MIT AND Apache-2.0, © 2021-2023 Tableland Network Contributors
