import { TESTNET_DETAILS } from "./network";
import { StellarWalletsKit } from "@sekmet/stellar-wallets-kit";
import { accountToScVal, simulateTx } from "./generalUtils";

import {
  TransactionBuilder,
  SorobanRpc,
  Contract, TimeoutInfinite,
} from '@stellar/stellar-sdk';

// --- Used to get USDC balance of user
export const getBalanceUSDC = async (
  server: SorobanRpc.Server,
  walletConnectKit: StellarWalletsKit | undefined,
) => {
  // Get public key
  const accPubkey = await walletConnectKit!.getPublicKey();
  // Get account using public key
  const account = await server.getAccount(accPubkey);

  const params = [accountToScVal(accPubkey)];
  // Get contract using contract id
  const contract = new Contract("CB7XVGJGKZNHPAATSVP67VOOIYJ4EPQZ5IMSGWAGDHDO6JW4NRIA5UPU");


  const fee = "100";
  // build 'balance' transaction with parameters
  const tx = new TransactionBuilder(account, { fee, networkPassphrase: TESTNET_DETAILS.networkPassphrase, })
    .addOperation(contract.call("balance", ...params))
    .setNetworkPassphrase(TESTNET_DETAILS.networkPassphrase)
    .setTimeout(TimeoutInfinite)
    .build();

  // Simulate transaction
  const result = await simulateTx<string>(tx, server);

  return result;
};
