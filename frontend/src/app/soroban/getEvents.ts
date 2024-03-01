
import { Server } from "@stellar/stellar-sdk/lib/soroban";
import { xdr, scValToNative, nativeToScVal } from "soroban-client";
import { Network } from "./default_data";


const merCure = async () => {
    // "AAAADwAAAAVUT0tFTgAAAA=="
    // 'AAAADwAAAAVUT0tFTgAAAA==' "AAAADwAAAAhCdXlUb2tlbg==" 
    const rawval_1 = "AAAADwAAAAhCdXlUb2tlbg==" 
    const rawval_2 = "AAAAAAAAAAA="

    const scval_1 = xdr.ScVal.fromXDR(rawval_1, 'base64')
    const scval_2 = xdr.ScVal.fromXDR(rawval_2, 'base64')


    const native_1 = scValToNative(scval_1)
    const native_2 = scValToNative(scval_2)

    console.log(
        // native_1,
        native_2,
    )
    let inovative = "RAMMBuy"
    let sellin = "RAMMSell"
    console.log(
        // nativeToScVal(99, { type: 'u32' }).toXDR('base64'),
        nativeToScVal(sellin, {type: 'symbol'}).toXDR('base64'),
        // nativeToScVal(inovative).toXDR('base64'),
    )
}


export const getEvents = async (ledgerStartFrom: number | undefined, pool: string | undefined) => {

    merCure()
    if (ledgerStartFrom != undefined) {
       
        // 801798
        // 801791
        // let txnBuy = ("AAAADwAAAAdSQU1NQnV5AA==" && "AAAADwAAAAhSQU1NU2VsbA==")
        let txnSell = "AAAADwAAAAhSQU1NU2VsbA=="
        // console.log(txnBuy)

        let requestBody = {
            "jsonrpc": "2.0",
            "id": 8675309,
            // "id": 1,
            "method": "getEvents",
            "params": {
                "startLedger": ledgerStartFrom,
                "filters": [
                    {
                        "type": "contract",
                        "contractIds": [
                            `${pool}`,
                        ],
                        "topics": [
                            [
                                "AAAADwAAAAdSQU1NQnV5AA==",
                                "*",
                                "*",
                                "*"
                            ]
                          ]
                        
                    },
                    {
                        "type": "contract",
                        "contractIds": [
                            `${pool}`,
                        ],
                        "topics": [
                            [
                                "AAAADwAAAAhSQU1NU2VsbA==",
                                "*",
                                "*",
                                "*"
                            ]
                          ]
                    }
                ],
            }
        }

        let res = await fetch(`${Network}`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify(requestBody),
        })
        let json = await res.json()

        if (json?.result?.events) {
            let ptx = [];
            for (let i in json?.result?.events) {
                // let i1 = scValToNative(xdr.ScVal.fromXDR(json?.result?.events[i].topic[2], 'base64'))
                if (json?.result?.events[i]?.contractId == pool) {
                    ptx.push(json?.result?.events[i])
                }
            }
            return ptx
        }
        else {
            return json?.error
        }
    } else {
        console.log("undefined ledger start")
    }
};