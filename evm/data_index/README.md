### Data Index

This is the Data Index contracts that is used to store Publication deals. Basin Provider deploys this contract and is responsible to add the deal info for publications

This project is built using Foundy framework. Here is an overview of tools required to work with Foundry.

Foundry consists of:

-   **Forge**: Ethereum testing framework (like Truffle, Hardhat and DappTools).
-   **Cast**: Swiss army knife for interacting with EVM smart contracts, sending transactions and getting chain data.
-   **Anvil**: Local Ethereum node, akin to Ganache, Hardhat Network.
-   **Chisel**: Fast, utilitarian, and verbose solidity REPL.

## Documentation

https://book.getfoundry.sh/

## Usage

### Deploy

```shell
$ PRIVATE_KEY=<0xyourprivatekey> forge script script/DataIndex.s.sol:DataIndexScript --broadcast --rpc-url https://api.calibration.node.glif.io/rpc/v1 --skip-simulation --gas-estimate-multiplier 500
```
- DataIndex.s.sol contains the deployment script
- `--skip-simulation` and `--gas-estimate-multiplier` are current workaround to deploy on filecoin related networks


### Build

```shell
$ forge build
```

### Test

```shell
$ forge test
```

### Format

```shell
$ forge fmt
```

### Gas Snapshots

```shell
$ forge snapshot
```

### Anvil

```shell
$ anvil
```

### Cast

```shell
$ cast <subcommand>
```

### Help

```shell
$ forge --help
$ anvil --help
$ cast --help
```
