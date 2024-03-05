use core::ops::Add;

use num_integer::sqrt;
use soroban_fixed_point_math::FixedPoint;
use soroban_sdk::{log, token::{self, TokenClient}, Address, Env, IntoVal, String};

use crate::{lptoken::{self, Client}, PoolKey};

pub(crate) const DAY_IN_LEDGERS: u32 = 17280;
pub(crate) const INSTANCE_BUMP_AMOUNT: u32 = 7 * DAY_IN_LEDGERS;
pub(crate) const INSTANCE_LIFETIME_THRESHOLD: u32 = INSTANCE_BUMP_AMOUNT - DAY_IN_LEDGERS;

pub(crate) const PERSISTENT_STORAGE_BUMP_AMOUNT: u32 = 30 * DAY_IN_LEDGERS;
pub(crate) const PERSISTENT_STORAGE_LIFETIME_THRESHOLD: u32 = PERSISTENT_STORAGE_BUMP_AMOUNT - DAY_IN_LEDGERS;

const Q9:i128 = (10 as i128).pow(9);


pub(crate) fn set_pool_init_status(env:&Env){

    let key = PoolKey::PoolStatus;

    env.storage().persistent().set(&key, &0);

}

pub(crate) fn start_pool(env:&Env){

    let key = PoolKey::PoolStatus;

    let status = get_pool_status(env);

    if status == 0 {
        env.storage().persistent().set(&key, &1); 
    }

    if status == 2 {
        panic!("You cannot start pool once stop")
    }

}


pub(crate) fn stop_pool(env:&Env){

    let key = PoolKey::PoolStatus;

    let status = get_pool_status(env);

    if status == 1 {
        env.storage().persistent().set(&key, &2);
    }

    if status == 0 {
        panic!("You cannot stop pool before start pool")
    }

}


pub(crate) fn get_pool_status(env:&Env)->i32{

    let key = PoolKey::PoolStatus;

    env.storage().persistent().get(&key).unwrap_or(0)
}

pub(crate) fn is_pool_started(env:&Env){

    let key = PoolKey::PoolStatus;

    let status = env.storage().persistent().get::<PoolKey,i32>(&key).unwrap();

    if status == 0 {
        panic!("Pool is not started yet")
    }

    if status == 2 {
        panic!("Pool is stopped")
    }

}


pub(crate) fn is_initialized(env:&Env){

    let key = PoolKey::IsInitialized;

    let is_init = env.storage().instance().has(&key);

    if is_init{
        panic!("Already initialized")
    }else{
        init(env, &key)
    }

}

fn init(env:&Env,key:&PoolKey){
    env.storage().instance().set(key, &true);
}

pub(crate) fn set_pool_name(env:&Env,pool_name:String){

    let key = PoolKey::PoolName;

    env.storage().persistent().set(&key, &pool_name);

}

pub(crate) fn get_pool_name(env:&Env)->String{
    
    let key = PoolKey::PoolName;

    env.storage().persistent().get(&key).unwrap_or(String::from_str(env, "Default"))
}


pub(crate) fn set_x_init(env:&Env,v:i128){

    let key = PoolKey::X;

    env.storage().persistent().set(&key, &v);
}


pub(crate) fn set_x(env:&Env,v:i128){

    let key = PoolKey::X;

    let x = get_x(env);

    let value = x.checked_add(v).expect("Overflow occurs");

    env.storage().persistent().set(&key, &value);

}

pub(crate) fn get_x(env:&Env)->i128{

    let key = PoolKey::X;

    env.storage().persistent().get::<PoolKey,i128>(&key).unwrap_or(0 as i128)
}

pub(crate) fn set_pool_archive(env:&Env,pool_archive:bool){

    let key = PoolKey::Archived;

    env.storage().persistent().set(&key, &pool_archive);
}

pub(crate) fn get_pool_archive(env:&Env)->bool{
    
    let key = PoolKey::Archived;

    env.storage().persistent().get(&key).unwrap_or(false)
}

pub(crate) fn set_owner(env:&Env,owner:Address){

    let key = PoolKey::Owner;

    env.storage().persistent().set(&key, &owner);

}

pub(crate) fn get_owner(env:&Env)->Option<Address>{
    
    let key = PoolKey::Owner;

    let is_set = env.storage().persistent().get::<PoolKey,Address>(&key).is_some();

    if is_set {
        Some(env.storage().persistent().get(&key).unwrap())
    }else{
        None
    }

    // env.storage().persistent().get(&key).unwrap_or(Address::from_string(&String::from_str(env, "0x0")))
}

pub(crate) fn set_treasury_to_zero(env:&Env){

    let key = PoolKey::Treasury;

    env.storage().persistent().set(&key, &(0 as i128));

}

pub(crate) fn set_treasury_init(env:&Env,v:i128){

    let key = PoolKey::Treasury;

    env.storage().persistent().set(&key, &v);

}


pub(crate) fn set_treasury(env:&Env,v:i128){

    let key = PoolKey::Treasury;

    let x = get_treasury(env);

    let value = x.checked_add(v).expect("Overflow occurs");

    env.storage().persistent().set(&key, &value);

}

pub(crate) fn get_treasury(env:&Env)->i128{

    let key = PoolKey::Treasury;

    env.storage().persistent().get::<PoolKey,i128>(&key).unwrap_or(0 as i128)
}


pub(crate) fn set_in_secondary_mode(env:&Env,mode:bool){

    let key = PoolKey::InSecondaryMode;

    env.storage().persistent().set(&key, &mode);

}

pub(crate) fn get_in_secondary_mode(env:&Env)-> bool{

    let key = PoolKey::InSecondaryMode;

    env.storage().persistent().get(&key).unwrap_or(false)
}

pub(crate) fn set_pvt_qty_max_primary_init(env:&Env,v:i128){

    let key = PoolKey::PvtQtyMaxPrimary;

    // let scaled_value = v * (10 as i128).pow(9);

    env.storage().persistent().set(&key, &v);

}

pub(crate) fn set_pvt_qty_max_primary(env:&Env,v:i128){

    let key = PoolKey::PvtQtyMaxPrimary;

    let x = get_pvt_qty_max_primary(env);

    let scaled_value = v * (10 as i128).pow(9);

    let value = x.checked_add(scaled_value).expect("Overflow occurs");

    env.storage().persistent().set(&key, &value);

}

pub(crate) fn get_pvt_qty_max_primary(env:&Env)->i128{

    let key = PoolKey::PvtQtyMaxPrimary;

    env.storage().persistent().get(&key).unwrap_or(0)
}

pub(crate) fn set_pvt_qty_max_secondary_init(env:&Env,v:i128){

    let key = PoolKey::PvtQtyMaxSecondary;

    // let scaled_value = v * (10 as i128).pow(9);

    env.storage().persistent().set(&key, &v);

}


pub(crate) fn set_pvt_qty_max_secondary(env:&Env,v:i128){

    let key = PoolKey::PvtQtyMaxSecondary;

    let x = get_pvt_qty_max_secondary(env);

    // let scaled_value = v * (10 as i128).pow(9);

    let value = x.checked_add(v).expect("Overflow occurs");

    env.storage().persistent().set(&key, &value);

}

pub(crate) fn get_pvt_qty_max_secondary(env:&Env)->i128{

    let key = PoolKey::PvtQtyMaxSecondary;

    env.storage().persistent().get(&key).unwrap_or(0)
}

pub(crate) fn set_pvt_available_secondary_init(env:&Env,v:i128){

    let key = PoolKey::PvtAvailableSecondary;

    // let scaled_value = v * (10 as i128).pow(9);

    env.storage().persistent().set(&key, &v);

}

pub(crate) fn set_pvt_available_secondary(env:&Env,v:i128){

    let key = PoolKey::PvtAvailableSecondary;

    let x = get_pvt_available_secondary(env);

    let scaled_value = v * (10 as i128).pow(9);

    let value = x.checked_add(scaled_value).expect("Overflow occurs");

    env.storage().persistent().set(&key, &value);

}

pub(crate) fn get_pvt_available_secondary(env:&Env)->i128{

    let key = PoolKey::PvtAvailableSecondary;

    env.storage().persistent().get(&key).unwrap_or(0)
}


pub(crate) fn set_pvt_running_total_bought_init(env:&Env,v:i128){

    let key = PoolKey::PvtRunningTotalBought;

    env.storage().persistent().set(&key, &v);

}

pub(crate) fn set_pvt_running_total_bought(env:&Env,v:i128){

    let key = PoolKey::PvtRunningTotalBought;

    let x = get_pvt_running_total_bought(env);

    let value = x.checked_add(v).expect("Overflow occurs");

    env.storage().persistent().set(&key, &value);

}

pub(crate) fn get_pvt_running_total_bought(env:&Env)->i128{

    let key = PoolKey::PvtRunningTotalBought;

    env.storage().persistent().get(&key).unwrap_or(0)
}

pub(crate) fn set_pvt_running_total_sold_init(env:&Env,v:i128){

    let key = PoolKey::PvtRunningTotalSold;

    env.storage().persistent().set(&key, &v);

}

pub(crate) fn set_pvt_running_total_sold(env:&Env,v:i128){

    let key = PoolKey::PvtRunningTotalSold;

    let x = get_pvt_running_total_sold(env);

    let value = x.checked_add(v).expect("Overflow occurs");

    env.storage().persistent().set(&key, &value);

}

pub(crate) fn get_pvt_running_total_sold(env:&Env)->i128{

    let key = PoolKey::PvtRunningTotalSold;

    env.storage().persistent().get(&key).unwrap_or(0)
}

pub(crate) fn set_pvt_price_initial_primary_init(env:&Env,v:i128){

    let key = PoolKey::PvtPriceInitialPrimary;

    // let scaled_val = v * (10 as i128).pow(9);

    env.storage().persistent().set(&key, &v);

}

pub(crate) fn set_pvt_price_initial_primary(env:&Env,v:i128){

    let key = PoolKey::PvtPriceInitialPrimary;

    let x = get_pvt_price_initial_primary(env);

    let scaled_val = v * (10 as i128).pow(9);

    let value = x.checked_add(scaled_val).expect("Overflow occurs");

    env.storage().persistent().set(&key, &value);

}

pub(crate) fn get_pvt_price_initial_primary(env:&Env)->i128{

    let key = PoolKey::PvtPriceInitialPrimary;

    env.storage().persistent().get(&key).unwrap_or(0)
}

pub(crate) fn set_pvt_price_max_primary_init(env:&Env,v:i128){

    let key = PoolKey::PvtPriceMaxPrimary;

    // let scaled_value = v * (10 as i128).pow(9);

    env.storage().persistent().set(&key, &v);

}


pub(crate) fn set_pvt_price_max_primary(env:&Env,v:i128){

    let key = PoolKey::PvtPriceMaxPrimary;

    let x = get_pvt_price_max_primary(env);

    let scaled_value = v * (10 as i128).pow(9);

    let value = x.checked_add(scaled_value).expect("Overflow occurs");

    env.storage().persistent().set(&key, &value);

}

pub(crate) fn get_pvt_price_max_primary(env:&Env)->i128{

    let key = PoolKey::PvtPriceMaxPrimary;

    env.storage().persistent().get(&key).unwrap_or(0)
}

pub(crate) fn set_pvt_price_max_secondary_init(env:&Env,v:i128){

    let key = PoolKey::PvtPriceMaxSecondary;

    // let scaled_value = v * (10 as i128).pow(9);

    env.storage().persistent().set(&key, &v);

}


pub(crate) fn set_pvt_price_max_secondary(env:&Env,v:i128){

    let key = PoolKey::PvtPriceMaxSecondary;

    let x = get_pvt_price_max_secondary(env);

    let scaled_value = v * (10 as i128).pow(9);

    let value = x.checked_add(scaled_value).expect("Overflow occurs");

    env.storage().persistent().set(&key, &value);

}

pub(crate) fn get_pvt_price_max_secondary(env:&Env)->i128{

    let key = PoolKey::PvtPriceMaxSecondary;

    env.storage().persistent().get(&key).unwrap_or(0)
}

pub(crate) fn set_a_primary_midpoint_initial_and_max_init(env:&Env,pvt_price_max_primary:i128,pvt_price_initial_primary:i128){

    let key = PoolKey::APrimaryMidpointInitialAndMax;

    // let pvt_price_max_primary_scaled = pvt_price_max_primary * Q9;
    // let pvt_price_initial_primary_scaled = pvt_price_initial_primary * Q9;

    let a_primary_midpoint_initial_and_max = pvt_price_max_primary.checked_sub(pvt_price_initial_primary).expect("Underflow occur").checked_div(2).expect("Error");

    env.storage().persistent().set(&key, &a_primary_midpoint_initial_and_max);

}


pub(crate) fn set_a_primary_midpoint_initial_and_max(env:&Env){

    let key = PoolKey::APrimaryMidpointInitialAndMax;

    let x = get_a_primary_midpoint_initial_and_max(env);

    let pvt_price_max_primary = get_pvt_price_max_primary(env);
    let pvt_price_initial_primary = get_pvt_price_initial_primary(env);


    let a_primary_midpoint_initial_and_max = pvt_price_max_primary.checked_sub(pvt_price_initial_primary).expect("Underflow occur").checked_div(2).expect("Error");

    let value = x.checked_add(a_primary_midpoint_initial_and_max).expect("Overflow occurs");

    env.storage().persistent().set(&key, &value);

}

pub(crate) fn get_a_primary_midpoint_initial_and_max(env:&Env)->i128{

    let key = PoolKey::APrimaryMidpointInitialAndMax;

    env.storage().persistent().get(&key).unwrap_or(0)
}


pub(crate) fn set_b_primary_half_max_qty_init(env:&Env,pvt_qty_max_primary:i128){

    let key = PoolKey::BPrimaryHalfMaxQty;
    
    // let pvt_qty_max_primary_scaled = pvt_qty_max_primary * Q9;

    let b_primary_half_max_qty = pvt_qty_max_primary.checked_div(2).expect("Error");

    env.storage().persistent().set(&key, &b_primary_half_max_qty);

}


pub(crate) fn set_b_primary_half_max_qty(env:&Env){

    let key = PoolKey::BPrimaryHalfMaxQty;

    let x = get_b_primary_half_max_qty(env);
    
    let pvt_qty_max_primary = get_pvt_qty_max_primary(env);

    let b_primary_half_max_qty = pvt_qty_max_primary.checked_div(2).expect("Error");

    
    let value = x.checked_add(b_primary_half_max_qty).expect("Overflow occurs");

    env.storage().persistent().set(&key, &value);

}

pub(crate) fn get_b_primary_half_max_qty(env:&Env)->i128{

    let key = PoolKey::BPrimaryHalfMaxQty;

    env.storage().persistent().get(&key).unwrap_or(0)
}

pub(crate) fn set_c_primary_steepness_init(env:&Env,steepness:u32){

    let key = PoolKey::CPrimarySteepness;

    env.storage().persistent().set(&key, &steepness);

}

pub(crate) fn set_c_primary_steepness(env:&Env,steepness:u32){

    let key = PoolKey::CPrimarySteepness;

    let x = get_c_primary_steepness(env);
    
    let value = x.checked_add(steepness).expect("Overflow occurs");

    env.storage().persistent().set(&key, &value);

}

pub(crate) fn get_c_primary_steepness(env:&Env)->u32{

    let key = PoolKey::CPrimarySteepness;

    env.storage().persistent().get(&key).unwrap_or(0)
}

pub(crate) fn set_a_secondary_midpoint_initial_and_max_init(env:&Env,pvt_price_max_secondary:i128,pvt_price_initial_primary:i128){

    let key = PoolKey::ASecondaryMidpointInitialAndMax;

    // let pvt_price_max_secondary_scaled = pvt_price_max_secondary * Q9;
    // let pvt_price_initial_primary_scaled = pvt_price_initial_primary * Q9;

    let a_secondary_midpoint_initial_and_max = pvt_price_max_secondary.checked_sub(pvt_price_initial_primary).expect("Underflow occur").checked_div(2).expect("Error");

    env.storage().persistent().set(&key, &a_secondary_midpoint_initial_and_max);

}

pub(crate) fn set_a_secondary_midpoint_initial_and_max(env:&Env){

    let key = PoolKey::ASecondaryMidpointInitialAndMax;

    let x = get_a_secondary_midpoint_initial_and_max(env);

    let pvt_price_max_secondary = get_pvt_price_max_secondary(env);
    let pvt_price_initial_primary = get_pvt_price_initial_primary(env);

    let a_secondary_midpoint_initial_and_max = pvt_price_max_secondary.checked_sub(pvt_price_initial_primary).expect("Underflow occur").checked_div(2).expect("Error");

    let value = x.checked_add(a_secondary_midpoint_initial_and_max).expect("Overflow occurs");

    env.storage().persistent().set(&key, &value);

}



pub(crate) fn get_a_secondary_midpoint_initial_and_max(env:&Env)->i128{

    let key = PoolKey::ASecondaryMidpointInitialAndMax;

    env.storage().persistent().get(&key).unwrap_or(0)
}

pub(crate) fn set_b_secondary_half_max_qty_init(env:&Env,pvt_qty_max_primary:i128,pvt_qty_max_secondary:i128){

    let key = PoolKey::BSecondaryHalfMaxQty;
    
    // let pvt_qty_max_primary_scaled = pvt_qty_max_primary * Q9;

    // let pvt_qty_max_secondary_scaled = pvt_qty_max_secondary * Q9;

    let b_secondary_half_max_qty = pvt_qty_max_primary.checked_add(pvt_qty_max_secondary).expect("Overflow occur").checked_div(2).expect("Error");

    env.storage().persistent().set(&key, &b_secondary_half_max_qty);

}


pub(crate) fn set_b_secondary_half_max_qty(env:&Env){

    let key = PoolKey::BSecondaryHalfMaxQty;

    let x = get_b_secondary_half_max_qty(env);
    
    let pvt_qty_max_primary = get_pvt_qty_max_primary(env);

    let pvt_qty_max_secondary = get_pvt_qty_max_secondary(env);//working

    let b_secondary_half_max_qty = pvt_qty_max_primary.checked_add(pvt_qty_max_secondary).expect("Overflow occur").checked_div(2).expect("Error");

    let value = x.checked_add(b_secondary_half_max_qty).expect("Overflow occurs");

    env.storage().persistent().set(&key, &value);

}

pub(crate) fn get_b_secondary_half_max_qty(env:&Env)->i128{

    let key = PoolKey::BSecondaryHalfMaxQty;

    env.storage().persistent().get(&key).unwrap_or(0)
}

pub(crate) fn set_c_secondary_steepness_init(env:&Env,steepness:u32){

    let key = PoolKey::CSecondarySteepness;
    
    env.storage().persistent().set(&key, &steepness);

}


pub(crate) fn set_c_secondary_steepness(env:&Env,steepness:u32){

    let key = PoolKey::CSecondarySteepness;

    let x = get_c_secondary_steepness(env);
    
    let value = x.checked_add(steepness).expect("Overflow occurs");

    env.storage().persistent().set(&key, &value);

}

pub(crate) fn get_c_secondary_steepness(env:&Env)->u32{

    let key = PoolKey::CSecondarySteepness;

    env.storage().persistent().get(&key).unwrap_or(0)
}

pub(crate) fn set_p_prime_init(env:&Env,pvt_price_initial_primary:i128){

    let key = PoolKey::PPrime;

    // let pvt_price_initial_primary_scaled = pvt_price_initial_primary * Q9;

    let unadjusted_price = get_unadjusted_price(env, 1);

    let p_prime = pvt_price_initial_primary.checked_sub(unadjusted_price).expect("Underflow");
    
    env.storage().persistent().set(&key, &p_prime);

}

pub(crate) fn set_p_prime(env:&Env){

    let key = PoolKey::PPrime;

    let x = get_p_prime(env);

    let pvt_price_initial_primary = get_pvt_price_initial_primary(env);

    let unadjusted_price = get_unadjusted_price(env, 1);


    let p_prime = pvt_price_initial_primary.checked_sub(unadjusted_price).expect("Underflow");

    
    let value = x.checked_add(p_prime).expect("Overflow occurs");

    env.storage().persistent().set(&key, &value);

}

pub(crate) fn get_p_prime(env:&Env)->i128{

    let key = PoolKey::PPrime;

    env.storage().persistent().get(&key).unwrap_or(0)
}



pub(crate) fn set_p_doubleprime(env:&Env){

    let key = PoolKey::PDoublePrime;

    let x = get_p_doubleprime(env);

    let b_secondary_half_max_qty = get_b_secondary_half_max_qty(env);

    let a_secondary_midpoint_initial_and_max = get_a_secondary_midpoint_initial_and_max(env);

    let p_doubleprime = get_price_primary(env, (b_secondary_half_max_qty / (10 as i128).pow(9))).checked_sub(a_secondary_midpoint_initial_and_max).expect("Underflow");

    
    let value = x.checked_add(p_doubleprime).expect("Overflow occurs");

    env.storage().persistent().set(&key, &value);

}

pub(crate) fn get_p_doubleprime(env:&Env)->i128{

    let key = PoolKey::PDoublePrime;

    env.storage().persistent().get(&key).unwrap_or(0)
}

pub(crate) fn set_soldout_hits_init(env:&Env,v:i128){

    let key = PoolKey::SoldOutHits;
    
    env.storage().persistent().set(&key, &v);

}

pub(crate) fn set_soldout_hits(env:&Env,v:i128){

    let key = PoolKey::SoldOutHits;

    let x = get_soldout_hits(env);
    
    let value = x.checked_add(v).expect("Overflow occurs");

    env.storage().persistent().set(&key, &value);

}

pub(crate) fn get_soldout_hits(env:&Env)->i128{

    let key = PoolKey::SoldOutHits;

    env.storage().persistent().get(&key).unwrap_or(0)
}


pub(crate) fn get_unadjusted_price(env:&Env,x:i128)->i128{

    let a_primary_midpoint_initial_and_max = get_a_primary_midpoint_initial_and_max(env);

    let b_primary_half_max_qty = get_b_primary_half_max_qty(env);

    let c_primary_steepness = get_c_primary_steepness(env);

    let sq = sqrt(((c_primary_steepness as i128) + (x - (b_primary_half_max_qty / (10 as i128).pow(9))).fixed_mul_ceil((x - (b_primary_half_max_qty / (10 as i128).pow(9))), 1).unwrap()) * (10 as i128).pow(18));


    let v = a_primary_midpoint_initial_and_max * ((((x - (b_primary_half_max_qty / (10 as i128).pow(9)))*(10 as i128).pow(18)) / sq) + 1000000000 );

    v / (10 as i128).pow(9)
}


pub(crate) fn get_price_primary(env:&Env,x:i128)->i128{


    let a_primary_midpoint_initial_and_max = get_a_primary_midpoint_initial_and_max(env);

    let b_primary_half_max_qty = get_b_primary_half_max_qty(env);

    let c_primary_steepness = get_c_primary_steepness(env);

    let p_prime = get_p_prime(env);
    
    let sq = sqrt(((c_primary_steepness as i128) + (x  - (b_primary_half_max_qty / Q9)).fixed_mul_ceil((x - (b_primary_half_max_qty / Q9)), 1).unwrap()) * (10 as i128).pow(18));


    let v = a_primary_midpoint_initial_and_max * ((((x - (b_primary_half_max_qty / (10 as i128).pow(9)))*(10 as i128).pow(18)) / sq) + 1000000000 );

    v / (10 as i128).pow(9) + p_prime

}


pub(crate) fn get_price_secondary(env:&Env,x:i128)->i128{

    let a_secondary_midpoint_initial_and_max = get_a_secondary_midpoint_initial_and_max(env);

    let b_secondary_half_max_qty = get_b_secondary_half_max_qty(env);

    let c_secondary_steepness = get_c_secondary_steepness(env);

    let p_doubleprime = get_p_doubleprime(env);


    let sq= sqrt(((c_secondary_steepness as i128) + (x - (b_secondary_half_max_qty / (10 as i128).pow(9))).fixed_mul_ceil((x - (b_secondary_half_max_qty / (10 as i128).pow(9))), 1).unwrap()) * (10 as i128).pow(18));


    let v = a_secondary_midpoint_initial_and_max * ((((x - (b_secondary_half_max_qty / (10 as i128).pow(9)))*(10 as i128).pow(18)) / sq) + 1000000000 ) ;

    v / (10 as i128).pow(9) + p_doubleprime
}


pub(crate) fn set_pvt_address(env:&Env,token_address:Address){

    let key = PoolKey::TokenAddress;

    let is_available = env.storage().instance().has(&key);

    if is_available {
        panic!("Token address already set")
    }else{
        env.storage().instance().set(&key, &token_address);
    }

}


pub(crate) fn set_usdc_address(env:&Env,usdc_address:Address){

    let key = PoolKey::USDCAddress;

    let is_available = env.storage().instance().has(&key);

    if is_available {
        panic!("USDC address already set")
    }else{
        env.storage().instance().set(&key, &usdc_address);
    }

}

pub(crate) fn get_usdc_token(env:&Env)-> Client{
    let key = PoolKey::USDCAddress;

    let token_address:Address = env.storage().instance().get(&key).unwrap();

    lptoken::Client::new(env, &token_address)
}

pub(crate) fn get_pvt_token(env:&Env)-> Client{

    let key = PoolKey::TokenAddress;

    let token_address:Address = env.storage().instance().get(&key).unwrap();

    lptoken::Client::new(env, &token_address)
}




pub(crate) fn check_switch(env:&Env){
    let x = get_x(&env);
    let pvt_qty_max_primary = get_pvt_qty_max_primary(&env);
    let pvt_qty_max_primary_unscaled = pvt_qty_max_primary / (10 as i128).pow(9);

    if x == pvt_qty_max_primary_unscaled {
        set_in_secondary_mode(env, true);
    }
}

pub(crate) fn withdraw_all_fund(env:&Env,owner:Address){

    let status = get_pool_status(env);
    
    if status == 2 {

        let treasury = get_treasury(env);

        let usdc = get_usdc_token(env);

        set_treasury_to_zero(&env);

        usdc.transfer(&env.current_contract_address(), &owner, &treasury);

    }else{
        panic!("You cannot withdraw fund before stop pool");
    }

    
}








