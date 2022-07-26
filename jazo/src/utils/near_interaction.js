// near_interaction.js

import * as nearAPI from 'near-api-js'
const { connect, WalletConnection, keyStores, Contract } = nearAPI;

const CONTRACT_ID = "dev-1658426475128-11579451230587";
const config = {
    testnet: {
        networkId: 'testnet',
        keyStore: new keyStores.BrowserLocalStorageKeyStore(),
        nodeUrl: 'https://rpc.testnet.near.org',
        walletUrl: 'https://wallet.testnet.near.org',
        creareWalletUrl: 'https://wallet.testnet.near.org/create',        
        helperUrl: 'https://helper.testnet.near.org',
        explorerUrl: 'https://explorer.testnet.near.org'
    },

    mainnet: {
        networkId: 'mainnet',
        keyStore: new keyStores.BrowserLocalStorageKeyStore(),
        nodeUrl: 'https://rpc.mainnet.near.org',
        walletUrl: 'https://wallet.near.org',
        createWalletUrl: 'https://wallet.near.org/create',        
        helperUrl: 'https://helper.mainnet.near.org',
        explorerUrl: 'https://explorer.near.org'
    },
};

let gWallet = {};
let g_contract=  {};

near_env: `${process.env.VUE_APP_NEAR_ENV}`,


/*
data(){
    return {

    }
}
*/

/**
 *  Near signIn
 
 export async function nearSignIn(URL) {
    (process.env.VUE_APP_NEAR_ENV == "mainnet" ? this.near = await connect(config.mainnet) : this.near = await connect(config.testnet))
    this.wallet = new WalletConnection(this.near, "SharingShard");
    this.wallet.requestSignIn(
      CONTRACT_ID, // contract requesting access
      "SharingShard", // optional,
      URL, //SUCCESS
      URL // FAILURE
    );
  };
*/


initialize_near();

async function initialize_near(  ){

    //gNear = await connect(config);

    gWallet = new WalletConnection( near, 'SharingShard');
    g_contract =  new Contract( gWallet.account(), CONTRACT_ID, {
      ...methodOptions,
      sender: gWallet.account()
    });

    return g_contract;
    

};

export function get_wallet  ()
  return gWallet;
;