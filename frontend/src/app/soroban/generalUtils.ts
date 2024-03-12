import {
  Address,
  Transaction,
  Operation,
  SorobanRpc,
  scValToNative,
  nativeToScVal,
  Memo, MemoType, xdr
} from "@stellar/stellar-sdk";

// Used to Simulate the trasaction using the build transaction
export const simulateTx = async <ArgType>(
  tx: Transaction<Memo<MemoType>, Operation[]>,
  server: SorobanRpc.Server,
): Promise<ArgType> => {

  const response = await server.simulateTransaction(tx);

  if (
    SorobanRpc.Api.isSimulationSuccess(response) &&
    response.result !== undefined
  ) {
    // return result if simulation is successfull
    return scValToNative(response.result.retval);
  }
  // throws error if can't simulate transaction
  throw new Error("cannot simulate transaction");
};


// Can be used whenever you need an Address argument for a contract method
export const accountToScVal = (account: string) =>
  new Address(account).toScVal();

// Can be used whenever you need an i128 argument for a contract method
export const numberToI128 = (value: number): xdr.ScVal =>
  nativeToScVal(value, { type: "i128" });

// Can be used whenever you need an u32 argument for a contract method
export const numberTou32 = (value: number): xdr.ScVal =>
  nativeToScVal(value, { type: "u32" });

// Can be used whenever you need an bytes argument for a contract method
export const addressToByte = (value: string): xdr.ScVal =>
  nativeToScVal(value, { type: "bytes" });

export const stringToString = (value: string): xdr.ScVal =>
  nativeToScVal(value);

