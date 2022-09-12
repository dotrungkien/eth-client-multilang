# Smart Contract client in multi languages

- JS: use ethers.js
- Rust: use ethers-rs
- Python: use web3.py

## Prerequisite

- [Foundry](https://github.com/foundry-rs/foundry

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

## Copy ABIs

- Copy abi from `contracts/out/Contract.sol/SimpleStorage.json` to `clients/abis/SimpleStorage.json`

## Run Client

### Rust

```bash
cd clients/rust/

cargo run
```
