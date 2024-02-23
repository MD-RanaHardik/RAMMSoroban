class Pool {
    constructor(
      poolName,
      poolSeqId,
      pvtQtyMaxPrimary,
      pvtQtyMaxSecondary,
      pvtPriceMaxPrimary,
      pvtPriceMaxSecondary,
      pvtPriceInitialPrimary,
      pvtAvailableSecondary,
      steepness,
      owner
    ) {
      this.pvts = Array.from(
        { length: pvtPriceMaxPrimary + pvtQtyMaxSecondary },
        (_, n) => owner
      );
      
      this.poolId = 111111

    //   this.poolId = crypto.subtle.digest('SHA-3-256', new TextEncoder().encode(uuid.v4())).then(
    //     buffer => buffer.reduce((acc, val) => (acc << 8) | val, 0).toString(16)
    //   );
      this.poolName = poolName;
      this.poolSeqId = poolSeqId;
  
      this.x = 0;
      this.archived = false;
      this.owner = owner;
      this.treasury = 0;
      this.txSeqId = 1;
      this.inSecondaryMode = false;
  
      this.pvtQtyMaxPrimary = pvtQtyMaxPrimary;
      this.pvtQtyMaxSecondary = pvtQtyMaxSecondary;
      this.pvtAvailableSecondary = pvtAvailableSecondary;
      this.pvtRunningTotalBought = 0;
      this.pvtRunningTotalSold = 0;
      this.pvtPriceInitialPrimary = pvtPriceInitialPrimary;
      this.pvtPriceMaxPrimary = pvtPriceMaxPrimary;
      this.pvtPriceMaxSecondary = pvtPriceMaxSecondary;
  
      this.aPrimaryMidpointInitialAndMax = (pvtPriceMaxPrimary - pvtPriceInitialPrimary) / 2;
      this.bPrimaryHalfMaxQty = pvtQtyMaxPrimary / 2;
      this.cPrimarySteepness = steepness;
  
      this.aSecondaryMidpointInitialAndMax = (pvtPriceMaxSecondary - pvtPriceInitialPrimary) / 2;
      this.bSecondaryHalfMaxQty = (pvtQtyMaxPrimary + pvtQtyMaxSecondary) / 2;
      this.cSecondarySteepness = steepness;
  
      this.pPrime = this.pvtPriceInitialPrimary - this.getUnadjustedPrice(1);
      this.pDoublePrime =
        this.getPricePrimary(this.bSecondaryHalfMaxQty) - this.aSecondaryMidpointInitialAndMax;
  
      this.soldoutHits = 0;
      this.transactions = [];
    }
  
    getUnadjustedPrice(x) {
      return (
        this.aPrimaryMidpointInitialAndMax *
        ((x - this.bPrimaryHalfMaxQty) /
          Math.sqrt(this.cPrimarySteepness + (x - this.bPrimaryHalfMaxQty) * (x - this.bPrimaryHalfMaxQty))) +
        1
      );
    }
  
    getPricePrimary(x) {
      return (
        this.aPrimaryMidpointInitialAndMax *
        ((x - this.bPrimaryHalfMaxQty) /
          Math.sqrt(this.cPrimarySteepness + (x - this.bPrimaryHalfMaxQty) * (x - this.bPrimaryHalfMaxQty))) +
        1 +
        this.pPrime
      );
    }
  
    // Other methods (if needed)
  }


let pol =new Pool("Test",
    1,
    1000,
    1000,
    10,
    20,
    1,
    1000,
    2,
    0x111);
  
    console.log(pol.getUnadjustedPrice(1));
    console.log(pol.getPricePrimary(1));
    
    
    
    
    
    
    
   
