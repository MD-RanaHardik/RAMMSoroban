"use client"
import React, { useContext, useEffect, useState } from 'react'
import Link from 'next/link';
import { Context, server } from './Context/store';
import { getAllPools } from './soroban/getAllPools';
import MySorobanReactProvider from './soroban/provider';
import { VscOpenPreview } from "react-icons/vsc";


interface tableRowProps {
  index: number,
  pool_name: String,
  pool_id: String,
  pool_owner: String,
  pool_address:String
}

export interface Root {
  [key:string]: RammPool0
}

export interface RammPool0 {
  owner: string
  pool_address: string
  pool_id: string
  pool_name: string
}

export default function Page() {

  const { activePubkey, setActivePubKey, walletConnectKit } = useContext(Context);

  const [pools, setPools] = useState<Root | {}>();


  useEffect(() => {
    getAvailablePools();
  }, [])


  async function getAvailablePools() {

    if (walletConnectKit) {
      await getAllPools(server, walletConnectKit).then((e) => {
        if(e != undefined){
          setPools(e);
        }
      }).catch((e) => {
        console.log(e);
      })
    }
  }


  function TableRow(props: tableRowProps) {

    return <>
      <tr className="bg-white border-b hover:bg-gray-50">
        <th scope="row" className="px-6 py-4 font-medium text-gray-900 whitespace-nowrap ">
          {props.index}
        </th>
        <td className="px-6 py-4">
          {props.pool_name}
        </td>
        <td className="px-6 py-4">
          {props.pool_id}
        </td>
        <td className="px-6 py-4">
          {props.pool_owner}
        </td>
        <td className="px-6 py-4 flex items-center">
          <Link href={"/swap?poolid=" + props.pool_id} className="font-medium text-blue-600 dark:text-blue-500 hover:underline">Swap</Link>
          <Link href={`https://futurenet.steexp.com/contract/${props.pool_address}`} ><VscOpenPreview className='text-lg text-blue-700 ml-5 '  /></Link>
          
        </td>
      </tr>
    </>
  }

  


  return (
    <MySorobanReactProvider >
      <div className='p-5'>

        <h1 className='text-slate-700 font-semibold text-lg ml-1 mb-3'>Active Pools</h1>

        <div className="relative overflow-x-auto shadow-md sm:rounded-lg">
          <table className="w-full text-sm text-left rtl:text-right text-gray-500 ">
            <thead className="text-xs text-gray-700 uppercase bg-gray-100 ">
              <tr>
                <th scope="col" className="px-6 py-3">
                  Index
                </th>
                <th scope="col" className="px-6 py-3">
                  Pool Name
                </th>
                <th scope="col" className="px-6 py-3">
                  Pool ID
                </th>
                <th scope="col" className="px-6 py-3">
                  Pool Owner
                </th>
                <th scope="col" className="px-6 py-3">
                  Action
                </th>
              </tr>
            </thead>
            <tbody>
              {
                
                (pools != undefined) &&
                  Object.entries(pools).map((value, index) => {
                    return <TableRow key={index + 1} index={index + 1} pool_address={value[1]["pool_address"]} pool_id={value[1]["pool_id"]} pool_name={value[1]["pool_name"]} pool_owner={value[1]["owner"]} />
                  })
                
              }

            </tbody>
          </table>
        </div>

      </div>
    </MySorobanReactProvider>
  )
}
