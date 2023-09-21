# basin-provider

[![Test](https://github.com/tablelandnetwork/basin-provider/actions/workflows/test.yml/badge.svg?branch=main)](https://github.com/tablelandnetwork/basin-provider/actions/workflows/test.yml)
[![License](https://img.shields.io/github/license/tablelandnetwork/basin-provider.svg)](./LICENSE)
[![standard-readme compliant](https://img.shields.io/badge/standard--readme-OK-green.svg)](https://github.com/RichardLitt/standard-readme)

> Data ingestion for Tableland Basin

# Table of Contents

- [basin-provider](#basin-provider)
- [Table of Contents](#table-of-contents)
- [Background](#background)
- [Usage](#usage)
  - [Basin Worker](#basin-worker)
  - [Basin Exporter](#basin-exporter)
- [Development](#development)
  - [Running](#running)
  - [Run Tests](#run-tests)
- [Contributing](#contributing)
- [License](#license)

# Background

Tableland Basin is a secure and verifiable open data platform. The Basin Provider is a collection of daemons and libraries that work in conjuction with [`basin-cli`](https://github.com/tablelandnetwork/basin-cli.git) and [`basin-storage`](https://github.com/tablelandnetwork/basin-storage.git).

`basin-provider` is responsible for ingesting Basin Publications that are archived on Filecoin by [`basin-storage`](https://github.com/tablelandnetwork/basin-storage.git). Publication data can be ingested in two forms:
1. Continuously published data from a connected database (currently, only PostgreSQL is supported)
2. Raw parquet files

See [`basin-cli`](https://github.com/tablelandnetwork/basin-cli.git) for usage info.

This repo contains the following crates:
- [`basin_worker`](/lib/worker)`: Daemon that exposes a Cap’n Proto RPC API for creating publications and ingesting publication data.
- [`basin_exporter`](/lib/exporter): Daemon that periodically exports publication data as parquet to [`basin-storage`](https://github.com/tablelandnetwork/basin-storage.git).
- [`basin_evm`](/lib/evm): Rust bindings for the [`basin-storage`](https://github.com/tablelandnetwork/basin-storage.git) EVM contract.
- [`basin_protocol`](/lib/protocol): Cap’n Proto protocol definitions and Rust bindings.
- [`basin_common`](/lib/common): Shared types and utilities.

🚧 Basin is currently not in a production-ready state. Any data that is pushed to the network may be subject to deletion. 🚧

# Usage

## Basin Worker

```bash
basin_worker --help

Ingest daemon for Tableland Basin

Usage: basin_worker [OPTIONS] --export-bucket <EXPORT_BUCKET> --export-credentials <EXPORT_CREDENTIALS> --database-url <DATABASE_URL>

Options:
      --evm-type <EVM_TYPE>
          EVM type (other EVM flags are ignored when this is 'mem')

          [env: EVM_TYPE=]
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

      --export-bucket <EXPORT_BUCKET>
          Parquet export GCS bucket

          [env: EXPORT_BUCKET=]

      --export-credentials <EXPORT_CREDENTIALS>
          Parquet export sink credentials

          [env: EXPORT_CREDENTIALS=]

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
          
          [env: VERBOSITY=]

  -q, --quiet
          Silence logging
          
          [env: QUIET=]

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

## Basin Exporter

```bash
basin_exporter --help

Parquet exporter daemon for Tableland Basin

Usage: basin_exporter [OPTIONS] --export-bucket <EXPORT_BUCKET> --export-credentials <EXPORT_CREDENTIALS> --database-url <DATABASE_URL>

Options:
      --export-bucket <EXPORT_BUCKET>
          Parquet export GCS bucket [env: EXPORT_BUCKET=]
      --export-credentials <EXPORT_CREDENTIALS>
          Parquet export sink credentials [env: EXPORT_CREDENTIALS=]
      --export-schedule <EXPORT_SCHEDULE>
          Parquet export crontab schedule [env: EXPORT_SCHEDULE=] [default: "0 0 0 * * *"]
      --database-url <DATABASE_URL>
          Postgres-style database URL [env: DATABASE_URL=]
      --bind-health-address <BIND_HEALTH_ADDRESS>
          Host and port to bind the Health API to [env: BIND_HEALTH_ADDRESS=] [default: 127.0.0.1:3001]
  -v, --verbosity...
          Logging verbosity (repeat for more verbose logging) [env: VERBOSITY=]
  -q, --quiet
          Silence logging [env: QUIET=]
  -h, --help
          Print help
  -V, --version
          Print version
```

# Development

## Running

```bash
docker compose up
```

## Run Tests

```bash
docker compose up crdb
cargo test
```

# Contributing

PRs accepted.

Small note: If editing the README, please conform to the
[standard-readme](https://github.com/RichardLitt/standard-readme) specification.

# License

MIT AND Apache-2.0, © 2021-2023 Tableland Network Contributors
