"use client";
import React, { useContext, useEffect, useState } from "react";
import { Context, server } from "../Context/store";
import { createPool } from "../soroban/createPool";
import { AiOutlineLoading3Quarters } from "react-icons/ai";
import { getNetwork } from "@stellar/freighter-api";
import { SUPPORTED_NETWORK, SUPPORTED_NETWORK1 } from "../soroban/default_data";

const Steepness:{[key:string]:number} = {
  "STEEPNESS_FLATISH":10000000,
  "STEEPNESS_MODERATE":1000000,
  "STEEPNESS_MEDIUM":100000,
  "STEEPNESS_HIGH":10000,
  "STEEPNESS_AGGRESSIVE":1000
}

export default function Page() {
  const { activePubkey, setActivePubKey, walletConnectKit, showToast } = useContext(Context);

  const [poolName, setPoolName] = useState<string | undefined>();
  const [maxPrimaryQuantity, setMaxPrimaryQuantity] = useState<number | undefined>();
  const [maxPrimaryPrice, setMaxPrimaryPrice] = useState<number | undefined>();
  const [secondaryAvailable, setSecondaryAvailable] = useState<number | undefined>();
  const [initialPrimaryPrice, setInitialPrimaryPrice] = useState<number | undefined>();

  const [loader, setLoader] = useState<boolean>(false);

  const [isWrongNetwork, setIsWrongNetwork] = useState<boolean>(false);

  const [selectedSteepness, setSelectedSteepness] = useState("STEEPNESS_FLATISH");

  const handleSteepnessChange = (event:any) => {
    setSelectedSteepness(event.target.value);
  };

  console.log(selectedSteepness, "selectedSteepness");

  useEffect(() => {
    getConnectedNetwork();
  }, [activePubkey]);

  // --- This function will get connected network's details
  async function getConnectedNetwork() {
    // get network details
    let network = await getNetwork();
    console.log(network);

    if (network == SUPPORTED_NETWORK || network == SUPPORTED_NETWORK1) {
      setIsWrongNetwork(false);
    } else {
      setIsWrongNetwork(true);
    }
  }

  // --- This function will create a new pool.
  async function createNewPool() {
    setLoader(true);

    if (walletConnectKit && poolName != undefined && maxPrimaryQuantity != undefined && maxPrimaryPrice != undefined && secondaryAvailable != undefined && initialPrimaryPrice != undefined) {
      // Create pool using required parameters
      await createPool(server, walletConnectKit, poolName,maxPrimaryQuantity,maxPrimaryPrice,secondaryAvailable,initialPrimaryPrice,Steepness[selectedSteepness])
        .then((e) => {
          showToast("Pool created");
        })
        .catch((e) => {
          showToast("Something went wrong");
        });
    }

    setLoader(false);
  }

  return (
    <div>
      <div className="ring-1 ring-slate-100 shadow-md w-1/2 rounded-lg py-10 px-6 ml-auto mr-auto mt-16 mb-20">
        <h1 className="text-2xl font-bold mb-3 ml-1">Create Pool</h1>

        <input
          placeholder="Pool Name"
          value={poolName}
          onChange={(e) => {
            setPoolName(e.target.value);
          }}
          className="bg-slate-100 py-3 w-full rounded-md px-2"
        ></input>

        <input
          placeholder="Max Primary Quantity"
          value={maxPrimaryQuantity}
          onChange={(e) => {
            const inputValue = parseInt(e.target.value, 10);
            setMaxPrimaryQuantity(isNaN(inputValue) ? undefined : inputValue);
          }}
          className="bg-slate-100 py-3 w-full rounded-md px-2 mt-5"
        ></input>

        <input
          placeholder="Max Primary Price"
          value={maxPrimaryPrice}
          onChange={(e) => {
            const inputValue = parseInt(e.target.value, 10);
            setMaxPrimaryPrice(isNaN(inputValue) ? undefined : inputValue);
          }}
          className="bg-slate-100 py-3 w-full rounded-md px-2 mt-5"
        ></input>

        <input
          placeholder="Secondary Available"
          value={secondaryAvailable}
          onChange={(e) => {
            const inputValue = parseInt(e.target.value, 10);
            setSecondaryAvailable(isNaN(inputValue) ? undefined : inputValue);

          }}
          className="bg-slate-100 py-3 w-full rounded-md px-2 mt-5"
        ></input>

        <input
          placeholder="Initial Primary Price"
          value={initialPrimaryPrice}
          type="number"
          step="0.1"
          onChange={(e) => {
            setInitialPrimaryPrice(Number(e.target.value));
          }}
          className="bg-slate-100 py-3 w-full rounded-md px-2 mt-5"
        ></input>

        <div className="mt-3">
          <label className="block text-md p-2" htmlFor="steepnessDropdown">
            Steepness
          </label>
          <select
            id="steepnessDropdown"
            className="appearance-none border bg-slate-100 py-3 w-full rounded-md px-2 leading-tight focus:outline-none focus:shadow-outline"
            value={selectedSteepness}
            onChange={handleSteepnessChange}
          >
            <option value="STEEPNESS_FLATISH" >STEEPNESS_FLATISH</option>
            <option value="STEEPNESS_MODERATE">STEEPNESS_MODERATE</option>
            <option value="STEEPNESS_MEDIUM">STEEPNESS_MEDIUM</option>
            <option value="STEEPNESS_HIGH">STEEPNESS_HIGH</option>
            <option value="STEEPNESS_AGGRESSIVE">STEEPNESS_AGGRESSIVE</option>
          </select>


          <input
            readOnly={true}
            value={activePubkey}
            placeholder="Owner"
            className="bg-slate-100 mt-5 py-3 w-full rounded-md px-2 focus:outline-none"
          ></input>
        </div>

        {isWrongNetwork ? (
          <button
            onClick={() => { }}
            className="flex items-center justify-center bg-slate-900 text-lg py-2 mt-7 text-slate-200 font-bold rounded-lg w-full disabled:cursor-not-allowed disabled:bg-slate-700 disabled:opacity-30"
            disabled={true}
          >
            Connect Wallet
          </button>
        ) : (
          <button
            onClick={() => {
              createNewPool();
            }}
            className="flex items-center justify-center bg-slate-900 text-lg py-2 mt-7 text-slate-200 font-bold rounded-lg w-full disabled:cursor-not-allowed disabled:bg-slate-700 disabled:opacity-30 uppercase"
            disabled={loader}
          >
            Create Pool{" "}
            {loader && (
              <AiOutlineLoading3Quarters className="ml-4 font-extrabold animate-spin" />
            )}
          </button>
        )}
      </div>
    </div>
  );
}