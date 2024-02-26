use core::ops::Add;

use num_integer::sqrt;
use soroban_sdk::{log, token::{self, TokenClient}, Address, Env, String};

use crate::{lptoken::{self, Client}, PoolKey};

pub(crate) const DAY_IN_LEDGERS: u32 = 17280;
pub(crate) const INSTANCE_BUMP_AMOUNT: u32 = 7 * DAY_IN_LEDGERS;
pub(crate) const INSTANCE_LIFETIME_THRESHOLD: u32 = INSTANCE_BUMP_AMOUNT - DAY_IN_LEDGERS;

pub(crate) const PERSISTENT_STORAGE_BUMP_AMOUNT: u32 = 30 * DAY_IN_LEDGERS;
pub(crate) const PERSISTENT_STORAGE_LIFETIME_THRESHOLD: u32 = PERSISTENT_STORAGE_BUMP_AMOUNT - DAY_IN_LEDGERS;


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

    env.storage().persistent().extend_ttl(&key, PERSISTENT_STORAGE_LIFETIME_THRESHOLD, PERSISTENT_STORAGE_BUMP_AMOUNT)

}

pub(crate) fn get_pool_name(env:&Env)->String{
    
    let key = PoolKey::PoolName;

    env.storage().persistent().get(&key).unwrap_or(String::from_str(env, "Default"))
}


pub(crate) fn set_x(env:&Env,v:i128){

    let key = PoolKey::X;

    let x = get_x(env);

    let value = x.checked_add(v).expect("Overflow occurs");

    env.storage().persistent().set(&key, &value);

    env.storage().persistent().extend_ttl(&key, PERSISTENT_STORAGE_LIFETIME_THRESHOLD, PERSISTENT_STORAGE_BUMP_AMOUNT);

}

pub(crate) fn get_x(env:&Env)->i128{

    let key = PoolKey::X;

    env.storage().persistent().get(&key).unwrap_or(0)
}

pub(crate) fn set_pool_archive(env:&Env,pool_archive:bool){

    let key = PoolKey::Archived;

    env.storage().persistent().set(&key, &pool_archive);

    env.storage().persistent().extend_ttl(&key, PERSISTENT_STORAGE_LIFETIME_THRESHOLD, PERSISTENT_STORAGE_BUMP_AMOUNT)

}

pub(crate) fn get_pool_archive(env:&Env)->bool{
    
    let key = PoolKey::Archived;

    env.storage().persistent().get(&key).unwrap_or(false)
}

pub(crate) fn set_owner(env:&Env,owner:Address){

    let key = PoolKey::Owner;

    env.storage().persistent().set(&key, &owner);

    env.storage().persistent().extend_ttl(&key, PERSISTENT_STORAGE_LIFETIME_THRESHOLD, PERSISTENT_STORAGE_BUMP_AMOUNT)

}

pub(crate) fn get_owner(env:&Env)->Address{
    
    let key = PoolKey::Owner;

    env.storage().persistent().get(&key).unwrap_or(Address::from_string(&String::from_str(env, "0x0")))
}


pub(crate) fn set_treasury(env:&Env,v:i128){

    let key = PoolKey::Treasury;

    let x = get_treasury(env);

    let value = x.checked_add(v).expect("Overflow occurs");

    env.storage().persistent().set(&key, &value);

    env.storage().persistent().extend_ttl(&key, PERSISTENT_STORAGE_LIFETIME_THRESHOLD, PERSISTENT_STORAGE_BUMP_AMOUNT);

}

pub(crate) fn get_treasury(env:&Env)->i128{

    let key = PoolKey::Treasury;

    env.storage().persistent().get(&key).unwrap_or(0)
}


pub(crate) fn set_in_secondary_mode(env:&Env,mode:bool){

    let key = PoolKey::InSecondaryMode;

    env.storage().persistent().set(&key, &mode);

    env.storage().persistent().extend_ttl(&key, PERSISTENT_STORAGE_LIFETIME_THRESHOLD, PERSISTENT_STORAGE_BUMP_AMOUNT);

}

pub(crate) fn get_in_secondary_mode(env:&Env)-> bool{

    let key = PoolKey::InSecondaryMode;

    env.storage().persistent().get(&key).unwrap_or(false)
}



pub(crate) fn set_pvt_qty_max_primary(env:&Env,v:i128){

    let key = PoolKey::PvtQtyMaxPrimary;

    let x = get_pvt_qty_max_primary(env);

    let value = x.checked_add(v).expect("Overflow occurs");

    env.storage().persistent().set(&key, &value);

    env.storage().persistent().extend_ttl(&key, PERSISTENT_STORAGE_LIFETIME_THRESHOLD, PERSISTENT_STORAGE_BUMP_AMOUNT);

}

pub(crate) fn get_pvt_qty_max_primary(env:&Env)->i128{

    let key = PoolKey::PvtQtyMaxPrimary;

    env.storage().persistent().get(&key).unwrap_or(0)
}


pub(crate) fn set_pvt_qty_max_secondary(env:&Env,v:i128){

    let key = PoolKey::PvtQtyMaxSecondary;

    let x = get_pvt_qty_max_secondary(env);

    let value = x.checked_add(v).expect("Overflow occurs");

    env.storage().persistent().set(&key, &value);

    env.storage().persistent().extend_ttl(&key, PERSISTENT_STORAGE_LIFETIME_THRESHOLD, PERSISTENT_STORAGE_BUMP_AMOUNT);

}

pub(crate) fn get_pvt_qty_max_secondary(env:&Env)->i128{

    let key = PoolKey::PvtQtyMaxSecondary;

    env.storage().persistent().get(&key).unwrap_or(0)
}

pub(crate) fn set_pvt_available_secondary(env:&Env,v:i128){

    let key = PoolKey::PvtAvailableSecondary;

    let x = get_pvt_available_secondary(env);

    let value = x.checked_add(v).expect("Overflow occurs");

    env.storage().persistent().set(&key, &value);

    env.storage().persistent().extend_ttl(&key, PERSISTENT_STORAGE_LIFETIME_THRESHOLD, PERSISTENT_STORAGE_BUMP_AMOUNT);

}

pub(crate) fn get_pvt_available_secondary(env:&Env)->i128{

    let key = PoolKey::PvtAvailableSecondary;

    env.storage().persistent().get(&key).unwrap_or(0)
}


pub(crate) fn set_pvt_running_total_bought(env:&Env,v:i128){

    let key = PoolKey::PvtRunningTotalBought;

    let x = get_pvt_running_total_bought(env);

    let value = x.checked_add(v).expect("Overflow occurs");

    env.storage().persistent().set(&key, &value);

    env.storage().persistent().extend_ttl(&key, PERSISTENT_STORAGE_LIFETIME_THRESHOLD, PERSISTENT_STORAGE_BUMP_AMOUNT);

}

pub(crate) fn get_pvt_running_total_bought(env:&Env)->i128{

    let key = PoolKey::PvtRunningTotalBought;

    env.storage().persistent().get(&key).unwrap_or(0)
}

pub(crate) fn set_pvt_running_total_sold(env:&Env,v:i128){

    let key = PoolKey::PvtRunningTotalSold;

    let x = get_pvt_running_total_sold(env);

    let value = x.checked_add(v).expect("Overflow occurs");

    env.storage().persistent().set(&key, &value);

    env.storage().persistent().extend_ttl(&key, PERSISTENT_STORAGE_LIFETIME_THRESHOLD, PERSISTENT_STORAGE_BUMP_AMOUNT);

}

pub(crate) fn get_pvt_running_total_sold(env:&Env)->i128{

    let key = PoolKey::PvtRunningTotalSold;

    env.storage().persistent().get(&key).unwrap_or(0)
}

pub(crate) fn set_pvt_price_initial_primary(env:&Env,v:i128){

    let key = PoolKey::PvtPriceInitialPrimary;

    let x = get_pvt_price_initial_primary(env);

    let value = x.checked_add(v).expect("Overflow occurs");

    env.storage().persistent().set(&key, &value);

    env.storage().persistent().extend_ttl(&key, PERSISTENT_STORAGE_LIFETIME_THRESHOLD, PERSISTENT_STORAGE_BUMP_AMOUNT);

}

pub(crate) fn get_pvt_price_initial_primary(env:&Env)->i128{

    let key = PoolKey::PvtPriceInitialPrimary;

    env.storage().persistent().get(&key).unwrap_or(0)
}


pub(crate) fn set_pvt_price_max_primary(env:&Env,v:i128){

    let key = PoolKey::PvtPriceMaxPrimary;

    let x = get_pvt_price_max_primary(env);

    let value = x.checked_add(v).expect("Overflow occurs");

    env.storage().persistent().set(&key, &value);

    env.storage().persistent().extend_ttl(&key, PERSISTENT_STORAGE_LIFETIME_THRESHOLD, PERSISTENT_STORAGE_BUMP_AMOUNT);

}

pub(crate) fn get_pvt_price_max_primary(env:&Env)->i128{

    let key = PoolKey::PvtPriceMaxPrimary;

    env.storage().persistent().get(&key).unwrap_or(0)
}


pub(crate) fn set_pvt_price_max_secondary(env:&Env,v:i128){

    let key = PoolKey::PvtPriceMaxSecondary;

    let x = get_pvt_price_max_secondary(env);

    let value = x.checked_add(v).expect("Overflow occurs");

    env.storage().persistent().set(&key, &value);

    env.storage().persistent().extend_ttl(&key, PERSISTENT_STORAGE_LIFETIME_THRESHOLD, PERSISTENT_STORAGE_BUMP_AMOUNT);

}

pub(crate) fn get_pvt_price_max_secondary(env:&Env)->i128{

    let key = PoolKey::PvtPriceMaxSecondary;

    env.storage().persistent().get(&key).unwrap_or(0)
}


pub(crate) fn set_a_primary_midpoint_initial_and_max(env:&Env){

    let key = PoolKey::APrimaryMidpointInitialAndMax;

    let x = get_a_primary_midpoint_initial_and_max(env);

    let pvt_price_max_primary = get_pvt_price_max_primary(env);
    let pvt_price_initial_primary = get_pvt_price_initial_primary(env);

    let a_primary_midpoint_initial_and_max = pvt_price_max_primary.checked_sub(pvt_price_initial_primary).expect("Underflow occur").checked_div(2).expect("Error");

    let value = x.checked_add(a_primary_midpoint_initial_and_max).expect("Overflow occurs");

    env.storage().persistent().set(&key, &value);

    env.storage().persistent().extend_ttl(&key, PERSISTENT_STORAGE_LIFETIME_THRESHOLD, PERSISTENT_STORAGE_BUMP_AMOUNT);

}

pub(crate) fn get_a_primary_midpoint_initial_and_max(env:&Env)->i128{

    let key = PoolKey::APrimaryMidpointInitialAndMax;

    env.storage().persistent().get(&key).unwrap_or(0)
}



pub(crate) fn set_b_primary_half_max_qty(env:&Env){

    let key = PoolKey::BPrimaryHalfMaxQty;

    let x = get_b_primary_half_max_qty(env);
    
    let pvt_qty_max_primary = get_pvt_qty_max_primary(env);

    let b_primary_half_max_qty = pvt_qty_max_primary.checked_div(2).expect("Error");

    let value = x.checked_add(b_primary_half_max_qty).expect("Overflow occurs");

    env.storage().persistent().set(&key, &value);

    env.storage().persistent().extend_ttl(&key, PERSISTENT_STORAGE_LIFETIME_THRESHOLD, PERSISTENT_STORAGE_BUMP_AMOUNT);

}

pub(crate) fn get_b_primary_half_max_qty(env:&Env)->i128{

    let key = PoolKey::BPrimaryHalfMaxQty;

    env.storage().persistent().get(&key).unwrap_or(0)
}

pub(crate) fn set_c_primary_steepness(env:&Env,steepness:u32){

    let key = PoolKey::CPrimarySteepness;

    let x = get_c_primary_steepness(env);
    
    let value = x.checked_add(steepness).expect("Overflow occurs");

    env.storage().persistent().set(&key, &value);

    env.storage().persistent().extend_ttl(&key, PERSISTENT_STORAGE_LIFETIME_THRESHOLD, PERSISTENT_STORAGE_BUMP_AMOUNT);

}

pub(crate) fn get_c_primary_steepness(env:&Env)->u32{

    let key = PoolKey::CPrimarySteepness;

    env.storage().persistent().get(&key).unwrap_or(0)
}



pub(crate) fn set_a_secondary_midpoint_initial_and_max(env:&Env){

    let key = PoolKey::ASecondaryMidpointInitialAndMax;

    let x = get_a_secondary_midpoint_initial_and_max(env);

    let pvt_price_max_secondary = get_pvt_price_max_secondary(env);
    let pvt_price_initial_primary = get_pvt_price_initial_primary(env);

    let a_secondary_midpoint_initial_and_max = pvt_price_max_secondary.checked_sub(pvt_price_initial_primary).expect("Underflow occur").checked_div(2).expect("Error");

    let value = x.checked_add(a_secondary_midpoint_initial_and_max).expect("Overflow occurs");

    env.storage().persistent().set(&key, &value);

    env.storage().persistent().extend_ttl(&key, PERSISTENT_STORAGE_LIFETIME_THRESHOLD, PERSISTENT_STORAGE_BUMP_AMOUNT);

}

pub(crate) fn get_a_secondary_midpoint_initial_and_max(env:&Env)->i128{

    let key = PoolKey::ASecondaryMidpointInitialAndMax;

    env.storage().persistent().get(&key).unwrap_or(0)
}


pub(crate) fn set_b_secondary_half_max_qty(env:&Env){

    let key = PoolKey::BSecondaryHalfMaxQty;

    let x = get_b_secondary_half_max_qty(env);
    
    let pvt_qty_max_primary = get_pvt_qty_max_primary(env);

    let pvt_qty_max_secondary = get_pvt_qty_max_secondary(env);

    let b_secondary_half_max_qty = pvt_qty_max_primary.checked_add(pvt_qty_max_secondary).expect("Overflow occur").checked_div(2).expect("Error");

    let value = x.checked_add(b_secondary_half_max_qty).expect("Overflow occurs");

    env.storage().persistent().set(&key, &value);

    env.storage().persistent().extend_ttl(&key, PERSISTENT_STORAGE_LIFETIME_THRESHOLD, PERSISTENT_STORAGE_BUMP_AMOUNT);

}

pub(crate) fn get_b_secondary_half_max_qty(env:&Env)->i128{

    let key = PoolKey::BSecondaryHalfMaxQty;

    env.storage().persistent().get(&key).unwrap_or(0)
}


pub(crate) fn set_c_secondary_steepness(env:&Env,steepness:u32){

    let key = PoolKey::CSecondarySteepness;

    let x = get_c_secondary_steepness(env);
    
    let value = x.checked_add(steepness).expect("Overflow occurs");

    env.storage().persistent().set(&key, &value);

    env.storage().persistent().extend_ttl(&key, PERSISTENT_STORAGE_LIFETIME_THRESHOLD, PERSISTENT_STORAGE_BUMP_AMOUNT);

}

pub(crate) fn get_c_secondary_steepness(env:&Env)->u32{

    let key = PoolKey::CSecondarySteepness;

    env.storage().persistent().get(&key).unwrap_or(0)
}


pub(crate) fn set_p_prime(env:&Env){

    let key = PoolKey::PPrime;

    let x = get_p_prime(env);

    let pvt_price_initial_primary = get_pvt_price_initial_primary(env);

    let unadjusted_price = get_unadjusted_price(env, 1);

    let p_prime = pvt_price_initial_primary.checked_sub(unadjusted_price).expect("Underflow");
    
    let value = x.checked_add(p_prime).expect("Overflow occurs");

    env.storage().persistent().set(&key, &value);

    env.storage().persistent().extend_ttl(&key, PERSISTENT_STORAGE_LIFETIME_THRESHOLD, PERSISTENT_STORAGE_BUMP_AMOUNT);

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

    let p_doubleprime = get_price_primary(env, b_secondary_half_max_qty).checked_sub(a_secondary_midpoint_initial_and_max).expect("Underflow");

    
    let value = x.checked_add(p_doubleprime).expect("Overflow occurs");

    env.storage().persistent().set(&key, &value);

    env.storage().persistent().extend_ttl(&key, PERSISTENT_STORAGE_LIFETIME_THRESHOLD, PERSISTENT_STORAGE_BUMP_AMOUNT);

}

pub(crate) fn get_p_doubleprime(env:&Env)->i128{

    let key = PoolKey::PDoublePrime;

    env.storage().persistent().get(&key).unwrap_or(0)
}

pub(crate) fn set_soldout_hits(env:&Env,v:i128){

    let key = PoolKey::SoldOutHits;

    let x = get_soldout_hits(env);
    
    let value = x.checked_add(v).expect("Overflow occurs");

    env.storage().persistent().set(&key, &value);

    env.storage().persistent().extend_ttl(&key, PERSISTENT_STORAGE_LIFETIME_THRESHOLD, PERSISTENT_STORAGE_BUMP_AMOUNT);

}

pub(crate) fn get_soldout_hits(env:&Env)->i128{

    let key = PoolKey::SoldOutHits;

    env.storage().persistent().get(&key).unwrap_or(0)
}






pub(crate) fn get_unadjusted_price(env:&Env,x:i128)->i128{

    let a_primary_midpoint_initial_and_max = get_a_primary_midpoint_initial_and_max(env);

    let b_primary_half_max_qty = get_b_primary_half_max_qty(env);

    let c_primary_steepness = get_c_primary_steepness(env);

    log!(env,"Hello {}",a_primary_midpoint_initial_and_max);
    log!(env,"Hello1 {}",b_primary_half_max_qty);
    log!(env,"Hello2 {}",c_primary_steepness);


    let a = x.checked_sub(b_primary_half_max_qty).expect("Underflow");
    
    log!(env,"Hello3 {}",a);

    let b = sqrt((c_primary_steepness as i128).checked_add(a).expect("Overflow").checked_mul(a).expect("Overflow"));

    log!(env,"Hello4 {}",b);

    let c = a.checked_div(b).expect("Error").checked_add(1).expect("Overflow");

    let value = a_primary_midpoint_initial_and_max.checked_mul(c).expect("Overflow");

    value
}


pub(crate) fn get_price_primary(env:&Env,x:i128)->i128{

    let a_primary_midpoint_initial_and_max = get_a_primary_midpoint_initial_and_max(env);

    let b_primary_half_max_qty = get_b_primary_half_max_qty(env);

    let c_primary_steepness = get_c_primary_steepness(env);

    let p_prime = get_p_prime(env);

    let a: i128 = x.checked_sub(b_primary_half_max_qty).expect("Underflow");

    let b = sqrt((c_primary_steepness as i128).checked_add(a).expect("Overflow").checked_mul(a).expect("Overflow"));

    let c = a.checked_div(b).expect("Error").checked_add(1).expect("Overflow");

    let value = a_primary_midpoint_initial_and_max.checked_mul(c).expect("Overflow").checked_add(p_prime).expect("Overflow");

    value
}


pub(crate) fn get_price_secondary(env:&Env,x:i128)->i128{

    let a_secondary_midpoint_initial_and_max = get_a_secondary_midpoint_initial_and_max(env);

    let b_secondary_half_max_qty = get_b_secondary_half_max_qty(env);

    let c_secondary_steepness = get_c_secondary_steepness(env);

    let p_doubleprime = get_p_doubleprime(env);

    let a: i128 = x.checked_sub(b_secondary_half_max_qty).expect("Underflow");

    let b = sqrt((c_secondary_steepness as i128).checked_add(a).expect("Overflow").checked_mul(a).expect("Overflow"));

    let c = a.checked_div(b).expect("Error").checked_add(1).expect("Overflow");

    let value = a_secondary_midpoint_initial_and_max.checked_mul(c).expect("Overflow").checked_add(p_doubleprime).expect("Overflow");

    value
}



pub(crate) fn set_token_address(env:&Env,token_address:Address){

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

pub(crate) fn get_token_token(env:&Env)-> Client{

    let key = PoolKey::TokenAddress;

    let token_address:Address = env.storage().instance().get(&key).unwrap();

    lptoken::Client::new(env, &token_address)
}




pub(crate) fn check_switch(env:&Env){
    let x = get_x(&env);
    let pvt_qty_max_primary = get_pvt_qty_max_primary(&env);

    if(x == pvt_qty_max_primary){
        set_in_secondary_mode(env, true);
    }
}

