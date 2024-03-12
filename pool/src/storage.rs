use core::ops::Add;

use num_integer::sqrt;
use soroban_fixed_point_math::FixedPoint;
use soroban_sdk::{ Address, Env, IntoVal, String};

use crate::{lptoken::{self, Client}, PoolKey};

pub(crate) const DAY_IN_LEDGERS: u32 = 17280;
pub(crate) const INSTANCE_BUMP_AMOUNT: u32 = 7 * DAY_IN_LEDGERS;
pub(crate) const INSTANCE_LIFETIME_THRESHOLD: u32 = INSTANCE_BUMP_AMOUNT - DAY_IN_LEDGERS;

pub(crate) const PERSISTENT_STORAGE_BUMP_AMOUNT: u32 = 30 * DAY_IN_LEDGERS;
pub(crate) const PERSISTENT_STORAGE_LIFETIME_THRESHOLD: u32 = PERSISTENT_STORAGE_BUMP_AMOUNT - DAY_IN_LEDGERS;

const Q9:i128 = (10 as i128).pow(9);

//Note
//checked_add - this function use for do addition without overflow
//checked_sub - this function use for do subtraction without underflow
//checked_div - this function use for do addition without underflow


//Internal start pool function which change the status of pool
pub(crate) fn start_pool(env:&Env){

    let key = PoolKey::PoolStatus;

    let status = get_pool_status(env);
    
    //If the pool status equals to 0(Not started yet) then set pool status to 1(Started)
    //Only works if pool not started
    if status == 0 {
        env.storage().persistent().set(&key, &1); 
    }

    //If the pool status equals to 2(Stopped) then it will panic
    //We cannot start pool once it stopped
    if status == 2 {
        panic!("You cannot start pool once stop")
    }

}


//Internal stop pool function which change the status of pool
pub(crate) fn stop_pool(env:&Env){

    let key = PoolKey::PoolStatus;

    let status = get_pool_status(env);

    //If the pool status equals to 1(Running) then set pool status to 2(Stop)
    //We cannot stop pool if it's not running
    if status == 1 {
        env.storage().persistent().set(&key, &2);
    }
    //If status equals to 0 (Not started yet) it will panic
    //We cannot stop pool if not running
    if status == 0 {
        panic!("You cannot stop pool before start pool")
    }

}

//function helps to get pool status
pub(crate) fn get_pool_status(env:&Env)->i32{

    let key = PoolKey::PoolStatus;

    env.storage().persistent().get(&key).unwrap_or(0)
}

//Function check whether pool running or not
pub(crate) fn is_pool_started(env:&Env){

    let key = PoolKey::PoolStatus;

    let status = env.storage().persistent().get::<PoolKey,i32>(&key).unwrap();

    //If pool not started it will panic 
    if status == 0 {
        panic!("Pool is not started yet")
    }

    //If pool stopped it will panic 
    if status == 2 {
        panic!("Pool is stopped")
    }

}

//Function use for check pool initialized or not
pub(crate) fn is_initialized(env:&Env){

    let key = PoolKey::IsInitialized;

    let is_init = env.storage().instance().has(&key);

    //Panic if pool already initialized other wise set pool initialize to true
    if is_init{
        panic!("Already initialized")
    }else{
        //Update pool initialize status
        init(env, &key)
    }

}

//Set initialize to true
fn init(env:&Env,key:&PoolKey){
    env.storage().instance().set(key, &true);
}

//Function use for set name into pool on initialization
pub(crate) fn set_pool_name(env:&Env,pool_name:String){

    let key = PoolKey::PoolName;

    env.storage().persistent().set(&key, &pool_name);

}

//Function use for set x value on initialization
pub(crate) fn set_x_init(env:&Env,v:i128){

    let key = PoolKey::X;

    env.storage().persistent().set(&key, &v);
}

//Function use for update x 
pub(crate) fn set_x(env:&Env,v:i128){

    let key = PoolKey::X;

    //Get existing value
    let x = get_x(env);

    //Update x value by adding new value
    let value = x.checked_add(v).expect("Overflow occurs");

    //Update 
    env.storage().persistent().set(&key, &value);

}

//Function use for get value of X
pub(crate) fn get_x(env:&Env)->i128{

    let key = PoolKey::X;

    env.storage().persistent().get::<PoolKey,i128>(&key).unwrap_or(0 as i128)
}

//Function use for update pool archive status
pub(crate) fn set_pool_archive(env:&Env,pool_archive:bool){

    let key = PoolKey::Archived;

    env.storage().persistent().set(&key, &pool_archive);
}

//Function use for set owner on initialization
pub(crate) fn set_owner(env:&Env,owner:Address){

    let key = PoolKey::Owner;

    env.storage().persistent().set(&key, &owner);

}

//Function use for get existing owner of pool
pub(crate) fn get_owner(env:&Env)->Option<Address>{
    
    let key = PoolKey::Owner;

    //Check owner value
    let is_set = env.storage().persistent().get::<PoolKey,Address>(&key).is_some();

    //If owner already set it will return owner address other wise return return none
    if is_set {
        Some(env.storage().persistent().get(&key).unwrap())
    }else{
        None
    }

}

//Function use for set treasury to zero when use withdraw fund
pub(crate) fn set_treasury_to_zero(env:&Env){

    let key = PoolKey::Treasury;

    //Set treasury to zero
    env.storage().persistent().set(&key, &(0 as i128));

}

//Function use for update value of treasury
pub(crate) fn set_treasury(env:&Env,v:i128){

    let key = PoolKey::Treasury;

    //Get current treasury 
    let x = get_treasury(env);

    //Update treasury value by new value
    let value = x.checked_add(v).expect("Overflow occurs");

    //Update treasury to storage
    env.storage().persistent().set(&key, &value);

}

//Function use for get treasury balance
pub(crate) fn get_treasury(env:&Env)->i128{

    let key = PoolKey::Treasury;

    //Fetch treasury balance and return balance
    env.storage().persistent().get::<PoolKey,i128>(&key).unwrap_or(0 as i128)
}


//Function use for change secondary mode status
pub(crate) fn set_in_secondary_mode(env:&Env,mode:bool){

    let key = PoolKey::InSecondaryMode;

    //Change mode and update to storage
    env.storage().persistent().set(&key, &mode);

}

//Function use for get secondary mode status
pub(crate) fn get_in_secondary_mode(env:&Env)-> bool{

    let key = PoolKey::InSecondaryMode;

    //Fetch secondary mode status and return
    env.storage().persistent().get(&key).unwrap_or(false)
}


//Function use for pvt_qty_max_primary on initialization
pub(crate) fn set_pvt_qty_max_primary_init(env:&Env,v:i128){

    let key = PoolKey::PvtQtyMaxPrimary;

    //Set pvt_qty_max_primary to storage
    env.storage().persistent().set(&key, &v);

}

//Function use for get pvt_qty_max_primary value
pub(crate) fn get_pvt_qty_max_primary(env:&Env)->i128{

    let key = PoolKey::PvtQtyMaxPrimary;

    //Fetch pvt_qty_max_primary value and return
    env.storage().persistent().get(&key).unwrap_or(0)
}

//Function use for set pvt_qty_max_secondary on initialization
pub(crate) fn set_pvt_qty_max_secondary_init(env:&Env,v:i128){

    let key = PoolKey::PvtQtyMaxSecondary;

    //Update pvt_qty_max_secondary value to storage
    env.storage().persistent().set(&key, &v);

}

//Function use for update pvt_qty_max_secondary
pub(crate) fn set_pvt_qty_max_secondary(env:&Env,v:i128){

    let key = PoolKey::PvtQtyMaxSecondary;

    //Fetch current pvt_qty_max_secondary value
    let x = get_pvt_qty_max_secondary(env);
    
    //Update value 
    let value = x.checked_add(v).expect("Overflow occurs");

    //Update pvt_qty_max_secondary to storage
    env.storage().persistent().set(&key, &value);

}

//Function use for get pvt_qty_max_secondary value
pub(crate) fn get_pvt_qty_max_secondary(env:&Env)->i128{

    let key = PoolKey::PvtQtyMaxSecondary;

    //Fetch pvt_qty_max_secondary value and return
    env.storage().persistent().get(&key).unwrap_or(0)
}

//Function use for set pvt_available_secondary on initialization 
pub(crate) fn set_pvt_available_secondary_init(env:&Env,v:i128){

    let key = PoolKey::PvtAvailableSecondary;

    //Update pvt_available_secondary to storage
    env.storage().persistent().set(&key, &v);

}

//Function use for get pvt_available_secondary value 
pub(crate) fn get_pvt_available_secondary(env:&Env)->i128{

    let key = PoolKey::PvtAvailableSecondary;

    //Fetch pvt_available_secondary value and return
    env.storage().persistent().get(&key).unwrap_or(0)
}


//Function use for update pvt_running_total_bought value
pub(crate) fn set_pvt_running_total_bought(env:&Env,v:i128){

    let key = PoolKey::PvtRunningTotalBought;

    //Fetch current pvt_running_total_bought value
    let x = get_pvt_running_total_bought(env);

    //Update pvt_running_total_bought by new value
    let value = x.checked_add(v).expect("Overflow occurs");

    //Update pvt_running_total_bought to storage
    env.storage().persistent().set(&key, &value);

}

//Function use for get pvt_running_total_bought  value
pub(crate) fn get_pvt_running_total_bought(env:&Env)->i128{

    let key = PoolKey::PvtRunningTotalBought;

    //Fetch pvt_running_total_bought value and return
    env.storage().persistent().get(&key).unwrap_or(0)
}


//Function use for update pvt_running_total_sold value
pub(crate) fn set_pvt_running_total_sold(env:&Env,v:i128){

    let key = PoolKey::PvtRunningTotalSold;

    //Fetch current pvt_running_total_sold value 
    let x = get_pvt_running_total_sold(env);

    //Update pvt_running_total_sold value by adding new value
    let value = x.checked_add(v).expect("Overflow occurs");

    //Update pvt_running_total_sold to storage
    env.storage().persistent().set(&key, &value);

}

//Function use to get pvt_running_total_sold value
pub(crate) fn get_pvt_running_total_sold(env:&Env)->i128{

    let key = PoolKey::PvtRunningTotalSold;

    //Update pvt_running_total_sold value and return
    env.storage().persistent().get(&key).unwrap_or(0)
}

//Function use for set pvt_price_initial_primary on initialization
pub(crate) fn set_pvt_price_initial_primary_init(env:&Env,v:i128){

    let key = PoolKey::PvtPriceInitialPrimary;

    //Update pvt_price_initial_primary to storage
    env.storage().persistent().set(&key, &v);

}

//Function use for get pvt_price_initial_primary value
pub(crate) fn get_pvt_price_initial_primary(env:&Env)->i128{

    let key = PoolKey::PvtPriceInitialPrimary;

    //Fetch pvt_price_initial_primary value and return
    env.storage().persistent().get(&key).unwrap_or(0)
}

//Function use for set pvt_price_max_primary value on initialization
pub(crate) fn set_pvt_price_max_primary_init(env:&Env,v:i128){

    let key = PoolKey::PvtPriceMaxPrimary;

    //Update pvt_price_max_primary value to storage
    env.storage().persistent().set(&key, &v);

}

//Function use for get pvt_price_max_primary value
pub(crate) fn get_pvt_price_max_primary(env:&Env)->i128{

    let key = PoolKey::PvtPriceMaxPrimary;

    //Fetch pvt_price_max_primary value and return
    env.storage().persistent().get(&key).unwrap_or(0)
}


//Function use for set pvt_price_max_secondary value on initialization
pub(crate) fn set_pvt_price_max_secondary_init(env:&Env,v:i128){

    let key = PoolKey::PvtPriceMaxSecondary;

    //Update pvt_price_max_secondary value to storage
    env.storage().persistent().set(&key, &v);

}

//Function use for get pvt_price_max_secondary value
pub(crate) fn get_pvt_price_max_secondary(env:&Env)->i128{

    let key = PoolKey::PvtPriceMaxSecondary;
    
    //Fetch pvt_price_max_secondary value and return
    env.storage().persistent().get(&key).unwrap_or(0)
}

//Function use for set a_primary_midpoint_initial_and_max on initialization
pub(crate) fn set_a_primary_midpoint_initial_and_max_init(env:&Env,pvt_price_max_primary:i128,pvt_price_initial_primary:i128){

    let key = PoolKey::APrimaryMidpointInitialAndMax;

    let a_primary_midpoint_initial_and_max = pvt_price_max_primary.checked_sub(pvt_price_initial_primary).expect("Underflow occur").checked_div(2).expect("Error");

    //Updated a_primary_midpoint_initial_and_max value to storage
    env.storage().persistent().set(&key, &a_primary_midpoint_initial_and_max);

}

//Function use for get a_primary_midpoint_initial_and_max value
pub(crate) fn get_a_primary_midpoint_initial_and_max(env:&Env)->i128{

    let key = PoolKey::APrimaryMidpointInitialAndMax;

    //Fetch a_primary_midpoint_initial_and_max value and return if value not set it will return default value
    env.storage().persistent().get(&key).unwrap_or(0)
}


//Function use for set b_primary_half_max_qty on initialization
pub(crate) fn set_b_primary_half_max_qty_init(env:&Env,pvt_qty_max_primary:i128){

    let key = PoolKey::BPrimaryHalfMaxQty;
    
    let b_primary_half_max_qty = pvt_qty_max_primary.checked_div(2).expect("Error");

    //Set b_primary_half_max_qty value to storage
    env.storage().persistent().set(&key, &b_primary_half_max_qty);

}


//Function use for get b_primary_half_max_qty value
pub(crate) fn get_b_primary_half_max_qty(env:&Env)->i128{

    let key = PoolKey::BPrimaryHalfMaxQty;
    
    //Fetch b_primary_half_max_qty value and return if value not set it will return default value
    env.storage().persistent().get(&key).unwrap_or(0)
}

//Function use for set c_primary_steepness on initialization
pub(crate) fn set_c_primary_steepness_init(env:&Env,steepness:u32){

    let key = PoolKey::CPrimarySteepness;

    //Set c_primary_steepness to storage
    env.storage().persistent().set(&key, &steepness);

}

//Function use for get c_primary_steepness value
pub(crate) fn get_c_primary_steepness(env:&Env)->u32{

    let key = PoolKey::CPrimarySteepness;

    //Fetch c_primary_steepness value and return if value not set it will return default value 
    env.storage().persistent().get(&key).unwrap_or(0)
}

//Function use for set a_secondary_midpoint_initial_and_max on initialization
pub(crate) fn set_a_secondary_midpoint_initial_and_max_init(env:&Env,pvt_price_max_secondary:i128,pvt_price_initial_primary:i128){

    let key = PoolKey::ASecondaryMidpointInitialAndMax;

    let a_secondary_midpoint_initial_and_max = pvt_price_max_secondary.checked_sub(pvt_price_initial_primary).expect("Underflow occur").checked_div(2).expect("Error");

    //Set a_secondary_midpoint_initial_and_max to storage
    env.storage().persistent().set(&key, &a_secondary_midpoint_initial_and_max);

}


//Function use for get a_secondary_midpoint_initial_and_max value
pub(crate) fn get_a_secondary_midpoint_initial_and_max(env:&Env)->i128{

    let key = PoolKey::ASecondaryMidpointInitialAndMax;

    //Fetch a_secondary_midpoint_initial_and_max value and return if value not set it will return default value 
    env.storage().persistent().get(&key).unwrap_or(0)
}


//Function use for set b_secondary_half_max_qty on initialization
pub(crate) fn set_b_secondary_half_max_qty_init(env:&Env,pvt_qty_max_primary:i128,pvt_qty_max_secondary:i128){

    let key = PoolKey::BSecondaryHalfMaxQty;

    let b_secondary_half_max_qty = pvt_qty_max_primary.checked_add(pvt_qty_max_secondary).expect("Overflow occur").checked_div(2).expect("Error");

    //Set b_secondary_half_max_qty to storage
    env.storage().persistent().set(&key, &b_secondary_half_max_qty);

}


//Function use for get b_secondary_half_max_qty value
pub(crate) fn get_b_secondary_half_max_qty(env:&Env)->i128{

    let key = PoolKey::BSecondaryHalfMaxQty;

    //Fetch b_secondary_half_max_qty value and return if value not set it will return default value 
    env.storage().persistent().get(&key).unwrap_or(0)
}

//Function use for set c_secondary_steepness on initialization
pub(crate) fn set_c_secondary_steepness_init(env:&Env,steepness:u32){

    let key = PoolKey::CSecondarySteepness;
    //Set c_secondary_steepness to storage
    env.storage().persistent().set(&key, &steepness);

}


//Function use for get c_secondary_steepness value
pub(crate) fn get_c_secondary_steepness(env:&Env)->u32{

    let key = PoolKey::CSecondarySteepness;

    //Fetch b_secondary_half_max_qty value and return if value not set it will return default value 
    env.storage().persistent().get(&key).unwrap_or(0)
}

//Function use for set p_prime on initialization
pub(crate) fn set_p_prime_init(env:&Env,pvt_price_initial_primary:i128){

    let key = PoolKey::PPrime;

    //We calculating value of unadjusted_price using unadjusted_price formula
    let unadjusted_price = get_unadjusted_price(env, 1);

    //Safely subtracted unadjusted_price value from pvt_price_initial_primary
    let p_prime = pvt_price_initial_primary.checked_sub(unadjusted_price).expect("Underflow");
    
    //Set p_prime to storage
    env.storage().persistent().set(&key, &p_prime);

}


//Function use for get p_prime  value
pub(crate) fn get_p_prime(env:&Env)->i128{

    let key = PoolKey::PPrime;

    //Fetch p_prime value and return if value not set it will return default value
    env.storage().persistent().get(&key).unwrap_or(0)
}


//Function use for set value of p_doubleprime
pub(crate) fn set_p_doubleprime(env:&Env){

    let key = PoolKey::PDoublePrime;

    //Get current value of p_doubleprime 
    let x = get_p_doubleprime(env);
    
    //Fetch b_secondary_half_max_qty value
    let b_secondary_half_max_qty = get_b_secondary_half_max_qty(env);

    //Fetch a_secondary_midpoint_initial_and_max value
    let a_secondary_midpoint_initial_and_max = get_a_secondary_midpoint_initial_and_max(env);

    //Get p_doubleprime value using get_price_primary formula
    let p_doubleprime = get_price_primary(env, (b_secondary_half_max_qty / (10 as i128).pow(9))).checked_sub(a_secondary_midpoint_initial_and_max).expect("Underflow");

    
    let value = x.checked_add(p_doubleprime).expect("Overflow occurs");

    //Set p_doubleprime to storage
    env.storage().persistent().set(&key, &value);

}

//Function use for get p_doubleprime value
pub(crate) fn get_p_doubleprime(env:&Env)->i128{

    let key = PoolKey::PDoublePrime;

    //Fetch p_doubleprime value and return if value not set it will return default value
    env.storage().persistent().get(&key).unwrap_or(0)
}

//Function use for set soldout_hits
pub(crate) fn set_soldout_hits(env:&Env,v:i128){

    let key = PoolKey::SoldOutHits;

    //Fetch current soldout_hits value
    let x = get_soldout_hits(env);
    
    let value = x.checked_add(v).expect("Overflow occurs");
    
    //Set soldout_hits to storage
    env.storage().persistent().set(&key, &value);

}

//Function use for get soldout_hits value
pub(crate) fn get_soldout_hits(env:&Env)->i128{

    let key = PoolKey::SoldOutHits;
    
    //Fetch soldout_hits value and return if value not set it will return default value
    env.storage().persistent().get(&key).unwrap_or(0)
}

//Function use for get unadjusted_price value
pub(crate) fn get_unadjusted_price(env:&Env,x:i128)->i128{

    //Fetch a_primary_midpoint_initial_and_max value
    let a_primary_midpoint_initial_and_max = get_a_primary_midpoint_initial_and_max(env);

    //Fetch b_primary_half_max_qty value
    let b_primary_half_max_qty = get_b_primary_half_max_qty(env);

    //Fetch c_primary_steepness
    let c_primary_steepness = get_c_primary_steepness(env);

    //Here we are finding square root
    //b_primary_half_max_qty is scaled value we need to upscale that value before we do subtraction from x 
    //x is also unscaled so we cannot subtract b_primary_half_max_qty(scaled value) from x
    //we are using fixed_mul_ceil function for safely calculates ceil(x * y / denominator). returns None if a phantom overflow occurs or if the denominator is 0
    //lastly we scaled value to 18 decimal point so we don't loss any precision and then we finding the square root of that scaled value 
    let sq = sqrt(((c_primary_steepness as i128) + (x - (b_primary_half_max_qty / (10 as i128).pow(9))).fixed_mul_ceil((x - (b_primary_half_max_qty / (10 as i128).pow(9))), 1).unwrap()) * (10 as i128).pow(18));
 
    //As above we need to scale x and b_primary_half_max_qty in to 18 decimal form because sq value is also in a form 18 decimal point
    let v = a_primary_midpoint_initial_and_max * ((((x - (b_primary_half_max_qty / (10 as i128).pow(9)))*(10 as i128).pow(18)) / sq) + 1000000000 );


    //Here we scale down the result into 9  decimal point
    v / (10 as i128).pow(9)
}


pub(crate) fn get_price_primary(env:&Env,x:i128)->i128{

    //Fetch a_primary_midpoint_initial_and_max value
    let a_primary_midpoint_initial_and_max = get_a_primary_midpoint_initial_and_max(env);

    //Fetch b_primary_half_max_qty value
    let b_primary_half_max_qty = get_b_primary_half_max_qty(env);

    //Fetch c_primary_steepness value
    let c_primary_steepness = get_c_primary_steepness(env);

    //Fetch p_prime value
    let p_prime = get_p_prime(env);


    //Here we are finding square root
    //b_primary_half_max_qty is scaled value we need to upscale that value before we do subtraction from x 
    //x is also unscaled so we cannot subtract b_primary_half_max_qty(scaled value) from x
    //we are using fixed_mul_ceil function for safely calculates ceil(x * y / denominator). returns None if a phantom overflow occurs or if the denominator is 0
    //lastly we scaled value to 18 decimal point so we don't loss any precision and then we finding the square root of that scaled value 
    let sq = sqrt(((c_primary_steepness as i128) + (x  - (b_primary_half_max_qty / Q9)).fixed_mul_ceil((x - (b_primary_half_max_qty / Q9)), 1).unwrap()) * (10 as i128).pow(18));

    //As above we need to scale x and b_primary_half_max_qty in to 18 decimal form because sq value is also in a form 18 decimal point
    let v = a_primary_midpoint_initial_and_max * ((((x - (b_primary_half_max_qty / (10 as i128).pow(9)))*(10 as i128).pow(18)) / sq) + 1000000000 );

    //Here we scale down the result into 9 decimal point and added p_prime which is also in form of 9 decimal
    v / (10 as i128).pow(9) + p_prime

}


pub(crate) fn get_price_secondary(env:&Env,x:i128)->i128{

    //Fetch a_secondary_midpoint_initial_and_max value
    let a_secondary_midpoint_initial_and_max = get_a_secondary_midpoint_initial_and_max(env);

    //Fetch b_secondary_half_max_qty value
    let b_secondary_half_max_qty = get_b_secondary_half_max_qty(env);

    //Fetch c_secondary_steepness value
    let c_secondary_steepness = get_c_secondary_steepness(env);

    //Fetch p_doubleprime value
    let p_doubleprime = get_p_doubleprime(env);

    //Here we are finding square root
    //b_secondary_half_max_qty is scaled value we need to upscale that value before we do subtraction from x 
    //x is also unscaled so we cannot subtract b_secondary_half_max_qty(scaled value) from x
    //we are using fixed_mul_ceil function for safely calculates ceil(x * y / denominator). returns None if a phantom overflow occurs or if the denominator is 0
    //lastly we scaled value to 18 decimal point so we don't loss any precision and then we finding the square root of that scaled value 
    let sq= sqrt(((c_secondary_steepness as i128) + (x - (b_secondary_half_max_qty / (10 as i128).pow(9))).fixed_mul_ceil((x - (b_secondary_half_max_qty / (10 as i128).pow(9))), 1).unwrap()) * (10 as i128).pow(18));

    //As above we need to scale x and b_secondary_half_max_qty in to 18 decimal form because sq value is also in a form 18 decimal point
    let v = a_secondary_midpoint_initial_and_max * ((((x - (b_secondary_half_max_qty / (10 as i128).pow(9)))*(10 as i128).pow(18)) / sq) + 1000000000 ) ;

    //Here we scale down the result into 9 decimal point and added p_doubleprime which is also in form of 9 decimal
    v / (10 as i128).pow(9) + p_doubleprime
}


//Function use for set PVT token address
pub(crate) fn set_pvt_address(env:&Env,token_address:Address){

    let key = PoolKey::TokenAddress;

    //Here we check can PVT token address already set or not
    let is_available = env.storage().instance().has(&key);

    //If already set it will panic other wise set to storage
    if is_available {
        panic!("Token address already set")
    }else{
        env.storage().instance().set(&key, &token_address);
    }

}

//Function use for set USDC token address
pub(crate) fn set_usdc_address(env:&Env,usdc_address:Address){

    let key = PoolKey::USDCAddress;

    //Here we check can PVT token address already set or not
    let is_available = env.storage().instance().has(&key);

    //If already set it will panic other wise set to storage
    if is_available {
        panic!("USDC address already set")
    }else{
        env.storage().instance().set(&key, &usdc_address);
    }

}

//Function use for get USDC token client
pub(crate) fn get_usdc_token(env:&Env)-> Client{
    let key = PoolKey::USDCAddress;

    //Fetch USDC token address
    let token_address:Address = env.storage().instance().get(&key).unwrap();

    //Created lptoken client using USDC token address
    lptoken::Client::new(env, &token_address)
}

//Function use for get PVT token client
pub(crate) fn get_pvt_token(env:&Env)-> Client{

    let key = PoolKey::TokenAddress;

    //Fetch PVT token address
    let token_address:Address = env.storage().instance().get(&key).unwrap();

    //Created lptoken client using PVT token address
    lptoken::Client::new(env, &token_address)
}



//Function use for switch secondary mode if condition meets
pub(crate) fn check_switch(env:&Env){
    let x = get_x(&env);
    let pvt_qty_max_primary = get_pvt_qty_max_primary(&env);
    //Here we scale down value because of x un scaled
    let pvt_qty_max_primary_unscaled = pvt_qty_max_primary / (10 as i128).pow(9);

    //If x == pvt_qty_max_primary_unscaled then we will change in_secondary_mode to true
    if x == pvt_qty_max_primary_unscaled {
        set_in_secondary_mode(env, true);
    }
}

//Function use for withdraw all fund
pub(crate) fn withdraw_all_fund(env:&Env,owner:Address){

    //Fetch current pool status
    let status = get_pool_status(env);

    //If pool status is 2(Stopped) then we will transfer all fund to owners account other wise it will panic
    if status == 2 {

        //Get current treasury balance
        let treasury = get_treasury(env);

        //Fetch USDC client
        let usdc = get_usdc_token(env);

        //Update treasury to zero
        set_treasury_to_zero(&env);

        //Transfer fund to owner account from contract
        usdc.transfer(&env.current_contract_address(), &owner, &treasury);

    }else{
        //If pool not stopped then it will panic
        panic!("You cannot withdraw fund before stop pool");
    }

    
}








