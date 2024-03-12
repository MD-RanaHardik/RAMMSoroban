# Pool Contract

### Compile Soroban Contract

```
soroban contract build
```

### Install Pool Contract Code Into Chain
```
soroban contract install --wasm target/wasm32-unknown-unknown/release/pool.wasm --source alice(identity) --network futurenet > ids/pool_wasm_hash
```
Note: Above command put code into on-chain and return pool_wasm_hash which we store into ids/pool_wash_hash file you can use that hash into factory contract in create_pool method
