import { Network } from "./default_data";

// --- getEvents will return details of particular pool's buy and sell events.
export const getEvents = async (ledgerStartFrom: number | undefined, pool: string | undefined) => {

    if (ledgerStartFrom != undefined) {
        // Request parameters
        let requestBody = {
            "jsonrpc": "2.0",
            "id": 8675309,
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
        // Fetch request 
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
                if (json?.result?.events[i]?.contractId == pool) {
                    ptx.push(json?.result?.events[i])
                }
            }
            // return events.
            return ptx
        }
        else {
            // return error if startLedger will be older than 24hrs.
            return json?.error
        }
    } else {
        console.log("undefined ledger start")
    }
};