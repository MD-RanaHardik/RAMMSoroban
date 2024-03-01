// class Pool {
//     constructor(
//       poolName,
//       poolSeqId,
//       pvtQtyMaxPrimary,
//       pvtQtyMaxSecondary,
//       pvtPriceMaxPrimary,
//       pvtPriceMaxSecondary,
//       pvtPriceInitialPrimary,
//       pvtAvailableSecondary,
//       steepness,
//       owner
//     ) {
//       this.pvts = Array.from(
//         { length: pvtPriceMaxPrimary + pvtQtyMaxSecondary },
//         (_, n) => owner
//       );
      
//       this.poolId = 111111

//     //   this.poolId = crypto.subtle.digest('SHA-3-256', new TextEncoder().encode(uuid.v4())).then(
//     //     buffer => buffer.reduce((acc, val) => (acc << 8) | val, 0).toString(16)
//     //   );
//       this.poolName = poolName;
//       this.poolSeqId = poolSeqId;
  
//       this.x = 0;
//       this.archived = false;
//       this.owner = owner;
//       this.treasury = 0;
//       this.txSeqId = 1;
//       this.inSecondaryMode = false;
  
//       this.pvtQtyMaxPrimary = pvtQtyMaxPrimary;
//       this.pvtQtyMaxSecondary = pvtQtyMaxSecondary;
//       this.pvtAvailableSecondary = pvtAvailableSecondary;
//       this.pvtRunningTotalBought = 0;
//       this.pvtRunningTotalSold = 0;
//       this.pvtPriceInitialPrimary = pvtPriceInitialPrimary;
//       this.pvtPriceMaxPrimary = pvtPriceMaxPrimary;
//       this.pvtPriceMaxSecondary = pvtPriceMaxSecondary;
  
//       this.aPrimaryMidpointInitialAndMax = (pvtPriceMaxPrimary - pvtPriceInitialPrimary) / 2;
//       this.bPrimaryHalfMaxQty = pvtQtyMaxPrimary / 2;
//       this.cPrimarySteepness = steepness;
  
//       this.aSecondaryMidpointInitialAndMax = (pvtPriceMaxSecondary - pvtPriceInitialPrimary) / 2;
//       this.bSecondaryHalfMaxQty = (pvtQtyMaxPrimary + pvtQtyMaxSecondary) / 2;
//       this.cSecondarySteepness = steepness;
  
//       this.pPrime = this.pvtPriceInitialPrimary - this.getUnadjustedPrice(1);
//       this.pDoublePrime =
//         this.getPricePrimary(this.bSecondaryHalfMaxQty) - this.aSecondaryMidpointInitialAndMax;
  
//       this.soldoutHits = 0;
//       this.transactions = [];
//     }
  
//     getUnadjustedPrice(x) {
//       return (
//         this.aPrimaryMidpointInitialAndMax *
//         ((x - this.bPrimaryHalfMaxQty) /
//           Math.sqrt(this.cPrimarySteepness + (x - this.bPrimaryHalfMaxQty) * (x - this.bPrimaryHalfMaxQty))) +
//         1
//       );
//     }
  
//     getPricePrimary(x) {
//       return (
//         this.aPrimaryMidpointInitialAndMax *
//         ((x - this.bPrimaryHalfMaxQty) /
//           Math.sqrt(this.cPrimarySteepness + (x - this.bPrimaryHalfMaxQty) * (x - this.bPrimaryHalfMaxQty))) +
//         1 +
//         this.pPrime
//       );
//     }
  
//     // Other methods (if needed)
//   }


// let pol =new Pool("Test",
//     1,
//     1000,
//     1000,
//     5000,
//     7000,
//     3000,
//     2000,
//     1000,
//     0x111);
    

// let pol1 =new Pool("Test",
//     1,
//     1000,
//     1500,
//     5000,
//     7000,
//     3000,
//     2000,
//     100000,
//     0x111);
    
//     let pol2 =new Pool("Test",
//     1,
//     1000,
//     1900,
//     10,
//     19,
//     1,
//     1900,
//     100000,
//     0x111);
  
//     console.log(pol.getUnadjustedPrice(1));
//     console.log(pol.getPricePrimary(1));
    
//     console.log(pol1.getPricePrimary(1));
    
//      console.log(pol2.getPricePrimary(1));
    


class Pool {
  constructor(pool_name, pool_seq_id, pvt_qty_max_primary, pvt_qty_max_secondary,
      pvt_price_max_primary, pvt_price_max_secondary, pvt_price_initial_primary,
      pvt_available_secondary, steepness, owner) {
      this.pool_name = pool_name;
      this.pool_seq_id = pool_seq_id;
      this.x = 1;
      this.archived = false;
      this.owner = owner;
      this.treasury = 0;
      this.tx_seq_id = 1;
      this.in_secondary_mode = false;
      this.pvt_qty_max_primary = pvt_qty_max_primary;
      this.pvt_qty_max_secondary = pvt_qty_max_secondary;
      this.pvt_available_secondary = pvt_available_secondary;
      this.pvt_running_total_bought = 0;
      this.pvt_running_total_sold = 0;
      this.pvt_price_initial_primary = pvt_price_initial_primary;
      this.pvt_price_max_primary = pvt_price_max_primary;
      this.pvt_price_max_secondary = pvt_price_max_secondary;
      this.a_primary_midpoint_initial_and_max = (pvt_price_max_primary - pvt_price_initial_primary) / 2;
      this.b_primary_half_max_qty = pvt_qty_max_primary / 2;
      this.c_primary_steepness = steepness;
      this.a_secondary_midpoint_initial_and_max = (pvt_price_max_secondary - pvt_price_initial_primary) / 2;
      this.b_secondary_half_max_qty = (pvt_qty_max_primary + pvt_qty_max_secondary) / 2;
      this.c_secondary_steepness = steepness;
      console.log("Unadjusted price------------------",this.get_unadjusted_price(1));
      this.p_prime = this.pvt_price_initial_primary - this.get_unadjusted_price(1);
    //   console.log("Double prime",this.get_price_primary(this.b_secondary_half_max_qty));

      this.p_doubleprime = this.get_price_primary(this.b_secondary_half_max_qty) - this.a_secondary_midpoint_initial_and_max;
      this.soldout_hits = 0;
      this.transactions = [];
  }

  get_unadjusted_price(x) {

    let sqrt = Math.sqrt(this.c_primary_steepness + (x - this.b_primary_half_max_qty) *
    (x - this.b_primary_half_max_qty));

    // console.log("Sqrt ----",sqrt);

      return this.a_primary_midpoint_initial_and_max * ((x - this.b_primary_half_max_qty) /
          (sqrt) + 1);
  }

  get_price_primary(x) {

    let sqrt = Math.sqrt(this.c_primary_steepness + (x - this.b_primary_half_max_qty) *
    (x - this.b_primary_half_max_qty));

    // console.log("D Prime SQRT X",x);
    // console.log("D Prime SQRT",sqrt);

      return this.a_primary_midpoint_initial_and_max * ((x - this.b_primary_half_max_qty) /
          (sqrt) + 1) + this.p_prime;
  }

  get_price_secondary(x) {
      return this.a_secondary_midpoint_initial_and_max * ((x - this.b_secondary_half_max_qty) /
          (Math.sqrt(this.c_secondary_steepness + (x - this.b_secondary_half_max_qty) *
              (x - this.b_secondary_half_max_qty))) + 1) + this.p_doubleprime;
  }

  buy(y) {
    let price;
      if (!this.in_secondary_mode) {
          if (this.x > (this.pvt_qty_max_primary + this.pvt_qty_max_secondary)) {
              this.soldout_hits += 1;
              return; // soldout
          }
          price = this.get_price_primary(this.x + y);
          
      } else {
          price = this.get_price_secondary(this.x + y);
          
      }

      console.log("Buy Price ",price);

      this.treasury += price;
      this.pvt_running_total_bought += y;
      this.x += y;
      if (!this.in_secondary_mode) this.check_switch();

      // let counterparty = Wallet.get_random_wallet();
      // while (counterparty.usdc_balance < price) {
      //     counterparty = Wallet.get_random_wallet();
      // }

      // counterparty.pvt_balance += 1;
      // counterparty.usdc_balance -= price;
      

  }

  sell() {
      if (this.in_secondary_mode) {
            console.log("XXXXXXXX",this.x - 1);
          let price = this.get_price_secondary(this.x - 1);
          this.pvt_running_total_sold += 1;
          this.x -= 1;
          this.treasury -= price;

          console.log("Sell Price ",price);
          // let counterparty = Wallet.get_random_wallet();
          // while (counterparty.pvt_balance <= 0) {
          //     counterparty = Wallet.get_random_wallet();
          // }
          // counterparty.pvt_balance -= 1;
          // counterparty.usdc_balance += price;    
      }
  }


  check_switch() {
    if (this.x == this.pvt_qty_max_primary) {
        this.in_secondary_mode = true;
    }
  }

  get_unadjusted_price1() {
    return 1000 * ((1 - 500) /
        (Math.sqrt(1000 + (1 - 500) *
            (1 - 500))) + 1);
}

}


let pool =new Pool("Test",
    1,
    2,
    2,
    5000,
    7000,
    3000,
    4,
    1000,
    0x111);

// let pool1 =new Pool("Test",
//     1,
//     3,
//     3,
//     5000,
//     7000,
//     3000,
//     6,
//     1000,
//     0x111);

// pool.buy(1);
pool.buy(1);
pool.buy(1);
pool.buy(1);
pool.sell();

// console.log(pool.get_price_primary(3));
// pool.buy(1);
// pool.buy(1);
// pool.buy(2);
// pool.buy(2);
// pool.buy();

// pool1.buy();
// pool1.buy();
// pool1.buy();
// pool1.sell();
// pool1.sell();

    // for (let index = 0; index < 1000; index++) {
    //   pool.buy()
    // }

    // pool.sell();

    // console.log(pool.x);
    // console.log(pool.treasury);

    // let d = [];

    // for (let index = 0; index < 10; index++) {
      
    //   pool.sell()
    //   // console.log("+++++++++++++++++++++++++++++")
    //   // console.log("Sell",index)
    //   // console.log(pool.x);
    //   // console.log(pool.treasury);
    //   // console.log("+++++++++++++++++++++++++++++")

    //   d.push(pool.treasury);
    // }

    // d.map((e,i)=>{

    //   if(i != d.length){
    //     console.log("+++++++++++++++++++++++++++++")
    //     console.log(d[i]- d[i+1])
    //     console.log("+++++++++++++++++++++++++++++")
    //   }
      

    //   // console.log("+++++++++++++++++++++++++++++")
    //   // console.log()
    //   // console.log("+++++++++++++++++++++++++++++")
    // });


    // pool.buy()

    // console.log(pool.x);
    // console.log(pool.treasury);

    // pool.sell()

    // console.log(pool.x);
    // console.log(pool.treasury);


    // console.log(pool.get_unadjusted_price1());


    
    
    
    
    
   
