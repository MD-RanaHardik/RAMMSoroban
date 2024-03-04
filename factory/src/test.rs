// extern  crate std;
// use crate::{Factory,FactoryClient};
// use soroban_sdk::{log, testutils::Logs, Address, Env, Map};

// #[test]
// fn increment() {
//     let env = Env::default();
//     let contract_id = env.register_contract(None, Factory);
//     let client = FactoryClient::new(&env, &contract_id);

//     let logs = env.logs().all();

//     std::println!("{}", logs.join("\n"));

// }