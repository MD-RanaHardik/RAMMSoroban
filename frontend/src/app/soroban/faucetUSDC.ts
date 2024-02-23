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
import { CONTRACT_ADDRESS, FACTORY_CONTRACT_ADDRESS } from "./default_data";
import { ERRORS, SendTxStatus } from "./erros";



export const FaucetUSDC = async (
    server: SorobanRpc.Server,
    walletConnectKit: StellarWalletsKit | undefined,
    showToast: (msg:string) => void
) => {

    const accPubkey = await walletConnectKit!.getPublicKey();

    const account = await server.getAccount(accPubkey);

    let owner = Keypair.fromSecret("SDUVTKDKZWIOGMT5A2QRK24GNZXVLAT73KUDCFWSAPV2LHMADRGBTEMA")

    const owner_account = await server.getAccount(owner.publicKey());


    const params = [accountToScVal(accPubkey), numberToI128(1000000000)];

    const contract = new Contract("CCU4UCIHCQRU3YY4465SWCQYSQYC5STUAPGKR3MDSWMESVSGOT3IPZOG");


    const fee = "100";

    const transaction = new TransactionBuilder(owner_account, { fee, networkPassphrase: TESTNET_DETAILS.networkPassphrase, }).
        addOperation(contract.call("mint", ...params)).setTimeout(30).build();


    const preparedtransaction = await server.prepareTransaction(transaction);


    preparedtransaction.sign(owner);


    const tx = TransactionBuilder.fromXDR(preparedtransaction.toXDR(), TESTNET_DETAILS.networkPassphrase);

    const sendResponse = await server.sendTransaction(tx);


    if (sendResponse.errorResult) {

        console.log("Failed", sendResponse.errorResult);

        showToast("Please try after some time");

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
            
            showToast("Successfully minted 10 USDC");

            if (txResponse.returnValue) {

                return scValToNative(txResponse.returnValue)
            }
        }
        // eslint-disable-next-line no-else-return
    }
    console.log("Failed Last")
};
