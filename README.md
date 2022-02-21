Counter example in Rust
=================================
```
near keys contract.spread.testnet   - если удалить тут ключ то контракт будет залоченным и уже не изменяемым

export CONTRACT_NAME=contract.spread.testnet
echo $CONTRACT_NAME

near view contract.spread.testnet get_num '{}'
near call contract.spread.testnet increment '{}' --accountId spread.testnet --amount 2
near view contract.spread.testnet get_users '{}'



near dev-deploy -f out/main.wasm  - деплой нового контракта чтоб не делать миграцию

export CONTRACT_NAME=dev-1644433678195-58699302756227
near view dev-1644433678195-58699302756227 get_users '{}'
near call dev-1644433678195-58699302756227 new '{}' --accountId spread.testnet 

near call dev-1644433678195-58699302756227 make_new_insurance '{"contract_address": "contract_addressABC", "nft_id": "nft_id123", "image_hash": "HGJGJFGHF"}' --accountId spread.testnet --amount 1.5

near call dev-1644433678195-58699302756227 new'{}' --accountId spread.testnet

near view dev-1644875835620-48624210077592 get_insurance_data '{"contract_address": "contract_addressABC", "nft_id": "nft_id123"}' --accountId spread.testnet  

near view dev-1644875835620-48624210077592 get_hash_image_nft '{"contract_address": "contract_addressABC", "nft_id": "nft_id123"}' --accountId spread.testnet  

```

[![Open in Gitpod!](https://gitpod.io/button/open-in-gitpod.svg)](https://gitpod.io/#https://github.com/near-examples/rust-counter)

<!-- MAGIC COMMENT: DO NOT DELETE! Everything above this line is hidden on NEAR Examples page -->

## Description

This contract implements simple counter backed by storage on blockchain.
Contract in `contract/src/lib.rs` provides methods to increment / decrement counter and get it's current value or reset.

Plus and minus buttons increase and decrease value correspondingly. When button L is toggled, a little light turns on, just for fun. RS button is for reset. LE and RE buttons to let the robot wink at you.

## To Run
Open in the Gitpod link above or clone the repository.

```
git clone https://github.com/near-examples/rust-counter
```


## Setup [Or skip to Login if in Gitpod](#login)
Install dependencies:

```
yarn
```

If you don't have `Rust` installed, complete the following 3 steps:

1) Install Rustup by running:

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

([Taken from official installation guide](https://www.rust-lang.org/tools/install))

2) Configure your current shell by running:

```
source $HOME/.cargo/env
```

3) Add wasm target to your toolchain by running:

```
rustup target add wasm32-unknown-unknown
```

Next, make sure you have `near-cli` by running:

```
near --version
```

If you need to install `near-cli`:

```
npm install near-cli -g
```

## Login
If you do not have a NEAR account, please create one with [NEAR Wallet](https://wallet.testnet.near.org).

In the project root, login with `near-cli` by following the instructions after this command:

```
near login
```

Modify the top of `src/config.js`, changing the `CONTRACT_NAME` to be the NEAR account that was just used to log in.

```javascript
…
const CONTRACT_NAME = 'YOUR_ACCOUNT_NAME_HERE'; /* TODO: fill this in! */
…
```

Start the example!

```
yarn start
```

## To Test

```
cd contract
cargo test -- --nocapture
```

## To Explore

- `contract/src/lib.rs` for the contract code
- `src/index.html` for the front-end HTML
- `src/main.js` for the JavaScript front-end code and how to integrate contracts
- `src/test.js` for the JS tests for the contract

## To Build the Documentation

```
cd contract
cargo doc --no-deps --open
```
