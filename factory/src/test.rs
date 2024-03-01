// extern  crate std;
// use crate::{Factory,FactoryClient};
// use soroban_sdk::{log, testutils::Logs, Address, Env, Map};

// #[test]
// fn increment() {
//     let env = Env::default();
//     let contract_id = env.register_contract(None, Factory);
//     let client = FactoryClient::new(&env, &contract_id);



//     let mut d:Map<u32,i128> = Map::new(&env);

//     d.set(0, 1);

//     log!(&env,"Data",d);

//     d.set(0, 2);

//     log!(&env,"Data",d);


//     let logs = env.logs().all();

//     std::println!("{}", logs.join("\n"));

// }