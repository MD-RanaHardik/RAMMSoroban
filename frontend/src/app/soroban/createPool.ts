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
import { FACTORY_CONTRACT_ADDRESS } from "./default_data";
import { ERRORS, SendTxStatus } from "./erros";


// --- Used to create pool using required parameters
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
    // get public key
    const accPubkey = await walletConnectKit.getPublicKey();
    // get account using public key
    const account = await server.getAccount(accPubkey);
    // token decimal
    const Q9 = 10**9;

    const params = [accountToScVal(accPubkey), xdr.ScVal.scvString(pool_name),numberToI128(max_primary_quantity * Q9),numberToI128(max_primary_price * Q9),numberToI128(initial_primary_price * Q9),numberToI128(secondary_available * Q9),numberTou32(steepness)];
    // get contract using factory contract address
    const contract = new Contract(FACTORY_CONTRACT_ADDRESS);

    const fee = "100";

    // Build 'create_pool' transaction
    const transaction = new TransactionBuilder(account, { fee, networkPassphrase: TESTNET_DETAILS.networkPassphrase, }).
        addOperation(contract.call("create_pool", ...params)).setTimeout(30).build();

    const preparedtransaction = await server.prepareTransaction(transaction);

    // get signed xdr using prepared transaction
    const { signedXDR } = await walletConnectKit.sign({
        xdr: preparedtransaction.toXDR(),
        publicKey: accPubkey
    });

    // Build transaction using signed xdr
    const tx = TransactionBuilder.fromXDR(signedXDR, TESTNET_DETAILS.networkPassphrase);

    // Send transaction
    const sendResponse = await server.sendTransaction(tx);


    if (sendResponse.errorResult) {
        // return error if unable to submit transaction
        return ERRORS.UNABLE_TO_SUBMIT_TX;
    }

    if (sendResponse.status === SendTxStatus.Pending) {
        // get transaction using transaction hash
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
    
    // return error if unable to submit transaction
    return ERRORS.UNABLE_TO_SUBMIT_TX;
};
