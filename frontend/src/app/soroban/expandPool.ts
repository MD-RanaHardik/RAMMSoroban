import { TESTNET_DETAILS } from "./network";
import { StellarWalletsKit } from "@sekmet/stellar-wallets-kit";
import { accountToScVal, numberToI128 } from "./generalUtils";

import {
    TransactionBuilder,
    SorobanRpc,
    Contract,
    scValToNative,
    nativeToScVal
} from '@stellar/stellar-sdk';
import { FACTORY_CONTRACT_ADDRESS } from "./default_data";
import { ERRORS, SendTxStatus } from "./erros";


// --- Used to expand limit of pvt_qty_max_secondary using their pool id
export const expandPool = async (
    server: SorobanRpc.Server,
    walletConnectKit: StellarWalletsKit | undefined,
    pool_id: string | undefined,
    amount: number
) => {
    // get public key
    const accPubkey = await walletConnectKit!.getPublicKey();
    // get account using public key
    const account = await server.getAccount(accPubkey);
    // Token decimals
    const Q9 = 10 ** 9;

    const params = [nativeToScVal(pool_id), accountToScVal(accPubkey), numberToI128(amount * Q9)];
    // get contract using factory contract address
    const contract = new Contract(FACTORY_CONTRACT_ADDRESS);


    const fee = "100";
    // build 'expand' transaction with parameters
    const transaction = new TransactionBuilder(account, { fee, networkPassphrase: TESTNET_DETAILS.networkPassphrase, }).
        addOperation(contract.call("expand", ...params)).setTimeout(30).build();


    const preparedtransaction = await server.prepareTransaction(transaction);

    // get signed xdr by signing prepared transaction 
    const { signedXDR } = await walletConnectKit!.sign({
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

    // return error if unable to submit transaction
    return ERRORS.UNABLE_TO_SUBMIT_TX;
};
