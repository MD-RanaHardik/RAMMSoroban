#![allow(unused)]

use soroban_sdk::{symbol_short, xdr::ToXdr, Address, Bytes, BytesN, Env, IntoVal, String, Val, Vec};

//Imported pool contract so that we can use this for cross contract call 
soroban_sdk::contractimport!(
    //path of pool contract wasm file
    file = ".././pool/target/wasm32-unknown-unknown/release/pool.wasm"
);

//Function use for create new pool from installed pool wasm
pub fn create_pool_contract(
    e: &Env,
    token_wasm_hash: BytesN<32>,
    pool_name:String
) -> Address {
    //Created new Bytes array and append pool name for unique salt
    let mut salt = Bytes::new(e);
    salt.append(&pool_name.to_xdr(e));
    
    let salt = e.crypto().sha256(&salt);
    
    //Deployed pool contract from pool wasm file which returns new pool address
    e.deployer()
        .with_current_contract(salt)
        .deploy(token_wasm_hash)
    
}