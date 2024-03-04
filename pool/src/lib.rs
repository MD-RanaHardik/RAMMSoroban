#![no_std]
mod lptoken;
mod storage;
mod test;

use soroban_fixed_point_math::{FixedPoint, STROOP};

use soroban_sdk::{
    contract, contractimpl, contractmeta, log, symbol_short,
    token::{self, TokenClient},
    Address, BytesN, ConversionError, Env, Map, String, Symbol, TryFromVal, Val,
};

use lptoken::create_contract;
use storage::{
    check_switch, get_in_secondary_mode, get_price_primary, get_price_secondary,
    get_pvt_available_secondary, get_pvt_qty_max_primary, get_pvt_qty_max_secondary,
    get_usdc_token, get_x, is_initialized,
    set_a_primary_midpoint_initial_and_max_init,
    set_a_secondary_midpoint_initial_and_max_init,
    set_b_primary_half_max_qty_init,
    set_b_secondary_half_max_qty_init, set_c_primary_steepness_init,
    set_c_secondary_steepness_init, set_owner,
    set_p_doubleprime, set_p_prime_init, set_pool_archive,
    set_pool_name, set_pvt_address, set_pvt_available_secondary_init,
    set_pvt_price_initial_primary_init,
    set_pvt_price_max_primary_init, set_pvt_price_max_secondary_init,
     set_pvt_qty_max_primary_init, set_pvt_qty_max_secondary,
    set_pvt_qty_max_secondary_init, set_pvt_running_total_bought,
     set_pvt_running_total_sold,
    set_soldout_hits, set_treasury, set_usdc_address,
    set_x, set_x_init, start_pool, stop_pool, withdraw_all_fund,
};

use crate::storage::{
    get_owner,
    get_pvt_token, get_treasury, is_pool_started,
};

const TOKEN: Symbol = symbol_short!("TOKEN");

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
        steepness: u32,
    ) {
        is_initialized(&env);

        set_pool_name(&env, pool_name);

        set_x_init(&env, 0);

        set_owner(&env, owner);

        set_pvt_qty_max_primary_init(&env, pvt_qty_max_primary);

        set_pvt_qty_max_secondary_init(&env, pvt_qty_max_secondary);

        set_pvt_available_secondary_init(&env, pvt_available_secondary);

        set_pvt_price_initial_primary_init(&env, pvt_price_initial_primary);

        set_pvt_price_max_primary_init(&env, pvt_price_max_primary);

        set_pvt_price_max_secondary_init(&env, pvt_price_max_secondary);

        set_a_primary_midpoint_initial_and_max_init(
            &env,
            pvt_price_max_primary,
            pvt_price_initial_primary,
        );

        set_b_primary_half_max_qty_init(&env, pvt_qty_max_primary);

        set_c_primary_steepness_init(&env, steepness);

        set_a_secondary_midpoint_initial_and_max_init(
            &env,
            pvt_price_max_secondary,
            pvt_price_initial_primary,
        );

        set_b_secondary_half_max_qty_init(&env, pvt_qty_max_primary, pvt_qty_max_secondary);

        set_c_secondary_steepness_init(&env, steepness);

        set_p_prime_init(&env, pvt_price_initial_primary);

        set_p_doubleprime(&env);

        let pvt_token_contract =
            create_contract(&env, token_wasm_hash.clone(), String::from_str(&env, "PVT"));

        lptoken::Client::new(&env, &pvt_token_contract.clone()).initialize(
            &env.current_contract_address(),
            &9u32,
            &String::from_str(&env, "PVT Token"),
            &String::from_str(&env, "PVT"),
        );

        set_pvt_address(&env, pvt_token_contract);

        // let usdc_token_contract = create_contract(&env, token_wasm_hash,String::from_str(&env, "USDC"));

        // lptoken::Client::new(&env, &usdc_token_contract.clone()).initialize(
        //         &env.current_contract_address(),
        //         &9u32,
        //         &String::from_str(&env, "USDC Token"),
        //         &String::from_str(&env,"USDC"),
        //     );

        // set_usdc_address(&env, usdc_token_contract);

        set_usdc_address(
            &env,
            Address::from_string(&String::from_str(
                &env,
                &"CB7XVGJGKZNHPAATSVP67VOOIYJ4EPQZ5IMSGWAGDHDO6JW4NRIA5UPU",
            )),
        );
    }

    pub fn expand(env: Env, owner: Address, amount: i128) -> (bool, i128) {
        owner.require_auth();

        match get_owner(&env) {
            Some(pool_owner) => {
                assert!(owner == pool_owner, "Wrong owner");
            }
            None => {
                panic!("Owner not set")
            }
        }

        let pvt_qty_max_secondary = get_pvt_qty_max_secondary(&env);
        let pvt_available_secondary = get_pvt_available_secondary(&env);

        let scaled_amount = amount * (10 as i128).pow(9);

        let left = pvt_qty_max_secondary
            .checked_add(scaled_amount)
            .expect("Overflow");

        let condition = left <= pvt_available_secondary;

        assert!(condition, "Quantity exceeding secondary capacity");

        set_pvt_qty_max_secondary(&env, amount);

        env.events()
            .publish((TOKEN, symbol_short!("Expanded")), scaled_amount);

        let pvt_qty_max_secondary = get_pvt_qty_max_secondary(&env);

        (condition, pvt_qty_max_secondary)
    }

    pub fn buy(env: Env, user: Address) -> (i128, i128) {
        user.require_auth();

        is_pool_started(&env);

        let in_secondary_mode = get_in_secondary_mode(&env);
        let x = get_x(&env);
        let pvt_qty_max_primary = get_pvt_qty_max_primary(&env);
        let pvt_qty_max_secondary = get_pvt_qty_max_secondary(&env);

        let mut price: i128 = 0;

        let Q9 = (10 as i128).pow(9);

        if (x * Q9)
            > pvt_qty_max_primary
                .checked_add(pvt_qty_max_secondary)
                .expect("Overflow")
        {
            set_soldout_hits(&env, 1);
            panic!("Maximum token sold");
        }

        if !in_secondary_mode {
            price = get_price_primary(&env, x.checked_add(1).expect("Overflow"));
        } else {
            price = get_price_secondary(&env, x.checked_add(1).expect("Overflow"));
        }

        // Transfer usdc to contract address
        let usdctoken = get_usdc_token(&env);

        let usdc_balance = usdctoken.balance(&user);

        assert!(usdc_balance >= price, "Insufficient balance");

        usdctoken.transfer(&user, &env.current_contract_address(), &price);

        set_treasury(&env, price);

        set_pvt_running_total_bought(&env, 1);

        // PVT token
        let pvttoken = get_pvt_token(&env);

        pvttoken.mint(&user, &(1 * &Q9));

        set_x(&env, 1);

        if !in_secondary_mode {
            check_switch(&env);
        }

        let currant_x = get_x(&env);
        let currant_treasury = get_treasury(&env);

        env.events().publish(
            (symbol_short!("RAMMBuy"), user, in_secondary_mode, currant_x),
            (price, (1 * Q9), currant_treasury),
        );

        (currant_x, currant_treasury)
    }

    pub fn sell(env: Env, user: Address) -> (bool, i128, i128) {
        user.require_auth();

        is_pool_started(&env);

        let in_secondary_mode = get_in_secondary_mode(&env);

        let Q9 = (10 as i128).pow(9);

        let x = get_x(&env);

        let mut price: i128 = 0;

        assert!(in_secondary_mode, "Not in secondary mode");

        price = get_price_secondary(&env, x.checked_sub(1).expect("Underflow"));

        let pvttoken = get_pvt_token(&env);
        let usdctoken = get_usdc_token(&env);

        let pvt_balance = pvttoken.balance(&user);

        assert!(pvt_balance >= (1 * Q9), "Insufficient balance");

        usdctoken.transfer(&env.current_contract_address(), &user, &price);

        set_pvt_running_total_sold(&env, 1);

        set_treasury(&env, -price);

        set_x(&env, -1);

        pvttoken.burn(&user, &(1 * &Q9));

        let currant_x = get_x(&env);
        let currant_treasury = get_treasury(&env);

        env.events().publish(
            (
                symbol_short!("RAMMSell"),
                user,
                in_secondary_mode,
                currant_x,
            ),
            (price, (1 * Q9), currant_treasury),
        );

        (in_secondary_mode, currant_x, currant_treasury)
    }

    pub fn simulate_buy_price(env: Env) -> i128 {
        let in_secondary_mode = get_in_secondary_mode(&env);
        let x = get_x(&env);
        let pvt_qty_max_primary = get_pvt_qty_max_primary(&env);
        let pvt_qty_max_secondary = get_pvt_qty_max_secondary(&env);

        let mut price: i128 = 0;

        let Q9 = (10 as i128).pow(9);

        if (x * Q9)
            > pvt_qty_max_primary
                .checked_add(pvt_qty_max_secondary)
                .expect("Overflow")
        {
            set_soldout_hits(&env, 1);
            panic!("Maximum token sold");
        }

        if !in_secondary_mode {
            price = get_price_primary(&env, x.checked_add(1).expect("Overflow"));
        } else {
            price = get_price_secondary(&env, x.checked_add(1).expect("Overflow"));
        }

        price
    }

    pub fn simulate_sell_price(env: Env) -> (bool,i128) {

        let in_secondary_mode = get_in_secondary_mode(&env);

        let x = get_x(&env);

        let mut price: i128 = 0;
        
        if in_secondary_mode {

            price = get_price_secondary(&env, x.checked_sub(1).expect("Underflow"));
        }


        (in_secondary_mode,price)
    }

    pub fn start(env: Env, owner: Address) {
        owner.require_auth();

        match get_owner(&env) {
            Some(pool_owner) => {
                assert!(owner == pool_owner, "Wrong owner");
            }
            None => {
                panic!("Owner not set")
            }
        }

        start_pool(&env);
    }

    pub fn stop(env: Env, owner: Address) {
        owner.require_auth();

        match get_owner(&env) {
            Some(pool_owner) => {
                assert!(owner == pool_owner, "Wrong owner");
            }
            None => {
                panic!("Owner not set")
            }
        }

        stop_pool(&env);
    }

    pub fn withdraw_fund(env: Env, owner: Address) {
        owner.require_auth();

        match get_owner(&env) {
            Some(pool_owner) => {
                assert!(owner == pool_owner, "Wrong owner");
            }
            None => {
                panic!("Owner not set")
            }
        }

        withdraw_all_fund(&env, owner);
    }

    pub fn archive_pool(env: Env, owner: Address) {
        owner.require_auth();

        match get_owner(&env) {
            Some(pool_owner) => {
                assert!(owner == pool_owner, "Wrong owner");
            }
            None => {
                panic!("Owner not set")
            }
        }

        set_pool_archive(&env, true);
    }

    pub fn unarchive_pool(env: Env, owner: Address) {
        owner.require_auth();

        match get_owner(&env) {
            Some(pool_owner) => {
                assert!(owner == pool_owner, "Wrong owner");
            }
            None => {
                panic!("Owner not set")
            }
        }

        set_pool_archive(&env, false);
    }

    pub fn balance(env: Env, user: Address) -> (i128, i128) {
        let usdc = get_usdc_token(&env);
        let pvt = get_pvt_token(&env);

        (usdc.balance(&user), pvt.balance(&user))
    }

    // pub fn mint_usdc_for_test(env:Env,user:Address){

    //     let usdc = get_usdc_token(&env);

    //     usdc.mint(&user, &1000000000000000);
    // }
}
