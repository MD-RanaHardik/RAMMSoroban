#![no_std]
mod lptoken;
mod storage;
mod test;

use soroban_sdk::{
    contract,
    contractimpl,
    contractmeta,
    symbol_short,
    Address,
    BytesN,
    ConversionError,
    Env,
    String,
    Symbol,
    TryFromVal,
    Val,
};

use lptoken::create_contract;
use storage::{
    check_switch,
    get_in_secondary_mode,
    get_price_primary,
    get_price_secondary,
    get_pvt_available_secondary,
    get_pvt_qty_max_primary,
    get_pvt_qty_max_secondary,
    get_usdc_token,
    get_x,
    is_initialized,
    set_a_primary_midpoint_initial_and_max_init,
    set_a_secondary_midpoint_initial_and_max_init,
    set_b_primary_half_max_qty_init,
    set_b_secondary_half_max_qty_init,
    set_c_primary_steepness_init,
    set_c_secondary_steepness_init,
    set_owner,
    set_p_doubleprime,
    set_p_prime_init,
    set_pool_archive,
    set_pool_name,
    set_pvt_address,
    set_pvt_available_secondary_init,
    set_pvt_price_initial_primary_init,
    set_pvt_price_max_primary_init,
    set_pvt_price_max_secondary_init,
    set_pvt_qty_max_primary_init,
    set_pvt_qty_max_secondary,
    set_pvt_qty_max_secondary_init,
    set_pvt_running_total_bought,
    set_pvt_running_total_sold,
    set_soldout_hits,
    set_treasury,
    set_usdc_address,
    set_x,
    set_x_init,
    start_pool,
    stop_pool,
    withdraw_all_fund,
};

use crate::storage::{ get_owner, get_pvt_token, get_treasury, is_pool_started };

const TOKEN: Symbol = symbol_short!("TOKEN");

//Storage which used for storage all required value particular pool
#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum PoolKey {
    IsInitialized = 0,
    PoolName = 1,
    Owner = 2,
    Archived = 3,
    Treasury = 4,
    InSecondaryMode = 5,
    PvtQtyMaxPrimary = 6,
    PvtQtyMaxSecondary = 7,
    PvtAvailableSecondary = 8,
    PvtRunningTotalBought = 9,
    PvtRunningTotalSold = 10,
    PvtPriceInitialPrimary = 11,
    PvtPriceMaxPrimary = 12,
    PvtPriceMaxSecondary = 13,
    APrimaryMidpointInitialAndMax = 14,
    BPrimaryHalfMaxQty = 15,
    CPrimarySteepness = 16,
    ASecondaryMidpointInitialAndMax = 17,
    BSecondaryHalfMaxQty = 18,
    CSecondarySteepness = 19,
    PPrime = 20,
    PDoublePrime = 21,
    SoldOutHits = 22,
    X = 23,
    TokenAddress = 24,
    USDCAddress = 25,
    PoolStatus = 26,
}

impl TryFromVal<Env, PoolKey> for Val {
    type Error = ConversionError;

    fn try_from_val(_env: &Env, v: &PoolKey) -> Result<Self, Self::Error> {
        Ok((*v as u32).into())
    }
}

// Metadata that is added on to the WASM custom section
contractmeta!(key = "Pool Smart Contract", val = "Bonding Curve");

#[contract]
pub struct Pool;

#[contractimpl]
impl Pool {
    //Function helps to initialize newly created pool
    pub fn init(
        env: Env,
        token_wasm_hash: BytesN<32>,
        owner: Address,
        pool_name: String,
        pvt_qty_max_primary: i128,
        pvt_qty_max_secondary: i128,
        pvt_price_max_primary: i128,
        pvt_price_max_secondary: i128,
        pvt_price_initial_primary: i128,
        pvt_available_secondary: i128,
        steepness: u32
    ) {
        //Check whether pool already initialized or not
        is_initialized(&env);

        //Set pool name
        set_pool_name(&env, pool_name);

        //Set x to value to 0
        set_x_init(&env, 0);

        //Set owner of pool
        set_owner(&env, owner);

        //Set pvt_qty_max_primary to storage
        set_pvt_qty_max_primary_init(&env, pvt_qty_max_primary);

        //Set pvt_qty_max_secondary to storage
        set_pvt_qty_max_secondary_init(&env, pvt_qty_max_secondary);

        //Set pvt_available_secondary to storage
        set_pvt_available_secondary_init(&env, pvt_available_secondary);

        //Set pvt_price_initial_primary to storage
        set_pvt_price_initial_primary_init(&env, pvt_price_initial_primary);

        //Set pvt_price_max_primary to storage
        set_pvt_price_max_primary_init(&env, pvt_price_max_primary);

        //Set pvt_price_max_secondary to storage
        set_pvt_price_max_secondary_init(&env, pvt_price_max_secondary);

        //Set a_primary_midpoint_initial_and_max to storage
        set_a_primary_midpoint_initial_and_max_init(
            &env,
            pvt_price_max_primary,
            pvt_price_initial_primary
        );

        //Set b_primary_half_max_qty to storage
        set_b_primary_half_max_qty_init(&env, pvt_qty_max_primary);

        //Set c_primary_steepness to storage
        set_c_primary_steepness_init(&env, steepness);

        //Set a_secondary_midpoint_initial_and_max to storage
        set_a_secondary_midpoint_initial_and_max_init(
            &env,
            pvt_price_max_secondary,
            pvt_price_initial_primary
        );

        //Set b_secondary_half_max_qty to storage
        set_b_secondary_half_max_qty_init(&env, pvt_qty_max_primary, pvt_qty_max_secondary);

        //Set c_secondary_steepness to storage
        set_c_secondary_steepness_init(&env, steepness);

        //Set p_prime to storage
        set_p_prime_init(&env, pvt_price_initial_primary);

        //Set p_doubleprime to storage
        set_p_doubleprime(&env);

        //Deployed new pvt token smart contract from token wasm hash which returns token address
        let pvt_token_contract = create_contract(
            &env,
            token_wasm_hash.clone(),
            String::from_str(&env, "PVT")
        );

        //Creates client from newly deployed contract address and initialize that token
        lptoken::Client
            ::new(&env, &pvt_token_contract.clone())
            .initialize(
                &env.current_contract_address(),
                &9u32,
                &String::from_str(&env, "PVT Token"),
                &String::from_str(&env, "PVT")
            );

        //Set pvt token address to storage
        set_pvt_address(&env, pvt_token_contract);

        //Uncomment for test

        // let usdc_token_contract = create_contract(&env, token_wasm_hash,String::from_str(&env, "USDC"));

        // lptoken::Client::new(&env, &usdc_token_contract.clone()).initialize(
        //         &env.current_contract_address(),
        //         &9u32,
        //         &String::from_str(&env, "USDC Token"),
        //         &String::from_str(&env,"USDC"),
        //     );

        // set_usdc_address(&env, usdc_token_contract);

        //Set usdc token address to storage
        set_usdc_address(
            &env,
            Address::from_string(
                &String::from_str(&env, &"CB7XVGJGKZNHPAATSVP67VOOIYJ4EPQZ5IMSGWAGDHDO6JW4NRIA5UPU")
            )
        );
    }

    //Function helps to expand pvt_qty_max_secondary
    pub fn expand(env: Env, owner: Address, amount: i128) -> (bool, i128) {

        //Check owner authenticity
        owner.require_auth();

        //Check whether the caller is owner or not
        match get_owner(&env) {
            Some(pool_owner) => {
                //If not owner then it will panic
                assert!(owner == pool_owner, "Wrong owner");
            }
            None => {
                panic!("Owner not set");
            }
        }

        let pvt_qty_max_secondary = get_pvt_qty_max_secondary(&env);
        let pvt_available_secondary = get_pvt_available_secondary(&env);

        //Safely add pvt_qty_max_secondary and amount
        let left = pvt_qty_max_secondary.checked_add(amount).expect("Overflow");

        let condition = left <= pvt_available_secondary;

        //If new qty gather then max it will panic
        assert!(condition, "Quantity exceeding secondary capacity");

        //Update pvt_qty_max_secondary
        set_pvt_qty_max_secondary(&env, amount);

        //Publish new event
        env.events().publish((TOKEN, symbol_short!("Expanded")), amount);

        //Get new pvt_qty_max_secondary
        let pvt_qty_max_secondary = get_pvt_qty_max_secondary(&env);

        //Return condition whether expanded or not and also returned 
        (condition, pvt_qty_max_secondary)
    }

    //Function helps to buy PVT token in exchange of USDC
    pub fn buy(env: Env, user: Address) -> (i128, i128) {

        //Check user authenticity
        user.require_auth();

        //Check pool started or not if not it will panic
        is_pool_started(&env);

        let in_secondary_mode = get_in_secondary_mode(&env);
        let x = get_x(&env);
        let pvt_qty_max_primary = get_pvt_qty_max_primary(&env);
        let pvt_qty_max_secondary = get_pvt_qty_max_secondary(&env);

        let mut price: i128 = 0;

        let Q9 = (10 as i128).pow(9);

        //If PVT token qty exceeded max limit it will panic 
        if x * Q9 >= pvt_qty_max_primary.checked_add(pvt_qty_max_secondary).expect("Overflow") {
            set_soldout_hits(&env, 1);
            panic!("Maximum token sold");
        }

        //If pool not in secondary then we will calculate price using primary formula other wise using secondary formula  
        if !in_secondary_mode {
            price = get_price_primary(&env, x.checked_add(1).expect("Overflow"));
        } else {
            price = get_price_secondary(&env, x.checked_add(1).expect("Overflow"));
        }

        // Transfer usdc to contract address
        let usdctoken = get_usdc_token(&env);

        //Get USDC balance
        let usdc_balance = usdctoken.balance(&user);

        //Check whether user have sufficient USDC or not
        assert!(usdc_balance >= price, "Insufficient balance");

        //Transfer USDC to contract
        usdctoken.transfer(&user, &env.current_contract_address(), &price);

        //Update treasury 
        set_treasury(&env, price);

        //Update pvt_running_total_bought
        set_pvt_running_total_bought(&env, 1);

        //Get PVT token client
        let pvttoken = get_pvt_token(&env);

        //Mint 1 PVT token to user account
        pvttoken.mint(&user, &(1 * &Q9));

        //Update X
        set_x(&env, 1);

        //if pool not in secondary then check for whether x equals to pvt_qty_max_primary or not
        if !in_secondary_mode {
            check_switch(&env);
        }

        let current_x = get_x(&env);
        let current_treasury = get_treasury(&env);

        //Publish event for buy pvt token
        env.events().publish(
            (symbol_short!("RAMMBuy"), user, in_secondary_mode, current_x),
            (price, 1 * Q9, current_treasury)
        );

        //Return current x and treasury
        (current_x, current_treasury)
    }

    //Function helps to sell PVT token in exchange of USDC
    pub fn sell(env: Env, user: Address) -> (bool, i128, i128) {

        //Check user authenticity
        user.require_auth();

        //Check pool started or not if not it will panic
        is_pool_started(&env);

        let in_secondary_mode = get_in_secondary_mode(&env);

        let Q9 = (10 as i128).pow(9);

        let x = get_x(&env);

        let mut price: i128 = 0;

        //Check whether pool in secondary or not if not in secondary mode then it will panic
        assert!(in_secondary_mode, "Not in secondary mode");

        //If X value less then 0 then it will panic
        assert!(x > 0, "Sell is disabled");

        //Get sell price based on get_price_secondary formula
        price = get_price_secondary(&env, x.checked_sub(1).expect("Underflow"));

        //Get PVT token client
        let pvttoken = get_pvt_token(&env);

        //Get USDC token client
        let usdctoken = get_usdc_token(&env);

        //Get PTV token balance
        let pvt_balance = pvttoken.balance(&user);

        //Check user have sufficient balance or not
        assert!(pvt_balance >= 1 * Q9, "Insufficient balance");

        //Transfer USDC to user from contract
        usdctoken.transfer(&env.current_contract_address(), &user, &price);

        //Update pvt_running_total_sold
        set_pvt_running_total_sold(&env, 1);
        
        //Update treasury
        set_treasury(&env, -price);

        //Update X
        set_x(&env, -1);

        //Burn 1 PVT token from user account
        pvttoken.burn(&user, &(1 * &Q9));

        let current_x = get_x(&env);
        let current_treasury = get_treasury(&env);

        //Publish sell event
        env.events().publish(
            (symbol_short!("RAMMSell"), user, in_secondary_mode, current_x),
            (price, 1 * Q9, current_treasury)
        );

        //Return status of secondary mode and current x and treasury
        (in_secondary_mode, current_x, current_treasury)
    }

    //Function give us price of USDC for buy
    pub fn simulate_buy_price(env: Env) -> i128 {

        let in_secondary_mode = get_in_secondary_mode(&env);
        let x = get_x(&env);
        let pvt_qty_max_primary = get_pvt_qty_max_primary(&env);
        let pvt_qty_max_secondary = get_pvt_qty_max_secondary(&env);

        let mut price: i128 = 0;

        let Q9 = (10 as i128).pow(9);

        //Panic if token bought more then max limit
        assert!(
            x * Q9 < pvt_qty_max_primary.checked_add(pvt_qty_max_secondary).expect("Overflow"),
            "Maximum token sold"
        );

        //It will return price based on pool secondary mode
        if !in_secondary_mode {
            price = get_price_primary(&env, x.checked_add(1).expect("Overflow"));
        } else {
            price = get_price_secondary(&env, x.checked_add(1).expect("Overflow"));
        }

        //Return price
        price
    }

    //Function give us price of USDC for sell
    pub fn simulate_sell_price(env: Env) -> (bool, i128) {
        let in_secondary_mode = get_in_secondary_mode(&env);

        let x = get_x(&env);

        let mut price: i128 = 0;

        //If pool have 0 PVT token then it will panic
        assert!(x > 0, "Sell is disabled");

        //If pool in secondary mode then it will calculate price base on get_price_secondary formula
        if in_secondary_mode {
            price = get_price_secondary(&env, x.checked_sub(1).expect("Underflow"));
        }

        //Return in_secondary_mode status and price
        (in_secondary_mode, price)
    }

    //Function for start pool
    pub fn start(env: Env, owner: Address) {
        //Check owner authenticity
        owner.require_auth();

        //Check whether the caller is owner or not
        match get_owner(&env) {
            Some(pool_owner) => {
                //If not owner it will panic
                assert!(owner == pool_owner, "Wrong owner");
            }
            None => {
                panic!("Owner not set");
            }
        }

        //Update pool status
        start_pool(&env);
    }

    //Function for stop pool
    pub fn stop(env: Env, owner: Address) {

        //Check owner authenticity
        owner.require_auth();

        //Check whether the caller is owner or not
        match get_owner(&env) {
            Some(pool_owner) => {
                
                //If not owner it will panic
                assert!(owner == pool_owner, "Wrong owner");
            }
            None => {
                panic!("Owner not set");
            }
        }

        //Update pool status
        stop_pool(&env);
    }

    //Function use for withdraw pool fund
    pub fn withdraw_fund(env: Env, owner: Address) {

        //Check owner authenticity
        owner.require_auth();

        //Check whether the caller is owner or not
        match get_owner(&env) {
            Some(pool_owner) => {
                //If not owner it will panic
                assert!(owner == pool_owner, "Wrong owner");
            }
            None => {
                panic!("Owner not set");
            }
        }

        //Update treasury balance and transfer USDC to owner
        withdraw_all_fund(&env, owner);
    }

    //Function use for archive pool
    pub fn archive_pool(env: Env, owner: Address) {

        //Check owner authenticity
        owner.require_auth();

        //Check whether the caller is owner or not
        match get_owner(&env) {
            Some(pool_owner) => {
                //If not owner it will panic
                assert!(owner == pool_owner, "Wrong owner");
            }
            None => {
                panic!("Owner not set");
            }
        }

        //Update pool archive status to true
        set_pool_archive(&env, true);
    }

    //Function use for unarchive pool
    pub fn unarchive_pool(env: Env, owner: Address) {
        
        //Check owner authenticity
        owner.require_auth();

        //Check whether the caller is owner or not
        match get_owner(&env) {
            Some(pool_owner) => {
                //If not owner it will panic
                assert!(owner == pool_owner, "Wrong owner");
            }
            None => {
                panic!("Owner not set");
            }
        }

        //Update pool archive status to false
        set_pool_archive(&env, false);
    }

    //Function use for get USDC and PVT token balance
    pub fn balance(env: Env, user: Address) -> (i128, i128) {

        //Get USDC token client
        let usdc = get_usdc_token(&env);
        //Get PVT token client
        let pvt = get_pvt_token(&env);

        //Return USDC and PVT balance
        (usdc.balance(&user), pvt.balance(&user))
    }

    // pub fn mint_usdc_for_test(env:Env,user:Address){

    //     let usdc = get_usdc_token(&env);

    //     usdc.mint(&user, &1000000000000000);
    // }
}
