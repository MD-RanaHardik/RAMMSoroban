import { TESTNET_DETAILS } from "./network";
import { StellarWalletsKit } from "@sekmet/stellar-wallets-kit";
import { simulateTx } from "./generalUtils";

import {
  Networks,
  TransactionBuilder,
  SorobanRpc,
  Contract, TimeoutInfinite,
  nativeToScVal
} from '@stellar/stellar-sdk';
import { FACTORY_CONTRACT_ADDRESS } from "./default_data";


// --- Used to get buy price of specific pool using pool_id
export const getBuyPrice = async (
  server: SorobanRpc.Server,
  walletConnectKit: StellarWalletsKit | undefined,
  pool_id: string
) => {
  // Get public key
  const accPubkey = await walletConnectKit!.getPublicKey();
  // Get account using public key
  const account = await server.getAccount(accPubkey);
  // Get contract using factory contract address
  const contract = new Contract(FACTORY_CONTRACT_ADDRESS);

  const fee = "100";

  // Build "get_buy_price" transaction
  const tx = new TransactionBuilder(account, { fee, networkPassphrase: TESTNET_DETAILS.networkPassphrase, })
    .addOperation(contract.call("get_buy_price", nativeToScVal(pool_id)))
    .setNetworkPassphrase(Networks.FUTURENET)
    .setTimeout(TimeoutInfinite)
    .build();

  // Simulate transaction
  const result = await simulateTx<string>(tx, server);

  return result;
};
