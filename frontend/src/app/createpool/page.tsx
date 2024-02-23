"use client";
import React, { useContext, useEffect, useState } from 'react'
import { Context, server } from '../Context/store';
import { createPool } from '../soroban/createPool';
import { AiOutlineLoading3Quarters } from 'react-icons/ai';
import { getNetwork } from '@stellar/freighter-api';
import { SUPPORTED_NETWORK } from '../soroban/default_data';

export default function Page() {
  const {activePubkey,setActivePubKey,walletConnectKit,showToast} = useContext(Context);

  const [poolName,setPoolName] = useState<string | undefined>();

  const [loader,setLoader] = useState<boolean>(false);

  const [isWrongNetwork,setIsWrongNetwork] = useState<boolean>(false);

  useEffect(()=>{
    getConnectedNetwork();
  },[activePubkey])

  async function getConnectedNetwork() {
    let network = await getNetwork();
    if(network != SUPPORTED_NETWORK){
      console.log("first")
      setIsWrongNetwork(true);
    }
  }

  async function createNewPool() {

    setLoader(true);

    if(walletConnectKit && poolName != undefined){

      await createPool(server,walletConnectKit,poolName).then((e)=>{
        showToast("Pool created");
      }).catch((e)=>{
        showToast("Something went wrong");
      })

    }

    setLoader(false);
    
  }

  return (
    <div>
       <div className="bg-slate-200 shadow-md w-1/3 rounded-lg py-10 px-6 ml-auto mr-auto mt-16 ">
        <h1 className="text-2xl font-bold mb-3 ml-1">Create Pool</h1>

        <input placeholder='Pool Name' value={poolName} onChange={(e)=>{setPoolName(e.target.value)}} className='bg-slate-100 py-3 w-full rounded-md px-2' ></input>
        
        <input readOnly={true} value={activePubkey} placeholder='Owner' className='bg-slate-100 mt-5 py-3 w-full rounded-md px-2 focus:outline-none' ></input>

        {
          isWrongNetwork ? 
          <button onClick={() => { }} className="flex items-center justify-center bg-slate-900 text-lg py-2 mt-7 text-slate-200 font-bold rounded-lg w-full disabled:cursor-not-allowed disabled:bg-slate-700 disabled:opacity-30" disabled={true}>Connect Wallet</button>
          :
          <button onClick={()=>{createNewPool()}} className="flex items-center justify-center bg-slate-900 text-lg py-2 mt-7 text-slate-200 font-bold rounded-lg w-full disabled:cursor-not-allowed disabled:bg-slate-700 disabled:opacity-30 uppercase" disabled={loader} >Create Pool { loader && <AiOutlineLoading3Quarters className="ml-4 font-extrabold animate-spin" /> }</button>
        }

       </div>

    </div>
  )
}
