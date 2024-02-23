#![no_std]
mod pool;
extern  crate alloc;

use alloc::string::{String as STR, ToString};
use pool::create_pool_contract;

use soroban_sdk::{bytesn, contract, contractimpl, contractmeta, contracttype, Address, ConversionError, Env, Map, String, TryFromVal, Val};

mod pool_contract {
    soroban_sdk::contractimport!(
        file = ".././pool/target/wasm32-unknown-unknown/release/pool.wasm"
    );
}

#[derive(Debug,Clone, Copy)]
#[repr(u32)]
pub enum DataKey {
    Pools=1
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Pool {
    owner:Address,
    pool_name:String,
    pool_address:Address,
    pool_id:String
}

impl TryFromVal<Env, DataKey> for Val {
    type Error = ConversionError;

    fn try_from_val(_env: &Env, v: &DataKey) -> Result<Self, Self::Error> {
        Ok((*v as u32).into())
    }
}

// Metadata that is added on to the WASM custom section
contractmeta!(
    key = "Factory Contract",
    val = "Factory"
);

fn get_next_pool_id(env:&Env,pool_index:u32)-> String{
    let mut str = STR::new();

    str.push_str("RAMM Pool - ");
    str.push_str(&pool_index.to_string().as_str());
    

    let str_name = &str.as_str();

    String::from_str(&env, str_name)
}

fn get_pool_address(env:&Env,pool_id:String)->Address{

    let key = DataKey::Pools;

    let available_pools:Map<String, Pool> = env.storage().persistent().get(&key).unwrap_or(Map::new(&env));

    let is_available = available_pools.get(pool_id.clone()).is_some();

    assert!(is_available,"Pool Already Exist");

    available_pools.get(pool_id).unwrap().pool_address
}

fn set_pool(env:&Env,pool_name:&String,owner:&Address,pool_address:Address){

    let key = DataKey::Pools;

    let mut pools:Map<String, Pool> = env.storage().persistent().get(&key).unwrap_or(Map::new(&env));
    
    let pool_id = get_next_pool_id(env,pools.len());
        
    let new_pool = Pool{
        owner:owner.clone(),
        pool_address:pool_address,
        pool_name:pool_name.clone(),
        pool_id:pool_id.clone()
    };

    pools.set(pool_id, new_pool);

    env.storage().persistent().set(&key, &pools);
}

fn get_all_pool(env:&Env)->Map<String, Pool>{

    let key = DataKey::Pools;

    let pools:Map<String, Pool> = env.storage().persistent().get(&key).unwrap_or(Map::new(&env));

    pools
}







#[contract]
pub struct Factory;

#[contractimpl]
impl Factory {

    pub fn create_pool(env:Env,owner:Address,pool_name:String)->Address{

        let pool_wasm_hash = bytesn!(&env,0x1fb96ff50c74f0eb9b0762ef2a8cef09f46bb3f4861d859c2d4fdd662cd4c9f0);
        
        let token_wasm_hash = bytesn!(&env,0xc04dc2300124d5869a2dbbe81600ba0008f609e75ce254aca065c43d3a4abbe5);

        let created_pool = create_pool_contract(&env, pool_wasm_hash, pool_name.clone());

        let p = pool_contract::Client::new(&env, &created_pool);

        p.initialize(&owner, &owner, &token_wasm_hash);

        set_pool(&env, &pool_name, &owner, created_pool.clone());

        created_pool
    }

    pub fn deposit(env:Env,pool_id:String,from:Address,amount:i128){

        from.require_auth();

        let pool_address = get_pool_address(&env, pool_id);

        let pool = pool_contract::Client::new(&env, &pool_address);

        pool.deposit(&from, &amount);

    }

    pub fn withdraw(env:Env,pool_id:String,from:Address,amount:i128){

        from.require_auth();

        let pool_address = get_pool_address(&env, pool_id);

        let pool = pool_contract::Client::new(&env, &pool_address);

        pool.withdraw(&from, &amount);

    }

    pub fn get_balance(env:Env,pool_id:String,user:Address)->(i128,i128){
        
        let pool_address = get_pool_address(&env, pool_id);

        let pool = pool_contract::Client::new(&env, &pool_address);

        pool.get_balance(&user)
    }


    pub fn get_pool(env:Env)->Map<String, Pool>{

        get_all_pool(&env)
    }

         
}

