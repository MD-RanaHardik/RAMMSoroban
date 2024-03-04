# Contract Deployment

### Compile soroban contract

```
soroban contract build
```

### Get pool contract wasm hash
```
soroban contract install --wasm target/wasm32-unknown-unknown/release/pool.wasm --source alice(identity account name) --network futurenet
```

Note: Above command returns wasm hash replace that hash into factory contract (in to create_pool method )

### Deploy factory contract
```
soroban contract deploy --wasm target/wasm32-unknown-unknown/release/factory.wasm --source alice --network futurenet
```
Note: Above command returns factory contract address replace that factory contract address with factory contract address of frontend default factory contract address



