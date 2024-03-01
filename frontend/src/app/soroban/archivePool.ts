import { TESTNET_DETAILS } from "./network";
import { StellarWalletsKit } from "@sekmet/stellar-wallets-kit";
import { accountToScVal, addressToByte, numberToI128, numberTou32, simulateTx, stringToString } from "./generalUtils";

import {
    TransactionBuilder,
    SorobanRpc,
    Contract,
    scValToNative,
    nativeToScVal
} from '@stellar/stellar-sdk';
import { CONTRACT_ADDRESS, FACTORY_CONTRACT_ADDRESS } from "./default_data";
import { ERRORS, SendTxStatus } from "./erros";
import { xdr } from "soroban-client";



export const archivePool = async (
    server: SorobanRpc.Server,
    walletConnectKit: StellarWalletsKit |undefined,
    pool_id: string | undefined,
) => {

    const accPubkey = await walletConnectKit!.getPublicKey();

    const account = await server.getAccount(accPubkey);

    // xdr.ScVal.scvBytes(Buffer.from("0xc04dc2300124d5869a2dbbe81600ba0008f609e75ce254aca065c43d3a4abbe5","hex"))

    const params = [nativeToScVal(pool_id), accountToScVal(accPubkey)];

    const contract = new Contract(FACTORY_CONTRACT_ADDRESS);


    const fee = "100";

    const transaction = new TransactionBuilder(account, { fee, networkPassphrase: TESTNET_DETAILS.networkPassphrase, }).
        addOperation(contract.call("archive_pool", ...params)).setTimeout(30).build();



    const preparedtransaction = await server.prepareTransaction(transaction);


    const { signedXDR } = await walletConnectKit!.sign({
        xdr: preparedtransaction.toXDR(),
        publicKey: accPubkey
    });


    const tx = TransactionBuilder.fromXDR(signedXDR, TESTNET_DETAILS.networkPassphrase);

    const sendResponse = await server.sendTransaction(tx);


    if (sendResponse.errorResult) {

        return ERRORS.UNABLE_TO_SUBMIT_TX;
    }

    if (sendResponse.status === SendTxStatus.Pending) {

        let txResponse = await server.getTransaction(sendResponse.hash);


        // Poll this until the status is not "NOT_FOUND"

        while (txResponse.status === SorobanRpc.Api.GetTransactionStatus.NOT_FOUND) {
            // See if the transaction is complete
            // eslint-disable-next-line no-await-in-loop
            txResponse = await server.getTransaction(sendResponse.hash);

            // Wait a second
            // eslint-disable-next-line no-await-in-loop
            await new Promise((resolve) => setTimeout(resolve, 1000));
        }

        if (txResponse.status === SorobanRpc.Api.GetTransactionStatus.SUCCESS) {

            if (txResponse.returnValue) {

                return scValToNative(txResponse.returnValue)
            }

        }
        // eslint-disable-next-line no-else-return
    }


    return ERRORS.UNABLE_TO_SUBMIT_TX;
};
