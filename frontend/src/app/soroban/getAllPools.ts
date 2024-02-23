import { TESTNET_DETAILS } from "./network";
import { StellarWalletsKit } from "@sekmet/stellar-wallets-kit";
import { accountToScVal, simulateTx } from "./generalUtils";

import {
  Networks,
  TransactionBuilder,
  SorobanRpc,
  Contract, TimeoutInfinite
} from '@stellar/stellar-sdk';
import { CONTRACT_ADDRESS, FACTORY_CONTRACT_ADDRESS } from "./default_data";



export const getAllPools = async (
  server: SorobanRpc.Server,
  walletConnectKit: StellarWalletsKit,
) => {

  const accPubkey = await walletConnectKit.getPublicKey();

  const account = await server.getAccount(accPubkey);

  const contract = new Contract(FACTORY_CONTRACT_ADDRESS);


  const fee = "100";

  const tx = new TransactionBuilder(account, { fee, networkPassphrase: TESTNET_DETAILS.networkPassphrase, })
    .addOperation(contract.call("get_pool"))
    .setNetworkPassphrase(Networks.FUTURENET)
    .setTimeout(TimeoutInfinite)
    .build();


  const result = await simulateTx<string>(tx, server);
  return result;
};
