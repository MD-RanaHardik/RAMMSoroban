# Factory Contract

### Compile Soroban Contract

```
soroban contract build
```

### Deploy Factory Contract
```
soroban contract deploy --wasm target/wasm32-unknown-unknown/release/factory.wasm --source alice --network futurenet
```
Note: Above command returns factory contract address replace that factory contract address with factory contract address of frontend default factory contract address


### Create Pool
```
soroban contract invoke --id CBJTB3DOF3JW5P7OLTWS45Z2B4DTTCTLVWSY6YFOVJ2COJ4TUEBAVQIT(deployed factory contract address) --source alice(identity) --network futurenet -- create_pool --owner alice --pool_name "Pool Name" --pvt_qty_max_primary 1000 --pvt_price_max_primary 5000 --pvt_price_initial_primary 1 --pvt_available_secondary 2000 --steepness 1000

```

### Start Pool
```
soroban contract invoke --id CBJTB3DOF3JW5P7OLTWS45Z2B4DTTCTLVWSY6YFOVJ2COJ4TUEBAVQIT(deployed factory contract address) --source alice(identity) --network futurenet -- start --pool_id "PoolID" --owner alice

```

### Buy
```
soroban contract invoke --id CBJTB3DOF3JW5P7OLTWS45Z2B4DTTCTLVWSY6YFOVJ2COJ4TUEBAVQIT(deployed factory contract address) --source alice(identity) --network futurenet -- buy --pool_id "PoolID" --from alice

```

### Sell
```
soroban contract invoke --id CBJTB3DOF3JW5P7OLTWS45Z2B4DTTCTLVWSY6YFOVJ2COJ4TUEBAVQIT(deployed factory contract address) --source alice(identity) --network futurenet -- sell --pool_id "PoolID" --from alice

```


### Stop Pool
```
soroban contract invoke --id CBJTB3DOF3JW5P7OLTWS45Z2B4DTTCTLVWSY6YFOVJ2COJ4TUEBAVQIT(deployed factory contract address) --source alice(identity) --network futurenet -- stop --pool_id "PoolID" --owner alice

```

### Withdraw Fund
```
soroban contract invoke --id CBJTB3DOF3JW5P7OLTWS45Z2B4DTTCTLVWSY6YFOVJ2COJ4TUEBAVQIT(deployed factory contract address) --source alice(identity) --network futurenet -- withdraw_fund --pool_id "PoolID" --owner alice

```





