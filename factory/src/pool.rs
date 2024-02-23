#![allow(unused)]

use soroban_sdk::{symbol_short, xdr::ToXdr, Address, Bytes, BytesN, Env, IntoVal, String, Val, Vec};

soroban_sdk::contractimport!(
    file = ".././pool/target/wasm32-unknown-unknown/release/pool.wasm"
);


pub fn create_pool_contract(
    e: &Env,
    token_wasm_hash: BytesN<32>,
    pool_name:String
) -> Address {
    let mut salt = Bytes::new(e);
    salt.append(&pool_name.to_xdr(e));
    
    let salt = e.crypto().sha256(&salt);
    
    e.deployer()
        .with_current_contract(salt)
        .deploy(token_wasm_hash)
    
}