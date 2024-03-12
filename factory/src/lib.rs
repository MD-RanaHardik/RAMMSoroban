#![no_std]
mod pool;
mod test;
extern  crate alloc;

use alloc::string::{String as STR, ToString};
use pool::create_pool_contract;

use soroban_sdk::{bytesn, contract, contractimpl, contractmeta, contracttype, Address, ConversionError, Env, Map, String, TryFromVal, Val};

//Imported pool contract so that we can use this for cross contract call 
mod pool_contract {
    soroban_sdk::contractimport!(
        //path of pool contract wasm file
        file = ".././pool/target/wasm32-unknown-unknown/release/pool.wasm"
    );
}

//Q9 variable used for scale unscaled 
const Q9:i128 = (10 as i128).pow(9);

//Storage where we store all pool data in mapping
#[derive(Debug,Clone, Copy)]
#[repr(u32)]
pub enum DataKey {
    Pools=1
}

//Storage which used for storage all required value particular pool
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
    pvt_available_secondary:i128,
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

//Function for get new pool id based on current pool index
fn get_next_pool_id(env:&Env,pool_index:u32)-> String{
    //Created new alloc string and append pool name and index of current pool into string
    let mut str = STR::new();

    str.push_str("RAMM Pool - ");
    str.push_str(&pool_index.to_string().as_str());
    
    //Converted string to str so we can convert that str to Soroban String 
    let str_name = &str.as_str();

    //Converted str to String and return new pool id
    String::from_str(&env, str_name)
}

//Function use for get pool address from pool id
fn get_pool_address(env:&Env,pool_id:String)->Address{

    let key = DataKey::Pools;

    //Fetched all available pools
    let available_pools:Map<String, Pool> = env.storage().persistent().get(&key).unwrap_or(Map::new(&env));

    //Checking whether pool available or not based on given pool id
    let is_available = available_pools.get(pool_id.clone()).is_some();

    //If pool not exist it will panic
    assert!(is_available,"Pool Already Exist");

    //If pool available the returns pool address
    available_pools.get(pool_id).unwrap().pool_address
}

//Set pool data to pool storage when anyone create new pool
fn set_pool(env:&Env,pool_name:&String,owner:&Address,pool_address:Address,pvt_qty_max_primary:i128,pvt_price_max_primary:i128,pvt_price_max_secondary:i128,pvt_price_initial_primary:i128,pvt_available_secondary:i128,c_primary_steepness:u32){

    let key = DataKey::Pools;

    //Fetched all available pools
    let mut pools:Map<String, Pool> = env.storage().persistent().get(&key).unwrap_or(Map::new(&env));
    
    //Generate new pool id based on current pool index
    let pool_id = get_next_pool_id(env,pools.len());

    //Set all initial pool data to Pool storage structure
    let new_pool = Pool{
        owner:owner.clone(),
        pool_address:pool_address,
        pool_name:pool_name.clone(),
        pool_id:pool_id.clone(),
        archived:false,
        treasury:0,
        x:0,
        pvt_qty_max_primary:pvt_qty_max_primary,
        pvt_qty_max_secondary:pvt_qty_max_primary,
        pvt_price_max_primary:pvt_price_max_primary,
        pvt_price_max_secondary:pvt_price_max_secondary,
        pvt_price_initial_primary:pvt_price_initial_primary,
        pvt_available_secondary:pvt_available_secondary,
        c_primary_steepness:c_primary_steepness,
        pool_status:0
    };

    //Set new pool into pools storage based on new generated pool id
    pools.set(pool_id, new_pool);

    //Update new added data to Pools storage
    env.storage().persistent().set(&key, &pools);
}


//Returns all available pool data
fn get_all_pool(env:&Env)->Map<String, Pool>{

    let key = DataKey::Pools;

    //Fetch all pool data
    let pools:Map<String, Pool> = env.storage().persistent().get(&key).unwrap_or(Map::new(&env));

    //Return pool data
    pools
}

//Function help to set new x and treasury whenever anyone buy or sell 
fn set_pool_x_and_treasury(env:&Env,pool_id:String,x:i128,treasury:i128){

    let key = DataKey::Pools;

    //Fetched all available pools
    let mut available_pools:Map<String, Pool> = env.storage().persistent().get(&key).unwrap_or(Map::new(&env));

    //Checking whether pool available or not based on given pool id
    let is_available = available_pools.get(pool_id.clone()).is_some();

    //If pool not exist it will panic
    assert!(is_available,"Pool Not Exist");

    //Get specific pool data
    let mut pool_data = available_pools.get(pool_id.clone()).unwrap();

    //Updated x value with new one
    pool_data.x = x * Q9;
    
    //Updated treasury value with new one
    pool_data.treasury = treasury;
    
    //Set updated pool to pools storage 
    available_pools.set(pool_id, pool_data);
    
    //Update pools storage
    env.storage().persistent().set(&key,&available_pools);
}


//Function help to update pvt_qty_max_secondary whenever owner expand pool
fn set_pool_pvt_qty_max_secondary(env:&Env,pool_id:String,pvt_qty_max_secondary:i128){

    let key = DataKey::Pools;

    //Fetched all available pools
    let mut available_pools:Map<String, Pool> = env.storage().persistent().get(&key).unwrap_or(Map::new(&env));

    //Checking whether pool available or not based on given pool id
    let is_available = available_pools.get(pool_id.clone()).is_some();

    //If pool not exist it will panic
    assert!(is_available,"Pool Not Exist");

    //Get specific pool data
    let mut pool_data = available_pools.get(pool_id.clone()).unwrap();

    //Updated pvt_qty_max_secondary with new pvt_qty_max_secondary value
    pool_data.pvt_qty_max_secondary = pvt_qty_max_secondary;

    //Set updated pool to pools storage
    available_pools.set(pool_id, pool_data);

    //Update pools storage
    env.storage().persistent().set(&key,&available_pools);
}


//Function help to update pool_archive status whenever owner archive and unarchive pool
fn set_pool_archive(env:&Env,pool_id:String,archive:bool){

    let key = DataKey::Pools;

    //Fetched all available pools
    let mut available_pools:Map<String, Pool> = env.storage().persistent().get(&key).unwrap_or(Map::new(&env));

    //Checking whether pool available or not based on given pool id
    let is_available = available_pools.get(pool_id.clone()).is_some();

    //If pool not exist it will panic
    assert!(is_available,"Pool Not Exist");

    //Get specific pool data
    let mut pool_data = available_pools.get(pool_id.clone()).unwrap();

    //Updated archive status with new one
    pool_data.archived = archive;

    //Set updated pool to pools storage
    available_pools.set(pool_id, pool_data);

    //Update pools storage
    env.storage().persistent().set(&key,&available_pools);
}

//Function help to update pool state status whenever owner start and stop pool
fn set_pool_status(env:&Env,pool_id:String,status:u32){

    let key = DataKey::Pools;

    //Fetched all available pools
    let mut available_pools:Map<String, Pool> = env.storage().persistent().get(&key).unwrap_or(Map::new(&env));

    //Checking whether pool available or not based on given pool id
    let is_available = available_pools.get(pool_id.clone()).is_some();

    //If pool not exist it will panic
    assert!(is_available,"Pool Not Exist");

    //Get specific pool data
    let mut pool_data = available_pools.get(pool_id.clone()).unwrap();

    //Updated pool status with new one
    pool_data.pool_status = status;

    //Set updated pool to pools storage
    available_pools.set(pool_id, pool_data);

    //Update pools storage
    env.storage().persistent().set(&key,&available_pools);
}

//Function help to update treasury value to zero whenever owner close pool and withdraw all funds
fn set_pool_fund_to_zero(env:&Env,pool_id:String){

    let key = DataKey::Pools;

    //Fetched all available pools
    let mut available_pools:Map<String, Pool> = env.storage().persistent().get(&key).unwrap_or(Map::new(&env));

    //Checking whether pool available or not based on given pool id
    let is_available = available_pools.get(pool_id.clone()).is_some();

    //If pool not exist it will panic
    assert!(is_available,"Pool Not Exist");

    //Get specific pool data
    let mut pool_data = available_pools.get(pool_id.clone()).unwrap();
    
    //Updated treasury to zero
    pool_data.treasury = 0;

    //Set updated pool to pools storage
    available_pools.set(pool_id, pool_data);

    //Update pools storage
    env.storage().persistent().set(&key,&available_pools);
}








#[contract]
pub struct Factory;

#[contractimpl]
impl Factory {

    //Function helps to create new pool
    pub fn create_pool(env:Env,owner:Address,pool_name:String,pvt_qty_max_primary:i128,pvt_price_max_primary:i128,pvt_price_initial_primary:i128,pvt_available_secondary:i128,steepness:u32)->Address{

        //Converted installed pool wasm hash into BytesN<32>
        let pool_wasm_hash = bytesn!(&env,0x0d38ef389299626c6e824b360e1d597762bfa5b997ac8887a75443ce9592498d);
        
        //Converted installed token wasm hash into BytesN<32>
        let token_wasm_hash = bytesn!(&env,0xc04dc2300124d5869a2dbbe81600ba0008f609e75ce254aca065c43d3a4abbe5);

        //Created new pool by providing installed pool wasm hash
        let created_pool = create_pool_contract(&env, pool_wasm_hash, pool_name.clone());
        
        //Created pool client from newly deployed pool contract address which help us to interact with pool
        let p = pool_contract::Client::new(&env, &created_pool);

        let pvt_price_max_secondary = (2 * pvt_price_max_primary) - pvt_price_initial_primary;

        //Initialized newly created pool
        p.init(&token_wasm_hash,&owner,&pool_name,&pvt_qty_max_primary,&pvt_qty_max_primary,&pvt_price_max_primary,&pvt_price_max_secondary,&pvt_price_initial_primary,&pvt_available_secondary,&steepness);
        
        //Created new pool into pool storage and set initial value for pool 
        set_pool(&env, &pool_name, &owner, created_pool.clone(),pvt_qty_max_primary,pvt_price_max_primary,pvt_price_max_secondary,pvt_price_initial_primary,pvt_available_secondary,steepness);

        //Returned new pool address
        created_pool
    }

    //Function helps to start pool
    pub fn start(env:Env,pool_id:String,owner:Address){

        //Check owner authenticity
        owner.require_auth();

        //Get pool address from provided pool id
        let pool_address = get_pool_address(&env, pool_id.clone());

        //Created pool client which help us to interact with pool contract 
        let pool = pool_contract::Client::new(&env, &pool_address);

        //Started pool by doing cross contract call
        pool.start(&owner);

        //Updated pool status to 1 (1 = Start)
        set_pool_status(&env,pool_id,1);
    }

    //Function helps to stop pool
    pub fn stop(env:Env,pool_id:String,owner:Address){

        //Check owner authenticity
        owner.require_auth();

        //Get pool address from provided pool id
        let pool_address = get_pool_address(&env, pool_id.clone());

        //Created pool client which help us to interact with pool contract 
        let pool = pool_contract::Client::new(&env, &pool_address);

        //Stop pool by doing cross contract call
        pool.stop(&owner);

        //Updated pool status to 2 (2 = Stop)
        set_pool_status(&env,pool_id,2);
    }

    //Function helps to expand pool qty
    pub fn expand(env:Env,pool_id:String,owner:Address,amount:i128){

        //Check owner authenticity
        owner.require_auth();

        //Get pool address from provided pool id
        let pool_address = get_pool_address(&env, pool_id.clone());

        //Created pool client which help us to interact with pool contract 
        let pool = pool_contract::Client::new(&env, &pool_address);

        //Expanded pool by doing cross contract call which return is_expand if pool successfully expanded the pool qty and also return new pvt_qty_max_secondary value
        let (is_expand,pvt_qty_max_secondary) = pool.expand(&owner,&amount);

        //If expanded if true then we will update pvt_qty_max_secondary into factory contract 
        if is_expand {
            set_pool_pvt_qty_max_secondary(&env, pool_id, pvt_qty_max_secondary);
        }

    }

    //Function helps to buy PVT tokens in exchange of USDC
    pub fn buy(env:Env,pool_id:String,from:Address){
        
        //Check user authenticity
        from.require_auth();

        //Get pool address from provided pool id
        let pool_address = get_pool_address(&env, pool_id.clone());

        //Created pool client which help us to interact with pool contract
        let pool = pool_contract::Client::new(&env, &pool_address);

        //Buy pvt token from pool by doing cross contract call which return current x and treasury
        let (x,treasury) = pool.buy(&from);

        //Update current x and treasury to factory contract
        set_pool_x_and_treasury(&env,pool_id,x,treasury);
    }

    //Function helps to sell PVT tokens in exchange of USDC
    pub fn sell(env:Env,pool_id:String,from:Address){

        //Check user authenticity
        from.require_auth();

        //Get pool address from provided pool id
        let pool_address = get_pool_address(&env, pool_id.clone());

        //Created pool client which help us to interact with pool contract
        let pool = pool_contract::Client::new(&env, &pool_address);
        
        //Sell pvt token by doing cross contract call which return in_secondary_mode, current x and treasury
        let (in_secondary_mode,x,treasury) = pool.sell(&from);

        //Only update x and treasury if pool in secondary mode
        if in_secondary_mode {
            set_pool_x_and_treasury(&env,pool_id,x,treasury);
        }

    }

    //Function helps to withdraw all USDC token from pool
    pub fn withdraw_fund(env:Env,pool_id:String,owner:Address){

        //Check owner authenticity
        owner.require_auth();

        //Get pool address from provided pool id
        let pool_address = get_pool_address(&env, pool_id.clone());
        
        //Created pool client which help us to interact with pool contract
        let pool = pool_contract::Client::new(&env, &pool_address);
        
        //Withdraw all USDC token by doing cross contract call
        pool.withdraw_fund(&owner);

        //Updated treasury to zero
        set_pool_fund_to_zero(&env,pool_id);

    }

    //Function helps to archive pool
    pub fn archive_pool(env:Env,pool_id:String,owner:Address){

        //Check owner authenticity
        owner.require_auth();

        //Get pool address from provided pool id
        let pool_address = get_pool_address(&env, pool_id.clone());

        //Created pool client which help us to interact with pool contract
        let pool = pool_contract::Client::new(&env, &pool_address);
        
        //Archive pool by doing cross contract call
        pool.archive_pool(&owner);

        //Update pool archive status to true
        set_pool_archive(&env,pool_id,true);
    }

    //Function helps to unarchive pool
    pub fn unarchive_pool(env:Env,pool_id:String,owner:Address){

        //Check owner authenticity
        owner.require_auth();

        //Get pool address from provided pool id
        let pool_address = get_pool_address(&env, pool_id.clone());

        //Created pool client which help us to interact with pool contract
        let pool = pool_contract::Client::new(&env, &pool_address);
        
        //Unarchive pool by doing cross contract call
        pool.unarchive_pool(&owner);

        //Update pool archive status to false
        set_pool_archive(&env,pool_id,false);
    }

    //Function helps to get balance of USDC and PVT token of given pool id 
    pub fn get_balance(env:Env,pool_id:String,user:Address)->(i128,i128){
        
        //Get pool address from provided pool id
        let pool_address = get_pool_address(&env, pool_id);

        //Created pool client which help us to interact with pool contract
        let pool = pool_contract::Client::new(&env, &pool_address);

        //Get balance by doing cross contract call
        pool.balance(&user)
    }


    //Function helps to get all pool data 
    pub fn get_pool(env:Env)->Map<String, Pool>{
        get_all_pool(&env)
    }

    //Function gives us price of USDC in exchange of 1 PVT token
    pub fn get_buy_price(env:Env,pool_id:String)->i128{

        //Get pool address from provided pool id
        let pool_address = get_pool_address(&env, pool_id);

        //Created pool client which help us to interact with pool contract
        let pool = pool_contract::Client::new(&env, &pool_address);

        //Cross contract call which return USDC price
        pool.simulate_buy_price()
    }

    //Function gives us price of USDC in exchange of 1 PVT token
    pub fn get_sell_price(env:Env,pool_id:String)->(bool,i128){

        //Get pool address from provided pool id
        let pool_address = get_pool_address(&env, pool_id);

        //Created pool client which help us to interact with pool contract
        let pool = pool_contract::Client::new(&env, &pool_address);

        //Cross contract call which return USDC price
        pool.simulate_sell_price()
    }

         
}

