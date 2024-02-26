#![no_std]
mod test;
mod lptoken;
mod storage;
use core::{panic};

use soroban_sdk::{contract, contractimpl, contractmeta, token::{self, TokenClient}, Address, BytesN, ConversionError, Env, Map, String, TryFromVal, Val};

use lptoken::create_contract;
use storage::{check_switch, get_in_secondary_mode, get_price_primary, get_price_secondary, get_pvt_available_secondary, get_pvt_qty_max_primary, get_pvt_qty_max_secondary, get_token_token, get_usdc_token, get_x, is_initialized, set_a_primary_midpoint_initial_and_max, set_a_secondary_midpoint_initial_and_max, set_b_primary_half_max_qty, set_b_secondary_half_max_qty, set_c_primary_steepness, set_c_secondary_steepness, set_in_secondary_mode, set_owner, set_p_doubleprime, set_p_prime, set_pool_archive, set_pool_name, set_pvt_available_secondary, set_pvt_price_initial_primary, set_pvt_price_max_primary, set_pvt_price_max_secondary, set_pvt_qty_max_primary, set_pvt_qty_max_secondary, set_pvt_running_total_bought, set_pvt_running_total_sold, set_soldout_hits, set_token_address, set_treasury, set_usdc_address, set_x};





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

#[derive(Debug,Clone, Copy)]
#[repr(u32)]
pub enum PoolKey {
    IsInitialized=0,
    PoolName=1,
    Owner=2,
    Archived = 3,
    Treasury=4,
    InSecondaryMode=5,
    PvtQtyMaxPrimary = 6,
    PvtQtyMaxSecondary=7,
    PvtAvailableSecondary=8,
    PvtRunningTotalBought = 9,
    PvtRunningTotalSold = 10,
    PvtPriceInitialPrimary=11,
    PvtPriceMaxPrimary =12,
    PvtPriceMaxSecondary=13,
    APrimaryMidpointInitialAndMax = 14,
    BPrimaryHalfMaxQty = 15,
    CPrimarySteepness = 16,
    ASecondaryMidpointInitialAndMax = 17,
    BSecondaryHalfMaxQty = 18,
    CSecondarySteepness =19,
    PPrime = 20,
    PDoublePrime = 21,
    SoldOutHits = 22,
    X=23,
    TokenAddress=24,
    USDCAddress=25

}


impl TryFromVal<Env, PoolKey> for Val {
    type Error = ConversionError;

    fn try_from_val(_env: &Env, v: &PoolKey) -> Result<Self, Self::Error> {
        Ok((*v as u32).into())
    }
}




// Metadata that is added on to the WASM custom section
contractmeta!(
    key = "Description",
    val = "Simple Swap"
);

// fn is_initialized(env:&Env){

//     let key = DataKey::IsInitialized;

//     let is_init = env.storage().instance().has(&key);

//     if is_init{
//         panic!("Already initialized")
//     }else{
//         init(env, &key)
//     }

// }

// fn init(env:&Env,key:&DataKey){
//     env.storage().instance().set(key, &true);
// }

// fn set_admin(env:&Env,admin:Address){

//     let key = DataKey::Admin;

//     let is_available = env.storage().instance().has(&key);

//     if is_available {
//         panic!("Admin already set")
//     }else{
//         env.storage().instance().set(&key, &admin);
//     }
// }

// fn set_fee_to_init(env:&Env,feeto:Address){

//     let key = DataKey::FeeTo;

//     let is_available = env.storage().instance().has(&key);

//     if is_available {
//         panic!("FeeTo already set")
//     }else{
//         env.storage().instance().set(&key, &feeto);
//     }
// }

// fn set_xyz_address(env:&Env,xyz_address:Address){

//     let key = DataKey::XYZAddress;

//     let is_available = env.storage().instance().has(&key);

//     if is_available {
//         panic!("XYZ address already set")
//     }else{
//         env.storage().instance().set(&key, &xyz_address);
//     }

// }


// fn set_usdc_address(env:&Env,usdc_address:Address){

//     let key = DataKey::USDCAddress;

//     let is_available = env.storage().instance().has(&key);

//     if is_available {
//         panic!("USDC address already set")
//     }else{
//         env.storage().instance().set(&key, &usdc_address);
//     }

// }

// fn get_usdc_token(env:&Env)-> TokenClient{
//     let key = DataKey::USDCAddress;

//     let token_address:Address = env.storage().instance().get(&key).unwrap();

//     token::Client::new(env, &token_address)
// }

// fn mint_xyz(env:&Env,to:&Address,amount:&i128){

//     let key = DataKey::XYZAddress;

//     let token_address:Address = env.storage().instance().get(&key).unwrap();

//     lptoken::Client::new(env, &token_address).mint(&to, &amount);
// }

// fn burn_xyz(env:&Env,from:&Address,amount:&i128){

//     let key = DataKey::XYZAddress;

//     let token_address:Address = env.storage().instance().get(&key).unwrap();

//     let xyz_token = lptoken::Client::new(env, &token_address);

//     let xyz_balance = xyz_token.balance(from);

//     assert!(&xyz_balance >= amount,"Insufficient Balance");

//     xyz_token.burn(from, amount);
// }

// fn get_collected_fee(env:&Env)->i128{

//     let key = DataKey::CollectedFees;

//     let fees:i128 = env.storage().instance().get(&key).unwrap_or(0);

//     fees
// }

// fn update_udsc_balance(env:&Env,amount:&i128){
    
//     let key = DataKey::USDCBalance;

//     let reserve:i128 = env.storage().instance().get(&key).unwrap_or(0);

//     env.storage().instance().set(&key, &(reserve + amount));
// }

// fn update_collected_fees(env:&Env,amount:&i128){
    
//     let key = DataKey::CollectedFees;

//     let collected_fees:i128 = env.storage().instance().get(&key).unwrap_or(0);

//     env.storage().instance().set(&key, &(collected_fees + amount));
// }

// +++++++++++++++++++++++++++++++++++++++++++++++++++++++++







#[contract]
pub struct Pool;

#[contractimpl]
impl Pool {
   
    pub fn init(env:Env,token_wasm_hash:BytesN<32>,owner:Address,pool_name:String,pvt_qty_max_primary:i128,pvt_qty_max_secondary:i128,pvt_price_max_primary:i128,pvt_price_max_secondary:i128,pvt_price_initial_primary:i128,pvt_available_secondary:i128,steepness:u32){

        is_initialized(&env);

        set_pool_name(&env, pool_name);

        set_x(&env, 0);

        set_pool_archive(&env, false);

        set_owner(&env, owner);

        set_treasury(&env, 0);

        set_in_secondary_mode(&env, false);

        set_pvt_qty_max_primary(&env, pvt_qty_max_primary);

        set_pvt_qty_max_secondary(&env, pvt_qty_max_secondary);

        set_pvt_available_secondary(&env, pvt_available_secondary);

        set_pvt_running_total_bought(&env, 0);

        set_pvt_running_total_sold(&env, 0);

        set_pvt_price_initial_primary(&env, pvt_price_initial_primary);

        set_pvt_price_max_primary(&env, pvt_price_max_primary);

        set_pvt_price_max_secondary(&env, pvt_price_max_secondary);

        set_a_primary_midpoint_initial_and_max(&env);

        set_b_primary_half_max_qty(&env);

        set_c_primary_steepness(&env, steepness);

        set_a_secondary_midpoint_initial_and_max(&env);

        set_b_secondary_half_max_qty(&env);

        set_c_secondary_steepness(&env, steepness);

        set_p_prime(&env);

        set_p_doubleprime(&env);

        set_soldout_hits(&env, 0);

        let xyz_token_contract = create_contract(&env, token_wasm_hash.clone(),String::from_str(&env, "XYZ"));
        
        lptoken::Client::new(&env, &xyz_token_contract.clone()).initialize(
                &env.current_contract_address(),
                &8u32,
                &String::from_str(&env, "XYZ Token"),
                &String::from_str(&env,"XYZ"),
            );

        set_token_address(&env, xyz_token_contract);

        let usdc_token_contract = create_contract(&env, token_wasm_hash,String::from_str(&env, "USDC"));
        
        lptoken::Client::new(&env, &usdc_token_contract.clone()).initialize(
                &env.current_contract_address(),
                &8u32,
                &String::from_str(&env, "USDC Token"),
                &String::from_str(&env,"USDC"),
            );

        set_usdc_address(&env, usdc_token_contract);


    }
    
    pub fn expand(env:Env,amount:i128){

        let pvt_qty_max_secondary = get_pvt_qty_max_secondary(&env);
        let pvt_available_secondary = get_pvt_available_secondary(&env);

        let left = pvt_qty_max_secondary.checked_add(amount).expect("Overflow");

        if(left <= pvt_available_secondary){

            set_pvt_qty_max_secondary(&env, amount);
        }

    }

    pub fn buy(env:Env,user:Address){

        user.require_auth();
        
        let in_secondary_mode = get_in_secondary_mode(&env);
        let x = get_x(&env);
        let pvt_qty_max_primary = get_pvt_qty_max_primary(&env);
        let pvt_qty_max_secondary = get_pvt_qty_max_secondary(&env);

        let mut price:i128 = 0;

        if in_secondary_mode { 

            if x > pvt_qty_max_primary.checked_add(pvt_qty_max_secondary).expect("Overflow") {
                set_soldout_hits(&env, 1);
            }

            price = get_price_primary(&env, x.checked_add(1).expect("Overflow"));

        }else{
            price = get_price_secondary(&env, x.checked_add(1).expect("Overflow"));
        }


        //Transfer usdc to contract address
        let usdctoken = get_usdc_token(&env);

        let usdc_balance = usdctoken.balance(&user);

        assert!(usdc_balance >= price ,"Insufficient balance");

        usdctoken.transfer(&user, &env.current_contract_address(), &price);

        set_treasury(&env, price);

        set_pvt_running_total_bought(&env, 1);

        //xyz token
        let xyztoken = get_token_token(&env);

        xyztoken.mint(&user, &1);
    
        set_x(&env, 1);

        if !in_secondary_mode
        {
            check_switch(&env);
        }

    
    }

    pub fn sell(env:Env){
        
    }

    pub fn mint_usdc_for_test(env:Env,user:Address){
        let usdc = get_usdc_token(&env);

        usdc.mint(&user, &1000);
    }

   

    




    // pub fn initialize(env:Env,admin:Address,feeto:Address,token_wasm_hash: BytesN<32>) {
        
    //     is_initialized(&env);

    //     set_admin(&env, admin);

    //     set_fee_to_init(&env, feeto);

    //     let xyz_token_contract = create_contract(&env, token_wasm_hash);
        
    //     lptoken::Client::new(&env, &xyz_token_contract.clone()).initialize(
    //         &env.current_contract_address(),
    //         &8u32,
    //         &String::from_str(&env, "XYZ Token"),
    //         &String::from_str(&env,"XYZ"),
    //     );

    //     set_xyz_address(&env, xyz_token_contract);
    //     set_usdc_address(&env, Address::from_string(&String::from_str(&env, "CCU4UCIHCQRU3YY4465SWCQYSQYC5STUAPGKR3MDSWMESVSGOT3IPZOG")));
    // }

    // pub fn deposit(env:Env,from:Address,amount:i128)->(i128,i128){

    //     from.require_auth();

    //     let usdc = get_usdc_token(&env);
        
    //     let balace_of = usdc.balance(&from);

    //     assert!(balace_of >= amount, "Insufficient balance");

    //     usdc.transfer(&from, &env.current_contract_address(), &amount);

    //     mint_xyz(&env, &from, &amount);

    //     update_udsc_balance(&env,&amount);

    //     (amount,amount)
    // }

    // pub fn withdraw(env:Env,from:Address,amount:i128)->(i128,i128){

    //     from.require_auth();

    //     let usdc = get_usdc_token(&env);

    //     let collected_fees = get_collected_fee(&env);
        
    //     let balace_of = usdc.balance(&env.current_contract_address()) - collected_fees;

    //     let amount_to_return = amount - (amount * 1 / 100);

    //     assert!(balace_of >= amount_to_return, "Insufficient balance");

    //     usdc.transfer(&env.current_contract_address(), &from, &amount_to_return);
        
    //     burn_xyz(&env,&from,&amount);

    //     update_udsc_balance(&env,&-amount);
    //     update_collected_fees(&env,&(&(amount * 1 ) / 100));

    //     (amount,amount_to_return)
    // }

    // pub fn get_collected_fees(env:Env)->i128{
    //     let key = DataKey::CollectedFees;
    //     env.storage().instance().get(&key).unwrap_or(0)
    // }

    // pub fn collect_fees(env:Env){
        
    //     let key = DataKey::FeeTo;

    //     let feeto:Address = env.storage().instance().get(&key).unwrap();

    //     let collected_fees:i128 = env.storage().instance().get(&DataKey::CollectedFees).unwrap();
        
    //     assert!(collected_fees > 0,"Insufficient Fees");

    //     let usdc = get_usdc_token(&env);

    //     usdc.transfer(&env.current_contract_address(), &feeto, &collected_fees);

    //     update_collected_fees(&env, &-collected_fees);
    // }

    // pub fn get_balance(env:Env,user:Address)->(i128,i128){
       
    //     let usdc = DataKey::USDCAddress;

    //     let usdc_address:Address = env.storage().instance().get(&usdc).unwrap();

    //     let usdc_balance = token::Client::new(&env, &usdc_address).balance(&user);

    //     let xyz = DataKey::XYZAddress;

    //     let xyz_address:Address = env.storage().instance().get(&xyz).unwrap();

    //     let xyz_balance = token::Client::new(&env, &xyz_address).balance(&user);

    //     (usdc_balance,xyz_balance)
    // }


}


