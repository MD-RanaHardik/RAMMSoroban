import { TESTNET_DETAILS } from "./network";
import { StellarWalletsKit } from "@sekmet/stellar-wallets-kit";
import { accountToScVal, addressToByte, numberToI128, numberTou32, simulateTx, stringToString } from "./generalUtils";

import {
  TransactionBuilder,
  SorobanRpc,
  Contract,
} from '@stellar/stellar-sdk';
import { CONTRACT_ADDRESS } from "./default_data";
import { ERRORS, SendTxStatus } from "./erros";



export const mintToken = async (
    server: SorobanRpc.Server,
    walletConnectKit:StellarWalletsKit,
    token_id:string,

  ) => {

    const accPubkey = await walletConnectKit.getPublicKey();

    const account = await server.getAccount(accPubkey);

    const params = [accountToScVal(token_id),accountToScVal(accPubkey),accountToScVal(accPubkey),numberToI128(100)];
    
    const contract = new Contract("CCU4UCIHCQRU3YY4465SWCQYSQYC5STUAPGKR3MDSWMESVSGOT3IPZOG");


    const fee = "100";

    const transaction = new TransactionBuilder(account, { fee, networkPassphrase: TESTNET_DETAILS.networkPassphrase, }).
        addOperation(contract.call("mint", ...params)).setTimeout(30).build();
    

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
            

              return "Success"
            
          }
          // eslint-disable-next-line no-else-return
      }
      
      return ERRORS.UNABLE_TO_SUBMIT_TX;
  };
