#![allow(unused)]
use soroban_sdk::{xdr::ToXdr, Address, Bytes, BytesN, Env, String};

//Imported token contract so that we can use this for cross contract call
soroban_sdk::contractimport!(
    //path of token contract wasm file
    file = "./token/soroban_token_contract.wasm"
);

//Function use for create new token from installed token wasm
pub fn create_contract(
    e: &Env,
    token_wasm_hash: BytesN<32>,
    name:String
) -> Address {
     //Created new Bytes array and append token name for unique salt
    let mut salt = Bytes::new(e);
    salt.append(&name.to_xdr(e));
    
    let salt = e.crypto().sha256(&salt);

    //Deployed token contract from token wasm file which returns new token address
    e.deployer()
        .with_current_contract(salt)
        .deploy(token_wasm_hash)
}