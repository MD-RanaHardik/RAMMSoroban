import { TESTNET_DETAILS } from "./network";
import { StellarWalletsKit } from "@sekmet/stellar-wallets-kit";
import { accountToScVal, simulateTx } from "./generalUtils";

import {
  TransactionBuilder,
  SorobanRpc,
  Contract, TimeoutInfinite,
  nativeToScVal
} from '@stellar/stellar-sdk';
import { FACTORY_CONTRACT_ADDRESS } from "./default_data";


// --- Used to get balance of pool using their pool id
export const getBalance = async (
  server: SorobanRpc.Server,
  walletConnectKit: StellarWalletsKit | undefined,
  pool_id: string | undefined
) => {
  // Get public key
  const accPubkey = await walletConnectKit!.getPublicKey();
  // Get account using public key
  const account = await server.getAccount(accPubkey);

  const params = [nativeToScVal(pool_id), accountToScVal(accPubkey)];
  // get contract using factory contract address
  const contract = new Contract(FACTORY_CONTRACT_ADDRESS);

  const fee = "100";
  // build 'get_balance' transaction with parameters
  const tx = new TransactionBuilder(account, { fee, networkPassphrase: TESTNET_DETAILS.networkPassphrase, })
    .addOperation(contract.call("get_balance", ...params))
    .setNetworkPassphrase(TESTNET_DETAILS.networkPassphrase)
    .setTimeout(TimeoutInfinite)
    .build();

  // Simulate transaction
  const result = await simulateTx<string>(tx, server);

  return result;
};
