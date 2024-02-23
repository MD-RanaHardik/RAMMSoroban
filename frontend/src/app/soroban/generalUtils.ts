import {Address,Transaction,Operation,SorobanRpc,scValToNative,nativeToScVal,Memo, MemoType, xdr} from "@stellar/stellar-sdk";

export const simulateTx = async <ArgType>(
    tx: Transaction<Memo<MemoType>, Operation[]>,
    server:SorobanRpc.Server,
  ): Promise<ArgType> => {

    const response = await server.simulateTransaction(tx);
        
    if (
      SorobanRpc.Api.isSimulationSuccess(response) &&
      response.result !== undefined
    ) {
      // console.log(scValToNative(response.result.retval));
      return scValToNative(response.result.retval);
    }
  
    throw new Error("cannot simulate transaction");
  };


  // Can be used whenever you need an Address argument for a contract method
export const accountToScVal = (account: string) =>
new Address(account).toScVal();

// Can be used whenever you need an i128 argument for a contract method
export const numberToI128 = (value: number): xdr.ScVal =>
nativeToScVal(value, { type: "i128" });

export const numberTou32 = (value: number): xdr.ScVal =>
nativeToScVal(value, { type: "u32" });

export const addressToByte = (value: string): xdr.ScVal =>
nativeToScVal(value, { type: "bytes" });

export const stringToString = (value: string): xdr.ScVal =>
nativeToScVal(value);

