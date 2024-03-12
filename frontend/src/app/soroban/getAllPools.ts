import { TESTNET_DETAILS } from "./network";
import { StellarWalletsKit } from "@sekmet/stellar-wallets-kit";
import { accountToScVal, simulateTx } from "./generalUtils";

import {
  Networks,
  TransactionBuilder,
  SorobanRpc,
  Contract, TimeoutInfinite
} from '@stellar/stellar-sdk';
import { FACTORY_CONTRACT_ADDRESS } from "./default_data";


// --- Used to get all the available pools
export const getAllPools = async (
  server: SorobanRpc.Server,
  walletConnectKit: StellarWalletsKit,
) => {
  // Get public key
  const accPubkey = await walletConnectKit.getPublicKey();
  // Get account using public key
  const account = await server.getAccount(accPubkey);
  // get contract using factory contract address
  const contract = new Contract(FACTORY_CONTRACT_ADDRESS);

  const fee = "100";
  // build 'get_pool' transaction
  const tx = new TransactionBuilder(account, { fee, networkPassphrase: TESTNET_DETAILS.networkPassphrase, })
    .addOperation(contract.call("get_pool"))
    .setNetworkPassphrase(Networks.FUTURENET)
    .setTimeout(TimeoutInfinite)
    .build();

  // Simulate transaction
  const result = await simulateTx<string>(tx, server);

  return result;
};
