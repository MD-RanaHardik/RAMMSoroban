# Token Contract

### Compile Soroban Contract

```
soroban contract build
```

### Install Token Contract Code Into Chain
```
soroban contract install --wasm target/wasm32-unknown-unknown/release/soroban_token_contract.wasm --source alice(identity) --network futurenet > ids/token_wasm_hash
```
Note: Above command put code into on-chain and return token_wasm_hash which we store into ids/token_wash_hash file you can use that hash into factory contract in create_pool method
