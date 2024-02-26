use core::str::FromStr;
extern crate std;

use crate::{storage, Pool, PoolClient};

use soroban_sdk::{bytesn, log, symbol_short, testutils::{Address, Logs}, vec, BytesN, Env, String};

fn install_token_wasm(e: &Env) -> BytesN<32> {
    soroban_sdk::contractimport!(
        file = "../../../token/target/wasm32-unknown-unknown/release/soroban_token_contract.wasm"
    );
    e.deployer().upload_contract_wasm(WASM)
}

fn install_usdc_wasm(e: &Env) -> BytesN<32> {
    soroban_sdk::contractimport!(
        file = "../../../token/target/wasm32-unknown-unknown/release/soroban_token_contract.wasm"
    );
    e.deployer().upload_contract_wasm(WASM)
}


#[test]
fn hello() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Pool);
    let client = PoolClient::new(&env, &contract_id);

    let token_wasm_hash = bytesn!(&env,0xc04dc2300124d5869a2dbbe81600ba0008f609e75ce254aca065c43d3a4abbe5);

    let owner = <soroban_sdk::Address as Address>::generate(&env);
    let user = <soroban_sdk::Address as Address>::generate(&env);
    

    // client.init(&token_wasm_hash, &owner, &String::from_str(&env,"Test"), &1000, &2000, &5000, &7000, &3000, &2000, &10000000);
    client.init(&install_token_wasm(&env),&owner, &String::from_str(&env,"Test"), &1, &1, &1, &1, &1, &1,&10000000);

    client.mint_usdc_for_test(&user);

    client.buy(&user);

    let logs = env.logs().all();

    std::println!("{}", logs.join("\n"));
}