'use client';
import Image from "next/image";
import React, { useEffect, useState, useContext, Suspense } from "react";
import { Context, server } from "../Context/store";
import { getBalance } from "../soroban/getBalance";
import { MdSwapVerticalCircle } from "react-icons/md"
import { depositUSDC } from "../soroban/deposite";
import { withdrawUSDC } from "../soroban/withdraw";
import { AiOutlineLoading3Quarters } from "react-icons/ai";
import { useRouter } from "next/navigation";
import { getNetwork } from "@stellar/freighter-api";
import { SUPPORTED_NETWORK } from "../soroban/default_data";



export default function Page() {

  const { activePubkey, setActivePubKey, walletConnectKit, showToast } = useContext(Context);

  const [token0, setToken0] = useState<number | undefined>();
  const [token1, setToken1] = useState<number | undefined>();

  const [token0Balance, setToken0Balance] = useState<number | undefined>(0.0);
  const [token1Balance, setToken1Balance] = useState<number | undefined>(0.0);

  const [zeroForOne, setZeroForOne] = useState<boolean>(true);

  const [poolId, setPoolID] = useState<string | undefined>();

  const [loader, setLoader] = useState<boolean>(false);
  const [balanceExceed, setBalanceExceed] = useState<boolean>(false);
  const [isWrongNetwork, setIsWrongNetwork] = useState<boolean>(false);

  const route = useRouter();

  useEffect(() => {
    getConnectedNetwork();
  }, [activePubkey])

  useEffect(() => {
    getId();
  }, [])

  useEffect(() => {
    getTokenBalance();
  }, [poolId])

  useEffect(() => {
    usdcForXYZ();

    if (token0 != undefined) {

      if (zeroForOne && (parseInt(token0!.toString()) > parseInt(token0Balance!.toString()))) {
        setBalanceExceed(true)
      } else {
        setBalanceExceed(false);
      }

      if (!zeroForOne && (parseInt(token0!.toString()) > parseInt(token1Balance!.toString()))) {
        setBalanceExceed(true)
      }
    }


  }, [token0, token1])




  async function getConnectedNetwork() {
    let network = await getNetwork();
    if (network != SUPPORTED_NETWORK) {
      console.log("first")
      setIsWrongNetwork(true);
    }
  }


  function getId() {

    const urlParams = new URLSearchParams(window.location.search);

    let id = urlParams.get("poolid");

    if (id != undefined) {
      setPoolID(id);
    } else {
      route.replace("/");
    }

  }


  async function getTokenBalance() {

    if (walletConnectKit) {

      await getBalance(server, walletConnectKit, poolId).then((e) => {

        setToken0Balance(parseInt(e[0]));
        setToken1Balance(parseInt(e[1]));
      }).catch((e) => {
        console.log(e);
      })
    }


  }



  const USDC = () => {
    return <div className="flex">
      <Image src={"/usdc.png"} alt={""} height={0} width={0} className="h-6 w-6 self-center mr-2" unoptimized />
      <p className="text-slate-700 font-semibold text-lg">USDC</p>
    </div>
  }

  const XYZ = () => {
    return <div className="flex">
      <Image src={"/xyz.webp"} alt={""} height={0} width={0} className="h-7 w-7 self-center mr-2" unoptimized />
      <p className="text-slate-700 font-semibold text-lg">XYZ</p>
    </div>
  }

  function usdcForXYZ() {

    zeroForOne ? setToken1(token0) : setToken1(token0! - (token0! * 1 / 100));

  }

  async function depositAndWithdraw() {

    setLoader(true);

    if (poolId && walletConnectKit && token0 != undefined) {

      if (zeroForOne) {
        await depositUSDC(server, walletConnectKit, poolId, token0).then((e) => {

          showToast(`Successfully deposit ${token0} USDC into pool`);
        }).catch((e) => {

          showToast(`Something went wrong`);
        })
      } else {
        await withdrawUSDC(server, walletConnectKit, poolId, token0).then((e) => {

          showToast(`Successfully withdraw ${token0} USDC from pool`);
        }).catch((e) => {

          showToast(`Something went wrong`);
        })
      }

      await getTokenBalance();
      setToken0(0);
      setToken1(0);

    }

    setLoader(false);

  }


  return (

    <Suspense >

      <div className="bg-slate-200 shadow-md w-1/3 rounded-lg py-10 px-6 ml-auto mr-auto mt-16 ">
        {
          zeroForOne ? <h1 className="text-2xl font-bold mb-3 ml-1">Buy</h1> : <h1 className="text-2xl font-bold mb-3 ml-1">Sell</h1>
        }

        <div className={`shadow-sm rounded-lg py-3 px-4 bg-slate-100`}>
          <div className="flex text-sm text-slate-700 justify-between">
            <p>You pay</p>
            <p onClick={() => { zeroForOne ? setToken0(token0Balance) : setToken0(token1Balance) }} className="cursor-pointer">Balance : {zeroForOne ? token0Balance : token1Balance}</p>
          </div>
          <div className="flex justify-between items-center mt-3">
            <input type="number" placeholder="0.0" pattern="\d+(9999999999999900\.\d+)?" value={token0} onChange={(e) => { setToken0(parseFloat(e.target.value)); }} className="bg-transparent text-3xl w-1/2 border-none ring-0 hover:ring-0 focus:outline-none" />
            {
              zeroForOne ? <USDC /> : <XYZ />
            }

          </div>
        </div>



        <MdSwapVerticalCircle onClick={() => { setZeroForOne(!zeroForOne); setToken0(0.0); setToken1(0.0) }} className="text-4xl ml-auto mr-auto -mt-2 -mb-2 cursor-pointer" />

        <div className="bg-slate-100 shadow-sm rounded-lg py-3 px-4 ">
          <div className="flex text-sm text-slate-700 justify-between">
            <p>You get</p>
            <p onClick={() => { zeroForOne ? setToken1(token1Balance) : setToken1(token0Balance) }} className="cursor-pointer">Balance : {zeroForOne ? token1Balance : token0Balance}</p>
          </div>
          <div className="flex justify-between items-center mt-3">
            <input type="number" placeholder="0.0" pattern="\d+(\.\d+)?" value={token1} onChange={(e) => { setToken1(parseFloat(e.target.value)) }} className="bg-transparent text-3xl w-1/2 border-none ring-0 hover:ring-0 focus:outline-none" />
            {
              zeroForOne ? <XYZ /> : <USDC />
            }
          </div>
        </div>
        {
          isWrongNetwork ?
            <button onClick={() => { }} className="flex items-center justify-center bg-slate-900 text-lg py-2 mt-7 text-slate-200 font-bold rounded-lg w-full disabled:cursor-not-allowed disabled:bg-slate-700 disabled:opacity-30" disabled={true}>Connect Wallet</button>
            : <>
              {

                balanceExceed ?
                  <button onClick={() => { }} className="flex items-center justify-center bg-slate-900 text-lg py-2 mt-7 text-slate-200 font-bold rounded-lg w-full disabled:cursor-not-allowed disabled:bg-slate-700 disabled:opacity-30" disabled={true}>Insufficient Balance</button>
                  :
                  <button onClick={() => { depositAndWithdraw() }} className="flex items-center justify-center bg-slate-900 text-lg py-2 mt-7 text-slate-200 font-bold rounded-lg w-full disabled:cursor-wait " disabled={loader}>{zeroForOne ? "BUY" : "SELL"} {loader && <AiOutlineLoading3Quarters className="ml-4 font-extrabold animate-spin" />}</button>
              }

            </>
        }

        {/* {
          balanceExceed ?
            <button onClick={() => { }} className="flex items-center justify-center bg-slate-900 text-lg py-2 mt-7 text-slate-200 font-bold rounded-lg w-full disabled:cursor-not-allowed disabled:bg-slate-700 disabled:opacity-30" disabled={true}>Insufficient Balance</button>
            :
            <button onClick={() => { depositAndWithdraw() }} className="flex items-center justify-center bg-slate-900 text-lg py-2 mt-7 text-slate-200 font-bold rounded-lg w-full disabled:cursor-wait " disabled={loader}>{zeroForOne ? "BUY" : "SELL"} {loader && <AiOutlineLoading3Quarters className="ml-4 font-extrabold animate-spin" />}</button>
        } */}

      </div>

    </Suspense>

  );
}
