import { TESTNET_DETAILS } from "./network";
import { StellarWalletsKit } from "@sekmet/stellar-wallets-kit";
import { accountToScVal, addressToByte, numberToI128, numberTou32, simulateTx, stringToString } from "./generalUtils";

import {
    TransactionBuilder,
    SorobanRpc,
    Contract,
    xdr,
    scValToNative,
    Keypair,

} from '@stellar/stellar-sdk';
import { ERRORS, SendTxStatus } from "./erros";


// --- Used to mint USDC faucet to connected user wallet
export const FaucetUSDC = async (
    server: SorobanRpc.Server,
    walletConnectKit: StellarWalletsKit | undefined,
    showToast: (msg: string) => void
) => {
    // Get public key
    const accPubkey = await walletConnectKit!.getPublicKey();
    // Create owner using key pair
    let owner = Keypair.fromSecret("SDUVTKDKZWIOGMT5A2QRK24GNZXVLAT73KUDCFWSAPV2LHMADRGBTEMA")
    // Get owner account using public key
    const owner_account = await server.getAccount(owner.publicKey());

    const params = [accountToScVal(accPubkey), numberToI128(10000000000)];
    // Get contract using contract id
    const contract = new Contract("CB7XVGJGKZNHPAATSVP67VOOIYJ4EPQZ5IMSGWAGDHDO6JW4NRIA5UPU");

    const fee = "100";
    // build 'mint' transaction with parameters
    const transaction = new TransactionBuilder(owner_account, { fee, networkPassphrase: TESTNET_DETAILS.networkPassphrase, }).
        addOperation(contract.call("mint", ...params)).setTimeout(30).build();

    const preparedtransaction = await server.prepareTransaction(transaction);

    preparedtransaction.sign(owner);

    // Build transaction using signed xdr
    const tx = TransactionBuilder.fromXDR(preparedtransaction.toXDR(), TESTNET_DETAILS.networkPassphrase);
    // Send transaction
    const sendResponse = await server.sendTransaction(tx);


    if (sendResponse.errorResult) {

        console.log("Failed", sendResponse.errorResult);

        showToast("Please try after some time");
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

            showToast("Successfully minted 10 USDC");

            if (txResponse.returnValue) {

                return scValToNative(txResponse.returnValue)
            }
        }
        // eslint-disable-next-line no-else-return
    }
    console.log("Failed Last")
};
