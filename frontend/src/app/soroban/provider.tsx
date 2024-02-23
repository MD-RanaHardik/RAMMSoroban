import {Connector, WalletChain} from "@soroban-react/types";
import {} from "@soroban-react/wallet-data"
import { SorobanReactProvider, } from "@soroban-react/core";

import {freighter} from "@soroban-react/freighter";
import { Networks,} from "@stellar/stellar-sdk";


const allowedChains:WalletChain[] = [
  // {
  //   id: "testnet",
  //   name: "Testnet",
  //   // eslint-disable-next-line
  //   networkPassphrase: Networks.TESTNET,
  //   network:"TESTNET",
  //   networkUrl:"https://soroban-testnet.stellar.org"
    
  // },
  {
    id: "futurenet",
    name: "Futurenet",
    // eslint-disable-next-line
    networkPassphrase: Networks.TESTNET,
    network:"FUTURENET",
    networkUrl:"https://rpc-futurenet.stellar.org"

  },

//   {
//     id: "standalone",
//     name: "Standalone",
//     // eslint-disable-next-line
//     networkPassphrase: Networks.STANDALONE,
//   },
];


const allowedConnectors: Connector[] = [
    freighter()
  ];


export default function MySorobanReactProvider({children}:{children: React.ReactNode}) {
    return (
      <SorobanReactProvider
        chains={allowedChains}
        connectors={allowedConnectors}>
          {children}
      </SorobanReactProvider>
    )
  }