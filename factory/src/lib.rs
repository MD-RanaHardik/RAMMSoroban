#![no_std]
mod pool;
mod test;
extern  crate alloc;

use alloc::string::{String as STR, ToString};
use pool::create_pool_contract;

use soroban_sdk::{bytesn, contract, contractimpl, contractmeta, contracttype, Address, ConversionError, Env, Map, String, TryFromVal, Val};

mod pool_contract {
    soroban_sdk::contractimport!(
        file = ".././pool/target/wasm32-unknown-unknown/release/pool.wasm"
    );
}

const Q9:i128 = (10 as i128).pow(9);

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
    pool_id:String,
    archived:bool,
    treasury:i128,
    x:i128,
    pvt_qty_max_primary:i128,
    pvt_qty_max_secondary:i128,
    pvt_price_max_primary:i128,
    pvt_price_max_secondary:i128,
    pvt_price_initial_primary:i128,
    c_primary_steepness:u32,
    pool_status:u32
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

fn set_pool(env:&Env,pool_name:&String,owner:&Address,pool_address:Address,pvt_qty_max_primary:i128,pvt_price_max_primary:i128,pvt_price_max_secondary:i128,pvt_price_initial_primary:i128,c_primary_steepness:u32){

    let key = DataKey::Pools;

    let mut pools:Map<String, Pool> = env.storage().persistent().get(&key).unwrap_or(Map::new(&env));
    
    let pool_id = get_next_pool_id(env,pools.len());
        
    let new_pool = Pool{
        owner:owner.clone(),
        pool_address:pool_address,
        pool_name:pool_name.clone(),
        pool_id:pool_id.clone(),
        archived:false,
        treasury:0,
        x:(1 * Q9),
        pvt_qty_max_primary:(pvt_qty_max_primary * Q9),
        pvt_qty_max_secondary:(pvt_qty_max_primary * Q9),
        pvt_price_max_primary:(pvt_price_max_primary * Q9),
        pvt_price_max_secondary:(pvt_price_max_secondary * Q9),
        pvt_price_initial_primary:(pvt_price_initial_primary * Q9),
        c_primary_steepness:c_primary_steepness,
        pool_status:0
    };

    pools.set(pool_id, new_pool);

    env.storage().persistent().set(&key, &pools);
}

fn get_all_pool(env:&Env)->Map<String, Pool>{

    let key = DataKey::Pools;

    let pools:Map<String, Pool> = env.storage().persistent().get(&key).unwrap_or(Map::new(&env));

    pools
}


fn set_pool_x_and_treasury(env:&Env,pool_id:String,x:i128,treasury:i128){

    let key = DataKey::Pools;

    let mut available_pools:Map<String, Pool> = env.storage().persistent().get(&key).unwrap_or(Map::new(&env));

    let is_available = available_pools.get(pool_id.clone()).is_some();

    assert!(is_available,"Pool Not Exist");

    let mut pool_data = available_pools.get(pool_id.clone()).unwrap();

    pool_data.x = x * Q9;

    pool_data.treasury = treasury;

    available_pools.set(pool_id, pool_data);

    env.storage().persistent().set(&key,&available_pools);
}

fn set_pool_pvt_qty_max_secondary(env:&Env,pool_id:String,pvt_qty_max_secondary:i128){

    let key = DataKey::Pools;

    let mut available_pools:Map<String, Pool> = env.storage().persistent().get(&key).unwrap_or(Map::new(&env));

    let is_available = available_pools.get(pool_id.clone()).is_some();

    assert!(is_available,"Pool Not Exist");

    let mut pool_data = available_pools.get(pool_id.clone()).unwrap();

    pool_data.pvt_qty_max_secondary = pvt_qty_max_secondary;

    available_pools.set(pool_id, pool_data);

    env.storage().persistent().set(&key,&available_pools);
}

fn set_pool_archive(env:&Env,pool_id:String,archive:bool){

    let key = DataKey::Pools;

    let mut available_pools:Map<String, Pool> = env.storage().persistent().get(&key).unwrap_or(Map::new(&env));

    let is_available = available_pools.get(pool_id.clone()).is_some();

    assert!(is_available,"Pool Not Exist");

    let mut pool_data = available_pools.get(pool_id.clone()).unwrap();

    pool_data.archived = archive;

    available_pools.set(pool_id, pool_data);

    env.storage().persistent().set(&key,&available_pools);
}


fn set_pool_status(env:&Env,pool_id:String,status:u32){

    let key = DataKey::Pools;

    let mut available_pools:Map<String, Pool> = env.storage().persistent().get(&key).unwrap_or(Map::new(&env));

    let is_available = available_pools.get(pool_id.clone()).is_some();

    assert!(is_available,"Pool Not Exist");

    let mut pool_data = available_pools.get(pool_id.clone()).unwrap();

    pool_data.pool_status = status;

    available_pools.set(pool_id, pool_data);

    env.storage().persistent().set(&key,&available_pools);
}

fn set_pool_fund_to_zero(env:&Env,pool_id:String){

    let key = DataKey::Pools;

    let mut available_pools:Map<String, Pool> = env.storage().persistent().get(&key).unwrap_or(Map::new(&env));

    let is_available = available_pools.get(pool_id.clone()).is_some();

    assert!(is_available,"Pool Not Exist");

    let mut pool_data = available_pools.get(pool_id.clone()).unwrap();

    pool_data.treasury = 0;

    available_pools.set(pool_id, pool_data);

    env.storage().persistent().set(&key,&available_pools);
}








#[contract]
pub struct Factory;

#[contractimpl]
impl Factory {

    pub fn create_pool(env:Env,owner:Address,pool_name:String,pvt_qty_max_primary:i128,pvt_price_max_primary:i128,pvt_price_initial_primary:i128,pvt_available_secondary:i128,steepness:u32)->Address{

        let pool_wasm_hash = bytesn!(&env,0x095323bf5622ab22863237a9a98f5543ec633b3e972c4a1faff0ae553c0108a9);
        
        let token_wasm_hash = bytesn!(&env,0xc04dc2300124d5869a2dbbe81600ba0008f609e75ce254aca065c43d3a4abbe5);

        let created_pool = create_pool_contract(&env, pool_wasm_hash, pool_name.clone());

        let p = pool_contract::Client::new(&env, &created_pool);

        let pvt_price_max_secondary = (2 * pvt_price_max_primary) - pvt_price_initial_primary;

        p.init(&token_wasm_hash,&owner,&pool_name,&pvt_qty_max_primary,&pvt_qty_max_primary,&pvt_price_max_primary,&pvt_price_max_secondary,&pvt_price_initial_primary,&pvt_available_secondary,&steepness);

        set_pool(&env, &pool_name, &owner, created_pool.clone(),pvt_qty_max_primary,pvt_price_max_primary,pvt_price_max_secondary,pvt_price_initial_primary,steepness);

        created_pool
    }

    pub fn start(env:Env,pool_id:String,owner:Address){

        owner.require_auth();

        let pool_address = get_pool_address(&env, pool_id.clone());

        let pool = pool_contract::Client::new(&env, &pool_address);

        pool.start(&owner);

        set_pool_status(&env,pool_id,1);
    }

    pub fn stop(env:Env,pool_id:String,owner:Address){

        owner.require_auth();

        let pool_address = get_pool_address(&env, pool_id.clone());

        let pool = pool_contract::Client::new(&env, &pool_address);

        pool.stop(&owner);

        set_pool_status(&env,pool_id,2);
    }

    pub fn expand(env:Env,pool_id:String,owner:Address,amount:i128){

        owner.require_auth();

        let pool_address = get_pool_address(&env, pool_id.clone());

        let pool = pool_contract::Client::new(&env, &pool_address);

        let (is_expand,pvt_qty_max_secondary) = pool.expand(&owner,&amount);

        if is_expand {
            set_pool_pvt_qty_max_secondary(&env, pool_id, pvt_qty_max_secondary);
        }

    }


    pub fn buy(env:Env,pool_id:String,from:Address){

        from.require_auth();

        let pool_address = get_pool_address(&env, pool_id.clone());

        let pool = pool_contract::Client::new(&env, &pool_address);

        let (x,treasury) = pool.buy(&from);

        set_pool_x_and_treasury(&env,pool_id,x,treasury);
    }

    pub fn sell(env:Env,pool_id:String,from:Address){

        from.require_auth();

        let pool_address = get_pool_address(&env, pool_id.clone());

        let pool = pool_contract::Client::new(&env, &pool_address);
        
        let (in_secondary_mode,x,treasury) = pool.sell(&from);

        if in_secondary_mode {
            set_pool_x_and_treasury(&env,pool_id,x,treasury);
        }

    }

    pub fn withdraw_fund(env:Env,pool_id:String,owner:Address){

        owner.require_auth();

        let pool_address = get_pool_address(&env, pool_id.clone());

        let pool = pool_contract::Client::new(&env, &pool_address);
        
        pool.withdraw_fund(&owner);

        set_pool_fund_to_zero(&env,pool_id);

    }

    pub fn archive_pool(env:Env,pool_id:String,owner:Address){

        owner.require_auth();

        let pool_address = get_pool_address(&env, pool_id.clone());

        let pool = pool_contract::Client::new(&env, &pool_address);
        
        pool.archive_pool(&owner);

        set_pool_archive(&env,pool_id,true);
    }

    pub fn unarchive_pool(env:Env,pool_id:String,owner:Address){

        owner.require_auth();

        let pool_address = get_pool_address(&env, pool_id.clone());

        let pool = pool_contract::Client::new(&env, &pool_address);
        
        pool.unarchive_pool(&owner);

        set_pool_archive(&env,pool_id,false);
    }

    pub fn get_balance(env:Env,pool_id:String,user:Address)->(i128,i128){
        
        let pool_address = get_pool_address(&env, pool_id);

        let pool = pool_contract::Client::new(&env, &pool_address);

        pool.balance(&user)
    }


    pub fn get_pool(env:Env)->Map<String, Pool>{

        get_all_pool(&env)
    }

    pub fn get_buy_price(env:Env,pool_id:String)->i128{

        let pool_address = get_pool_address(&env, pool_id);

        let pool = pool_contract::Client::new(&env, &pool_address);

        pool.simulate_buy_price()
    }

    pub fn get_sell_price(env:Env,pool_id:String)->(bool,i128){

        let pool_address = get_pool_address(&env, pool_id);

        let pool = pool_contract::Client::new(&env, &pool_address);

        pool.simulate_sell_price()
    }

         
}

