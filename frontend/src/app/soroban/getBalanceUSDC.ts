import { TESTNET_DETAILS } from "./network";
import { StellarWalletsKit } from "@sekmet/stellar-wallets-kit";
import { accountToScVal, simulateTx } from "./generalUtils";

import {
  TransactionBuilder,
  SorobanRpc,
  Contract, TimeoutInfinite,
  nativeToScVal
} from '@stellar/stellar-sdk';
import { CONTRACT_ADDRESS, FACTORY_CONTRACT_ADDRESS } from "./default_data";



export const getBalanceUSDC = async (
  server: SorobanRpc.Server,
  walletConnectKit: StellarWalletsKit | undefined,
) => {

  const accPubkey = await walletConnectKit!.getPublicKey();

  const account = await server.getAccount(accPubkey);

  const params = [accountToScVal(accPubkey)];

  const contract = new Contract("CB7XVGJGKZNHPAATSVP67VOOIYJ4EPQZ5IMSGWAGDHDO6JW4NRIA5UPU");


  const fee = "100";

  const tx = new TransactionBuilder(account, { fee, networkPassphrase: TESTNET_DETAILS.networkPassphrase, })
    .addOperation(contract.call("balance", ...params))
    .setNetworkPassphrase(TESTNET_DETAILS.networkPassphrase)
    .setTimeout(TimeoutInfinite)
    .build();


  const result = await simulateTx<string>(tx, server);

  return result;
};
