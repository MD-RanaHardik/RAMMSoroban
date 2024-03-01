import { TESTNET_DETAILS } from "./network";
import { StellarWalletsKit } from "@sekmet/stellar-wallets-kit";
import { accountToScVal, addressToByte, numberToI128, numberTou32, simulateTx, stringToString } from "./generalUtils";

import {
    TransactionBuilder,
    SorobanRpc,
    Contract,
    xdr,
    scValToNative
} from '@stellar/stellar-sdk';
import { CONTRACT_ADDRESS, FACTORY_CONTRACT_ADDRESS } from "./default_data";
import { ERRORS, SendTxStatus } from "./erros";



export const createPool = async (
    server: SorobanRpc.Server,
    walletConnectKit: StellarWalletsKit,
    pool_name: string,
    max_primary_quantity:number,
    max_primary_price:number,
    secondary_available:number,
    initial_primary_price:number,
    steepness:number
) => {

    const accPubkey = await walletConnectKit.getPublicKey();

    const account = await server.getAccount(accPubkey);

    // owner:Address,pool_name:String,pvt_qty_max_primary:i128,pvt_price_max_primary:i128,pvt_price_initial_primary:i128,pvt_available_secondary:i128,steepness:u32

    const params = [accountToScVal(accPubkey), xdr.ScVal.scvString(pool_name),numberToI128(max_primary_quantity),numberToI128(max_primary_price),numberToI128(initial_primary_price),numberToI128(secondary_available),numberTou32(steepness)];

    // const params = [accountToScVal(accPubkey), xdr.ScVal.scvString(pool_name)];

    const contract = new Contract(FACTORY_CONTRACT_ADDRESS);


    const fee = "100";

    const transaction = new TransactionBuilder(account, { fee, networkPassphrase: TESTNET_DETAILS.networkPassphrase, }).
        addOperation(contract.call("create_pool", ...params)).setTimeout(30).build();



    const preparedtransaction = await server.prepareTransaction(transaction);


    const { signedXDR } = await walletConnectKit.sign({
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
