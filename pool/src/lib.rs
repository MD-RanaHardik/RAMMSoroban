#![no_std]
mod lptoken;
use core::{panic};

use soroban_sdk::{contract, contractimpl, contractmeta, token::{self, TokenClient}, Address, BytesN, ConversionError, Env, Map, String, TryFromVal, Val};

use lptoken::create_contract;

#[derive(Debug,Clone, Copy)]
#[repr(u32)]
pub enum DataKey {
    IsInitialized = 0,
    USDCAddress = 1,
    XYZAddress = 2,
    USDCBalance = 3,
    CollectedFees = 4,
    FeeTo = 5,
    Admin = 6,
}

impl TryFromVal<Env, DataKey> for Val {
    type Error = ConversionError;

    fn try_from_val(_env: &Env, v: &DataKey) -> Result<Self, Self::Error> {
        Ok((*v as u32).into())
    }
}

// Metadata that is added on to the WASM custom section
contractmeta!(
    key = "Description",
    val = "Simple Swap"
);

fn is_initialized(env:&Env){

    let key = DataKey::IsInitialized;

    let is_init = env.storage().instance().has(&key);

    if is_init{
        panic!("Already initialized")
    }else{
        init(env, &key)
    }

}

fn init(env:&Env,key:&DataKey){
    env.storage().instance().set(key, &true);
}

fn set_admin(env:&Env,admin:Address){

    let key = DataKey::Admin;

    let is_available = env.storage().instance().has(&key);

    if is_available {
        panic!("Admin already set")
    }else{
        env.storage().instance().set(&key, &admin);
    }
}

fn set_fee_to_init(env:&Env,feeto:Address){

    let key = DataKey::FeeTo;

    let is_available = env.storage().instance().has(&key);

    if is_available {
        panic!("FeeTo already set")
    }else{
        env.storage().instance().set(&key, &feeto);
    }
}

fn set_xyz_address(env:&Env,xyz_address:Address){

    let key = DataKey::XYZAddress;

    let is_available = env.storage().instance().has(&key);

    if is_available {
        panic!("XYZ address already set")
    }else{
        env.storage().instance().set(&key, &xyz_address);
    }

}


fn set_usdc_address(env:&Env,usdc_address:Address){

    let key = DataKey::USDCAddress;

    let is_available = env.storage().instance().has(&key);

    if is_available {
        panic!("USDC address already set")
    }else{
        env.storage().instance().set(&key, &usdc_address);
    }

}

fn get_usdc_token(env:&Env)-> TokenClient{
    let key = DataKey::USDCAddress;

    let token_address:Address = env.storage().instance().get(&key).unwrap();

    token::Client::new(env, &token_address)
}

fn mint_xyz(env:&Env,to:&Address,amount:&i128){

    let key = DataKey::XYZAddress;

    let token_address:Address = env.storage().instance().get(&key).unwrap();

    lptoken::Client::new(env, &token_address).mint(&to, &amount);
}

fn burn_xyz(env:&Env,from:&Address,amount:&i128){

    let key = DataKey::XYZAddress;

    let token_address:Address = env.storage().instance().get(&key).unwrap();

    let xyz_token = lptoken::Client::new(env, &token_address);

    let xyz_balance = xyz_token.balance(from);

    assert!(&xyz_balance >= amount,"Insufficient Balance");

    xyz_token.burn(from, amount);
}

fn get_collected_fee(env:&Env)->i128{

    let key = DataKey::CollectedFees;

    let fees:i128 = env.storage().instance().get(&key).unwrap_or(0);

    fees
}

fn update_udsc_balance(env:&Env,amount:&i128){
    
    let key = DataKey::USDCBalance;

    let reserve:i128 = env.storage().instance().get(&key).unwrap_or(0);

    env.storage().instance().set(&key, &(reserve + amount));
}

fn update_collected_fees(env:&Env,amount:&i128){
    
    let key = DataKey::CollectedFees;

    let collected_fees:i128 = env.storage().instance().get(&key).unwrap_or(0);

    env.storage().instance().set(&key, &(collected_fees + amount));
}


#[contract]
pub struct Pool;

#[contractimpl]
impl Pool {


    pub fn initialize(env:Env,admin:Address,feeto:Address,token_wasm_hash: BytesN<32>) {
        
        is_initialized(&env);

        set_admin(&env, admin);

        set_fee_to_init(&env, feeto);

        let xyz_token_contract = create_contract(&env, token_wasm_hash);
        
        lptoken::Client::new(&env, &xyz_token_contract.clone()).initialize(
            &env.current_contract_address(),
            &8u32,
            &String::from_str(&env, "XYZ Token"),
            &String::from_str(&env,"XYZ"),
        );

        set_xyz_address(&env, xyz_token_contract);
        set_usdc_address(&env, Address::from_string(&String::from_str(&env, "CCU4UCIHCQRU3YY4465SWCQYSQYC5STUAPGKR3MDSWMESVSGOT3IPZOG")));
    }

    pub fn deposit(env:Env,from:Address,amount:i128)->(i128,i128){

        from.require_auth();

        let usdc = get_usdc_token(&env);
        
        let balace_of = usdc.balance(&from);

        assert!(balace_of >= amount, "Insufficient balance");

        usdc.transfer(&from, &env.current_contract_address(), &amount);

        mint_xyz(&env, &from, &amount);

        update_udsc_balance(&env,&amount);

        (amount,amount)
    }

    pub fn withdraw(env:Env,from:Address,amount:i128)->(i128,i128){

        from.require_auth();

        let usdc = get_usdc_token(&env);

        let collected_fees = get_collected_fee(&env);
        
        let balace_of = usdc.balance(&env.current_contract_address()) - collected_fees;

        let amount_to_return = amount - (amount * 1 / 100);

        assert!(balace_of >= amount_to_return, "Insufficient balance");

        usdc.transfer(&env.current_contract_address(), &from, &amount_to_return);
        
        burn_xyz(&env,&from,&amount);

        update_udsc_balance(&env,&-amount);
        update_collected_fees(&env,&(&(amount * 1 ) / 100));

        (amount,amount_to_return)
    }

    pub fn get_collected_fees(env:Env)->i128{
        let key = DataKey::CollectedFees;
        env.storage().instance().get(&key).unwrap_or(0)
    }

    pub fn collect_fees(env:Env){
        
        let key = DataKey::FeeTo;

        let feeto:Address = env.storage().instance().get(&key).unwrap();

        let collected_fees:i128 = env.storage().instance().get(&DataKey::CollectedFees).unwrap();
        
        assert!(collected_fees > 0,"Insufficient Fees");

        let usdc = get_usdc_token(&env);

        usdc.transfer(&env.current_contract_address(), &feeto, &collected_fees);

        update_collected_fees(&env, &-collected_fees);
    }

    pub fn get_balance(env:Env,user:Address)->(i128,i128){
       
        let usdc = DataKey::USDCAddress;

        let usdc_address:Address = env.storage().instance().get(&usdc).unwrap();

        let usdc_balance = token::Client::new(&env, &usdc_address).balance(&user);

        let xyz = DataKey::XYZAddress;

        let xyz_address:Address = env.storage().instance().get(&xyz).unwrap();

        let xyz_balance = token::Client::new(&env, &xyz_address).balance(&user);

        (usdc_balance,xyz_balance)
    }


}


