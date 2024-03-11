'use client';
import { StellarWalletsKit, WalletNetwork, WalletType } from '@sekmet/stellar-wallets-kit';
import React, { useState, useEffect, Dispatch, SetStateAction, createContext } from 'react'

import {
    SorobanRpc,
} from '@stellar/stellar-sdk';
import { usePathname } from 'next/navigation';
import { Network, SUPPORTED_NETWORK, SUPPORTED_NETWORK1 } from '../soroban/default_data';
import Header from '../components/header';
import { ToastContainer, toast } from 'react-toastify';
import { getNetwork } from "@stellar/freighter-api"
import { TESTNET_DETAILS } from '../soroban/network';


const { Server } = SorobanRpc;


type ContextType = {
    activePubkey: string | undefined;
    setActivePubKey: Dispatch<SetStateAction<string | undefined>>,
    walletConnectKit: StellarWalletsKit | undefined,
    showToast: (msg: string) => void
};

const DefaultContextData: ContextType = {
    activePubkey: undefined,
    setActivePubKey: (): string => '',
    walletConnectKit: undefined,
    showToast: () => { }
};


export const server = new Server(
    Network,
    { allowHttp: true }
)


export const Context = createContext<ContextType>(DefaultContextData);



export default function GlobalContext({ children }: React.PropsWithChildren) {
    // Using Stellar Wallet Kit for freighter wallet
    const [walletConnectKit] = useState(new StellarWalletsKit({ network: TESTNET_DETAILS.networkPassphrase as unknown as WalletNetwork, selectedWallet: WalletType.FREIGHTER, }))

    const [activePubkey, setActivePubKey] = useState<string | undefined>(undefined);

    const pathName = usePathname();

    useEffect(() => {
        get_wallet_pubkey();
    }, [activePubkey])


    // --- This function will get connected wallet's public key and will set that key to activePubKey
    async function get_wallet_pubkey() {
        let network = await getNetwork();
        if (network == SUPPORTED_NETWORK || network == SUPPORTED_NETWORK1) {
            if (walletConnectKit) {
                const key = await walletConnectKit.getPublicKey();
                if (key != undefined) {
                    //set public key
                    setActivePubKey(key);
                }
            }
        } else {
            showToast("You are connected to TESTNET switch to FUTURENET");
        }

    }

    function showToast(msg: String) { toast(msg) };

    return (
        <Context.Provider value={{ activePubkey, setActivePubKey, walletConnectKit, showToast }}>
            <ToastContainer />
            <Header activeWalletKey={activePubkey} setActiveWalletKey={setActivePubKey} walletKit={walletConnectKit} showToast={showToast} key="ConnectWallet" />
            {children}
        </Context.Provider>
    )
}



