"use client"
import { Button, Heading, Popover, } from '@radix-ui/themes';
import { ISupportedWallet, StellarWalletsKit, WalletType } from '@sekmet/stellar-wallets-kit'
import { usePathname, useRouter } from 'next/navigation';
import React, { useContext, useEffect, useState } from 'react'
import { getBalanceUSDC } from '../soroban/getBalanceUSDC';
import { Context, server } from '../Context/store';


interface ConnectWalletProp {
    activeWalletKey: string | undefined,
    setActiveWalletKey: React.Dispatch<React.SetStateAction<string | undefined>>,
    walletKit: StellarWalletsKit | undefined
}

// Wallet Connect
export default function ConnectWallet(prop: ConnectWalletProp) {

    const path = usePathname();

    const [usdcbalance,setUSDCBalance] = useState<number>(0);

    const { activePubkey, setActivePubKey, walletConnectKit, showToast } = useContext(Context);

    useEffect(() => {
        GetUSDCBalance();
    }, [])

    // --- This function is used to get USDC balance
    async function GetUSDCBalance() {
        await getBalanceUSDC(server,walletConnectKit).then((result) => {
            // Set USDC balance
            setUSDCBalance(parseInt(result)/ (10**9));
        }).catch((err) => {
            console.log(err)
        });
    }
    


    return (
        <div >
            {
                prop.activeWalletKey ?
                    <Popover.Root>
                        <Popover.Trigger>
                            <Button size={"3"} className='bg-slate-800 flex items-center outline-1 outline-slate-100 text-slate-50 px-4 py-2 rounded-lg ml-auto mr-0' >
                                <svg className='rounded-full mr-2' xmlns="http://www.w3.org/2000/svg" x="0" y="0" height="24" width="24"><rect x="0" y="0" rx="0" ry="0" height="24" width="24" transform="translate(3.58064099454198 3.52187167729838) rotate(155.9 12 12)" fill="#2477e1"></rect><rect x="0" y="0" rx="0" ry="0" height="24" width="24" transform="translate(7.0247189465624436 -14.358447255034422) rotate(393.3 12 12)" fill="#c7145c"></rect><rect x="0" y="0" rx="0" ry="0" height="24" width="24" transform="translate(-17.38126979634894 -8.666666587006299) rotate(289.4 12 12)" fill="#f3ad00"></rect><rect x="0" y="0" rx="0" ry="0" height="24" width="24" transform="translate(15.569874656032885 -23.697661207793633) rotate(438.4 12 12)" fill="#fc1975"></rect></svg>
                                {prop.activeWalletKey.substring(0, 7) + "..."}
                            </Button>
                        </Popover.Trigger>
                        <Popover.Content className='bg-slate-200 outline outline-1 outline-slate-300 px-10 py-5 rounded-md' >
                            <Heading className='text-slate-800 text-center'>USDC</Heading>
                            <p className='text-slate-800 text-center'>{usdcbalance}</p>
                        </Popover.Content>
                    </Popover.Root>

                    : <Button className='bg-slate-800 outline-1 outline-slate-100 text-slate-50 px-4 py-2 rounded-lg ml-auto mr-0' size={"3"} onClick={() => { connectWallet(prop.walletKit, prop.setActiveWalletKey) }}>Connect wallet</Button>
            }

        </div>
    )
}

// Connect Wallet 
export async function connectWallet(walletKit: StellarWalletsKit | undefined, setActiveWalletKey: React.Dispatch<React.SetStateAction<string | undefined>>) {

    if (walletKit != undefined) {

        let d = await walletKit.getSessions();

        console.log(d);

        await walletKit.openModal({

            modalTitle: "Connect Wallet",
            allowedWallets: [
                WalletType.FREIGHTER,
            ],
            onWalletSelected: async (option: ISupportedWallet) => {
                try {
                    console.log(option);
                    // Set selected wallet,  network, and public key
                    await walletKit.setWallet(option.type);

                    const publicKey = await walletKit.getPublicKey();
                    // set public key to activeWalletKey
                    setActiveWalletKey(publicKey.toString());

                } catch (error) {
                    console.log(error);

                }
            },
        });

    }

}