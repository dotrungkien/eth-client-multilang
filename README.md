# BC Client in Rust

a sample of blockchain client in Rust.

## Prerequisite

- [Foundry](https://github.com/foundry-rs/foundry)
- [Ethers-rs](https://github.com/gakonst/ethers-rs)

## Compile & Deploy contract

- run a local node:

```sh
anvil
ETH_FROM=<account address from anvil>
```

- deploy contract to local net

```sh
cd contracts/
forge create src/Contract.sol:SimpleStorage --constructor-args 123 --unlocked --from $ETH_FROM
SIMPLE_STORAGE=<your deployed address>
```

- test functionality

```sh
cast --to-dec $(cast call $SIMPLE_STORAGE "get()")
> 123

cast send $SIMPLE_STORAGE "set(uint256)" 321 --from $ETH_FROM

cast --to-dec $(cast call $SIMPLE_STORAGE "get()")
> 321
```

## Client

- Copy abi from `contracts/out/Contract.sol/SimpleStorage.json` to `abis/SimpleStorage.json`

- Source code in `src/main.rs`

- Run

```
cargo run
```
