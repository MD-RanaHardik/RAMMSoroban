"use client"
import Link from "next/link";
import { useEffect, useState } from "react";
import { VscOpenPreview } from "react-icons/vsc";
import { getEvents } from "../../soroban/getEvents";
import { scValToNative, xdr } from "soroban-client";
import { Network } from "@/app/soroban/default_data";


export default function PoolTxns({ params }:any) {

  const [latestLedger, setLatestLedger] = useState<number | undefined>();
  const [poolTransactions, setPoolTransactions] = useState([]);

  useEffect(() => {
    getLatestLedger()
    getPoolLedgers(params.pool_txns)
  }, [params, latestLedger])

  const getLatestLedger = async () => {
    let requestBody = {
      "jsonrpc": "2.0",
      "id": 8675309,
      "method": "getLatestLedger"
    }
    let res = await fetch(`${Network}`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(requestBody),
    })
    let json = await res.json()
    setLatestLedger(json?.result?.sequence)
  }

  const getPoolLedgers = async (poolId: string) => {
    if (latestLedger != undefined) {
      
      let data = await getEvents(latestLedger - 16500, poolId)
      if (!data?.message) {
        console.log(data)
        setPoolTransactions(data)
      } else {
        console.log(data?.message)
      }
    }
  }

  return (

    <div className='p-5 shadow-lg mx-20 rounded-md mt-10 ring-1 ring-slate-100'>

      <h1 className='text-slate-700 font-semibold text-lg mb-5 ml-5'>Pool Transactions</h1>

      <div className="relative overflow-x-auto  sm:rounded-lg">
        <table className="w-full text-sm text-left rtl:text-right text-gray-500 ">
          <thead className="text-xs text-gray-400 uppercase  border-dashed border-b ">
            <tr>
              <th scope="col" className="px-6 py-3">
                Ledger
              </th>
              <th scope="col" className="px-6 py-3">
                Type
              </th>
              <th scope="col" className="px-6 py-3">
                User Address
              </th>
              <th scope="col" className="px-6 py-3">
                In Secondary Mode
              </th>
              <th scope="col" className="px-6 py-3">
                Current X
              </th>
              <th scope="col" className="px-6 py-3">
                Ledger Close Time
              </th>
              <th scope="col" className="px-6 py-3">
                Action
              </th>
            </tr>
          </thead>
          <tbody>
            {
              poolTransactions.map((e) => {
                return (
                  <TableRow key={e} props={e} />
                )
              })
            }
          </tbody>
        </table>
      </div>

    </div>
  )
}


function TableRow(props: any) {
  return <>

    <tr className=" bg-white border-dashed border-b hover:bg-gray-50">
      <th scope="row" className="px-6 py-4 font-medium text-gray-900 whitespace-nowrap ">
        {props?.props?.ledger}
      </th>
      <td className="px-6 py-4">
        {(scValToNative(xdr.ScVal.fromXDR(props?.props?.topic[0], 'base64'))).substring(4)}
      </td>
      <td className="px-6 py-4">
        {shortenString(scValToNative(xdr.ScVal.fromXDR(props?.props?.topic[1], 'base64')))}
      </td>
      <td className="px-6 py-4">
        {(scValToNative(xdr.ScVal.fromXDR(props?.props?.topic[2], 'base64'))).toString()}
      </td>
      <td className="px-6 py-4">
        {Number(scValToNative(xdr.ScVal.fromXDR(props?.props?.topic[3], 'base64'))).toString()}
      </td>
      <td className="px-6 py-4">
        {props?.props?.ledgerClosedAt.replace(/T/g, '  ')}
      </td>
      <td className="px-6 py-4 flex items-center">
        <Link href={`https://futurenet.steexp.com/ledger/${props?.props?.ledger}`} ><VscOpenPreview className='text-lg text-blue-700 ml-5 ' /></Link>
      </td>
    </tr>
  </>
}

const shortenString = (str: string) => {
  if (str.length > 25) {
    return str.substring(0, 30) + "...";
  } else {
    return str;
  }
}