"use client"
import React, { useContext, useEffect, useState } from 'react'
import Link from 'next/link';
import { Context, server } from './Context/store';
import { getAllPools } from './soroban/getAllPools';
import MySorobanReactProvider from './soroban/provider';
import { VscOpenPreview } from "react-icons/vsc";
import { DropdownMenu, Button, Flex, } from '@radix-ui/themes';
import * as Dialog from '@radix-ui/react-dialog';
import { Cross2Icon } from '@radix-ui/react-icons';
import { IoMdMore } from "react-icons/io";
import { buyPVTTOken } from './soroban/buyPVTToken';
import { sellPVTToken } from './soroban/sellPVTToken';
import { archivePool } from './soroban/archivePool';
import { unarchivePool } from './soroban/unarchivePool';
import { startPool } from './soroban/startPool';
import { stopPool } from './soroban/stopPool';
import { expandPool } from './soroban/expandPool';
import { withdrawPoolFund } from './soroban/withdrawFund';
import { useRouter } from 'next/navigation';
import { getBuyPrice } from './soroban/getBuyPrice';
import { getSellPrice } from './soroban/getSellPrice';
import { AiOutlineLoading3Quarters } from "react-icons/ai";
import { tree } from 'next/dist/build/templates/app-page';
import { getBalance } from './soroban/getBalance';




interface tableRowProps {
  index: number,
  pool_name: String,
  pool_id: String,
  pool_owner: String,
  pool_address: String,
  primary_max_qty: number,
  secondary_max_qty: number,
  primary_max_price: number,
  secondary_max_price: number,
  initial_price: number,
  primary_steepness: number,
  secondary_steepness: number,
  treasury: number,
  pvttokens: number,
  pool_status: number,
  archived: boolean

}

export interface Root {
  [key: string]: RammPool0
}

export interface RammPool0 {
  c_primary_steepness: string,
  owner: string,
  pool_address: string
  pool_id: string
  pool_name: string
  pvt_price_initial_primary: number
  pvt_price_max_primary: number
  pvt_price_max_secondary: number
  pvt_qty_max_primary: number
  pvt_qty_max_secondary: number
  pvt_available_secondary:number,
  treasury: number
  x: string,
  pool_status: number,
  archived: boolean

}

export default function Page() {

  const { activePubkey, setActivePubKey, walletConnectKit, showToast } = useContext(Context);

  const [pools, setPools] = useState<Root | {}>();

  const [buyPrice, setBuyPrice] = useState(0);
  const [sellPrice, setSellPrice] = useState(0);
  const [buyOrSell, setBuyOrSell] = useState(true);
  const [inSecondaryMode, setInSecondaryMode] = useState(false);
  const [openModel, setOpenModel] = useState(false);
  const [poolID, setPoolID] = useState<string|undefined>(undefined);
  const [usdcBalance, setUSDCBalance] = useState(0);
  const [pvtBalance, setPVTBalance] = useState(0);

  const router = useRouter();


  useEffect(() => {
    getAvailablePools();
  }, [])





  async function getAvailablePools() {

    if (walletConnectKit) {
      await getAllPools(server, walletConnectKit).then((e) => {
        if (e != undefined) {
          setPools(e);
        }
      }).catch((e) => {
        console.log(e);
      })
    }
  }

  async function BuyPVTTOken(poolId: string) {

    await buyPVTTOken(server, walletConnectKit, poolId).then((e) => {
      setOpenModel(false);
      showToast(`Token bought successfully`);
      getAvailablePools();
    }).catch((e) => {
      setOpenModel(false);
      showToast(`Please check balance`);
    })


  }

  async function SellPVTTOken(poolId: string) {

    await sellPVTToken(server, walletConnectKit, poolId).then((e) => {
      setOpenModel(false);
      showToast(`Token sold successfully`);
      getAvailablePools();
    }).catch((e) => {
      setOpenModel(false);
      showToast(`Pool not in secondary mode`);
    })

  }

  async function ArchivePool(poolId: string) {

    await archivePool(server, walletConnectKit, poolId).then((e) => {
      showToast(`Pool successfully archive`);
      getAvailablePools();
    }).catch((e) => {

      showToast(`Something went wrong`);
    })

  }

  async function UnarchivePool(poolId: string) {

    await unarchivePool(server, walletConnectKit, poolId).then((e) => {
      showToast(`Pool successfully unarchive`);
      getAvailablePools();
    }).catch((e) => {

      showToast(`Something went wrong`);
    })
  }

  async function StartPool(poolId: string) {

    await startPool(server, walletConnectKit, poolId).then((e) => {
      showToast(`Pool started`);
      getAvailablePools();
    }).catch((e) => {

      showToast(`Something went wrong`);
    })
  }

  async function StopPool(poolId: string) {

    await stopPool(server, walletConnectKit, poolId).then((e) => {
      showToast(`Pool stop`);
      getAvailablePools();
    }).catch((e) => {

      showToast(`Something went wrong`);
    })
  }


  async function ExpandPool(poolId: string, amount: number) {

    await expandPool(server, walletConnectKit, poolId, amount).then((e) => {
      showToast(`Pool expanded by ${amount}`);
      getAvailablePools();
    }).catch((e) => {

      showToast(`Quantity exceeding secondary capacity`);
    })
  }

  async function WithdrawFund(poolId: string) {

    await withdrawPoolFund(server, walletConnectKit, poolId).then((e) => {
      showToast(`Successfully withdraw pool fund`);
      getAvailablePools();
    }).catch((e) => {

      showToast(`Something went wrong`);
    })
  }

  async function PoolBalance(poolId: string) {

    await getBalance(server, walletConnectKit, poolId).then((e) => {
      setUSDCBalance(parseInt(e[0]) / (10**9));
      setPVTBalance(parseInt(e[1]) / (10**9));
    }).catch((e) => {
      console.log(e);
    })
  }

 
  async function InitBuy(pool_id:string,x:number,pvt_qty_max_primary:number,pvt_qty_max_secondary:number){

      await PoolBalance(pool_id);

      await getBuyPrice(server, walletConnectKit, pool_id).then((e) => {
        setBuyPrice(parseInt(e.toString()) / (10 ** 9));
        setPoolID(pool_id);
        setBuyOrSell(true);
        setOpenModel(true);
      }).catch((e) => {

        if(x == (pvt_qty_max_primary + pvt_qty_max_secondary)){
          showToast("Buying limit reached");
        }
      })
  }

  async function InitSell(pool_id:string){

    await PoolBalance(pool_id);

    await getSellPrice(server, walletConnectKit, pool_id).then((e) => {
      setSellPrice(parseInt(e[1]) / (10 ** 9));
      setInSecondaryMode((e[0].toString() == "false") ? false : true);
      setPoolID(pool_id);
      setBuyOrSell(false);
      setOpenModel(true);
    }).catch((e) => {

      console.log(e);

    })
  }



  function Model() {
    return <>
      <Dialog.Root open={openModel} onOpenChange={()=>{setOpenModel(!openModel)}}>
        <Dialog.Portal>
          <Dialog.Overlay className="bg-blackA6 data-[state=open]:animate-overlayShow fixed inset-0 bg-slate-600/25 backdrop-blur" />
          <Dialog.Content className="data-[state=open]:animate-contentShow fixed top-[50%] left-[50%] max-h-[85vh] w-[90vw] max-w-[450px] translate-x-[-50%] translate-y-[-50%] rounded-[6px] bg-white p-[25px] shadow-[hsl(206_22%_7%_/_35%)_0px_10px_38px_-10px,_hsl(206_22%_7%_/_20%)_0px_10px_20px_-15px] focus:outline-none">
            <Dialog.Title className="text-3xl font-semibold">
              {buyOrSell ? "Buy" : "Sell"}
            </Dialog.Title>
            <Dialog.Description className="text-slate-800 mt-7 mb-5 text-lg leading-normal">
              {
                buyOrSell ? <>{buyPrice} USDC ≈ 1 PVT</>
                : 
                <>1 PVT ≈ {sellPrice} USDC</>
              }
              
              
            </Dialog.Description>
            <fieldset className="mb-[15px] flex items-center gap-5 justify-center">
              <input
                className="text-slate-900 bg-slate-100 shadow-violet focus:shadow-violet8 w-full flex-1 items-center justify-center rounded-[4px] px-5 py-5 text-2xl"
                disabled={true}
                value={buyOrSell ? buyPrice : sellPrice}
              />
            </fieldset>
            <div>
            <p>PVT Balance : {pvtBalance}</p>
            <p>USDC Balance : {usdcBalance}</p>
            </div>
            <div className="mt-[25px] flex justify-center">
              {/* <Dialog.Close asChild> */}
              

              {
                buyOrSell ?
                  (usdcBalance >= buyPrice)?
                  <button onClick={() => {poolID && BuyPVTTOken(poolID)}} className="text-xl bg-slate-900 w-full py-6 text-white hover:bg-green5 focus:shadow-green7 inline-flex h-[35px] items-center justify-center rounded-[4px] px-[15px] font-medium leading-none focus:shadow-[0_0_0_2px] focus:outline-none">Buy</button>
                  :
                  <button className="text-xl bg-slate-900 w-full py-6 text-white hover:bg-green5 focus:shadow-green7 inline-flex h-[35px] items-center justify-center rounded-[4px] px-[15px] font-medium leading-none focus:shadow-[0_0_0_2px] focus:outline-none disabled:cursor-not-allowed" disabled={true}>Insufficient Balance</button>
                :
                  inSecondaryMode ?
                    (pvtBalance >= 1)?
                    <button onClick={() => {poolID && SellPVTTOken(poolID) }} className="text-xl bg-slate-900 w-full py-6 text-white hover:bg-green5 focus:shadow-green7 inline-flex h-[35px] items-center justify-center rounded-[4px] px-[15px] font-medium leading-none focus:shadow-[0_0_0_2px] focus:outline-none">Sell</button>
                    :
                    <button className="text-xl bg-slate-900 w-full py-6 text-white hover:bg-green5 focus:shadow-green7 inline-flex h-[35px] items-center justify-center rounded-[4px] px-[15px] font-medium leading-none focus:shadow-[0_0_0_2px] focus:outline-none disabled:cursor-not-allowed" disabled={true}>Insufficient Balance</button>
                    :
                    <button className="cursor-not-allowed text-xl bg-slate-900 w-full py-6 text-white hover:bg-green5 focus:shadow-green7 inline-flex h-[35px] items-center justify-center rounded-[4px] px-[15px] font-medium leading-none focus:shadow-[0_0_0_2px] focus:outline-none " disabled={true}>
                      NOT IN SECONDARY MODE
                    </button>
              }
              
              {/* </Dialog.Close> */}
            </div>
            <Dialog.Close asChild>
              <button
                className="text-slate-900 hover:bg-slate-100 focus:shadow-violet7 absolute top-[10px] right-[10px] inline-flex h-[25px] w-[25px] appearance-none items-center justify-center rounded-full focus:shadow-[0_0_0_2px] focus:outline-none"
                aria-label="Close">

                <Cross2Icon />
              </button>
            </Dialog.Close>
          </Dialog.Content>
        </Dialog.Portal>
      </Dialog.Root>

    </>

  }

 

  function TableRow(props: tableRowProps) {

    // let pvtBalance = PoolBalance(props.pool_id.toString()).then((e)=>{return e[1]});

    return <>
      <tr className="bg-white border-dashed border-b hover:bg-gray-50">
        <th scope="row" className="px-6 py-4 font-medium  whitespace-nowrap ">
          {props.index}
        </th>
        <td className="px-6 py-4">
          {props.pool_name}
        </td>
        <td className="px-6 py-4">
          {(Number(props.primary_max_qty) / 10 ** 9).toString()}
        </td>
        <td className="px-6 py-4">
          {(Number(props.secondary_max_qty) / 10 ** 9).toString()}
        </td>
        <td className="px-6 py-4">
          {(Number(props.primary_max_price) / 10 ** 9).toString()}
        </td>
        <td className="px-6 py-4">
          {(Number(props.secondary_max_price) / 10 ** 9).toString()}
        </td>
        <td className="px-6 py-4">
          {(Number(props.initial_price) / 10 ** 9).toString()}
        </td>
        <td className="px-6 py-4">
          {props.primary_steepness.toString()}
        </td>
        <td className="px-6 py-4">
          {props.secondary_steepness.toString()}
        </td>
        <td className="px-6 py-4">
          {(Number(props.pvttokens) / 10 ** 9).toString()}
        </td>
        <td className="px-6 py-4">
          {(Number(props.treasury) / 10 ** 9).toString()}
        </td>
        
        <td className="px-6 py-4 flex items-center text-center">



          <DropdownMenu.Root >
            <DropdownMenu.Trigger>
              <button className='border-none ring-0'>
                <IoMdMore className='text-lg text-center' />
              </button>

            </DropdownMenu.Trigger>
            <DropdownMenu.Content className='bg-slate-100 shadow-md p-3'>

              <DropdownMenu.Item className='text-sm text-left px-3 py-1.5 rounded-sm w-full hover:bg-slate-300 hover:text-slate-900' onClick={() => { router.push(`/txns/${props?.pool_address}`) }}>Transactions</DropdownMenu.Item>
              <button onClick={() => { InitBuy(props.pool_id.toString(),(Number(props.pvttokens) / 10 ** 9),(Number(props.primary_max_qty) / 10 ** 9),(Number(props.secondary_max_qty) / 10 ** 9)) }} className='text-sm text-left px-3 py-1.5 rounded-sm w-full hover:bg-slate-300'>Buy</button>
              <button onClick={() => { InitSell(props.pool_id.toString()) }} className='text-sm text-left px-3 py-1.5 rounded-sm w-full hover:bg-slate-300'>Sell</button>
              
              <DropdownMenu.Separator />
              {(props.pool_status == 0 && activePubkey == props.pool_owner) && <DropdownMenu.Item className='text-sm text-left px-3 py-1.5 rounded-sm w-full hover:bg-slate-300 hover:text-slate-900' onClick={() => { StartPool(props.pool_id.toString()) }}>Start Pool</DropdownMenu.Item>}
              {(props.pool_status == 1 && activePubkey == props.pool_owner) && <DropdownMenu.Item className='text-sm text-left px-3 py-1.5 rounded-sm w-full hover:bg-slate-300 hover:text-slate-900' onClick={() => { StopPool(props.pool_id.toString()) }}>Stop Pool</DropdownMenu.Item>}
              {(!props.archived && activePubkey == props.pool_owner) && <DropdownMenu.Item className='text-sm text-left px-3 py-1.5 rounded-sm w-full hover:bg-slate-300 hover:text-slate-900' onClick={() => { ArchivePool(props.pool_id.toString()) }}>Archive</DropdownMenu.Item>}
              {(props.archived && activePubkey == props.pool_owner) && <DropdownMenu.Item className='text-sm text-left px-3 py-1.5 rounded-sm w-full hover:bg-slate-300 hover:text-slate-900' onClick={() => { UnarchivePool(props.pool_id.toString()) }} >Unarchive</DropdownMenu.Item>}
              {(props.pool_status == 1 && activePubkey == props.pool_owner) && <DropdownMenu.Item className='text-sm text-left px-3 py-1.5 rounded-sm w-full hover:bg-slate-300 hover:text-slate-900' onClick={() => { ExpandPool(props.pool_id.toString(), 100) }}>Expand by 100</DropdownMenu.Item>}
              {((props.pool_status == 1 && activePubkey == props.pool_owner)) && <DropdownMenu.Item className='text-sm text-left px-3 py-1.5 rounded-sm w-full hover:bg-slate-300 hover:text-slate-900' onClick={() => { ExpandPool(props.pool_id.toString(), 500) }}>Expand by 500</DropdownMenu.Item>}
              {(props.pool_status == 2 && activePubkey == props.pool_owner && props.treasury != 0) && <DropdownMenu.Item className='text-sm text-left px-3 py-1.5 rounded-sm w-full hover:bg-slate-300 hover:text-slate-900' onClick={() => { WithdrawFund(props.pool_id.toString()) }}>Withdraw Fund</DropdownMenu.Item>}
              {/* <DropdownMenu.Separator /> */}

            </DropdownMenu.Content>
          </DropdownMenu.Root>

        </td>
      </tr>
    </>
  }




  return (
    <MySorobanReactProvider >

      <div className='p-5 shadow-lg mx-20 rounded-md mt-10 mb-20 ring-1 ring-slate-100'>
        <Model />
        <h1 className='text-slate-700 font-semibold text-lg mb-5 ml-5'>Pools</h1>
        <div className="relative overflow-x-auto">
          <table className="w-full text-sm text-left rtl:text-right text-gray-500 ">
            <thead className="text-xs text-gray-400 uppercase  border-dashed border-b">
              <tr>
                <th scope="col" className="px-6 py-3">
                  Index
                </th>
                <th scope="col" className="px-6 py-3">
                  Pool Name
                </th>
                <th scope="col" className="px-6 py-3">
                  Primary Max Qty
                </th>
                <th scope="col" className="px-6 py-3">
                  Secondary Max Qty
                </th>
                <th scope="col" className="px-6 py-3">
                  Primary Max Price
                </th>
                <th scope="col" className="px-6 py-3">
                  Secondary Max Price
                </th>
                <th scope="col" className="px-6 py-3">
                  Initial Price
                </th>
                <th scope="col" className="px-6 py-3">
                  Primary Steepness
                </th>
                <th scope="col" className="px-6 py-3">
                  Secondary Steepness
                </th>
                <th scope="col" className="px-6 py-3">
                  PVT Tokens
                </th>
                <th scope="col" className="px-6 py-3">
                  Treasury
                </th>
                <th scope="col" className="px-6 py-3">
                  Action
                </th>
              </tr>
            </thead>

            <tbody>

              {

                (pools == undefined) ?
                  <tr className=''>
                    <td colSpan={12} className='text-center ml-auto mr-auto'>
                      <p className='font-bold py-5 text-center flex items-center justify-center'>Loading <AiOutlineLoading3Quarters className='animate-spin ml-3' /></p>
                    </td>
                  </tr>
                  :
                  Object.entries(pools).map((value, index) => {

                    return <TableRow key={index + 1} index={index + 1} pool_address={value[1]["pool_address"]} pool_id={value[1]["pool_id"]} pool_name={value[1]["pool_name"]} pool_owner={value[1]["owner"]} primary_max_qty={value[1]["pvt_qty_max_primary"]} secondary_max_qty={value[1]["pvt_qty_max_secondary"]} primary_max_price={value[1]["pvt_price_max_primary"]} secondary_max_price={value[1]["pvt_price_max_secondary"]} initial_price={value[1]["pvt_price_initial_primary"]} primary_steepness={parseInt(value[1]["c_primary_steepness"])} secondary_steepness={parseInt(value[1]["c_primary_steepness"])} treasury={value[1]["treasury"]} pvttokens={parseInt(value[1]["x"])} pool_status={value[1]["pool_status"]} archived={value[1]["archived"]} />
                  })

              }

            </tbody>

          </table>
        </div>

      </div>
    </MySorobanReactProvider>
  )
}

function showToast(arg0: string) {
  throw new Error('Function not implemented.');
}
