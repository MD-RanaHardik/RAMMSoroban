// extern crate std;
// use soroban_sdk::{log, testutils::{Address, Logs}, vec, BytesN, Env, String};

// use crate::{Pool, PoolClient};

// fn install_token_wasm(e: &Env) -> BytesN<32> {
//     soroban_sdk::contractimport!(
//         file = "../../../token/target/wasm32-unknown-unknown/release/soroban_token_contract.wasm"
//     );
//     e.deployer().upload_contract_wasm(WASM)
// }



// #[test]
// fn hello() {
//     let env = Env::default();

//     env.mock_all_auths();

//     let contract_id = env.register_contract(None, Pool);
//     let client = PoolClient::new(&env, &contract_id);


//     let owner = <soroban_sdk::Address as Address>::generate(&env);
//     let user = <soroban_sdk::Address as Address>::generate(&env);
    

//     // client.init(&install_token_wasm(&env), &owner, &String::from_str(&env,"Test"),&1000,&1000,&5000,&7000,&3000,&2000,&1000);
//     // client.init(&install_token_wasm(&env), &owner, &String::from_str(&env,"Test"),&2000000000,&2000000000,&5000000000000,&7000000000000,&3000000000000,&4000000000,&1000);
//     client.init(&install_token_wasm(&env), &owner, &String::from_str(&env,"Test"),&2000000000,&2000000000,&10000000000,&19000000000,&1000000000,&4000000000,&1000);
    
//     client.mint_usdc_for_test(&user);

//     client.start(&owner);

//     let (usdc,pvt) = client.balance(&user);

//     log!(&env,"USDC",usdc);
//     log!(&env,"PVT",pvt);

//     client.buy(&user);
//     client.buy(&user);
//     client.buy(&user);
//     client.sell(&user);

//     let (usdc,pvt) = client.balance(&user);

//     log!(&env,"USDC",usdc);
//     log!(&env,"PVT",pvt);

//     // client.stop(&owner);

//     // client.withdraw_fund(&owner);


//     let logs = env.logs().all();

//     std::println!("{}", logs.join("\n"));
// }