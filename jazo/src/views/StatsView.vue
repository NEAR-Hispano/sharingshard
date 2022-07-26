<template>
<div class="stats">
    <h1 class="subheading grey--text">Statistics</h1>

    <v-container class="my-5">

        <p> ------  Statistics  ----- </p>
        <p>Experiences available: {{ss_number_of_experiences}}</p>
        <p>Number of Users: {{ss_number_of_users}}
        <p>Fee for management: {{ss_fee}}%</p>
        <p>Amount of Rewards: {{ss_holdings}} Nears</p>
        <p>Profit: {{ss_earnings}} Nears</p>
        <p><a :href="ss_explorer" target="_blank">Review transactions in Explorer</a></p>

    </v-container>
</div>
</template>

<script>
  import * as nearAPI from 'near-api-js'
  const { connect, WalletConnection, keyStores, Contract } = nearAPI;

  const CONTRACT_ID = "dev-1658426475128-11579451230587";
  const config = {
    networkId: 'testnet',
    keyStore: new keyStores.BrowserLocalStorageKeyStore(),
    nodeUrl: 'https://rpc.testnet.near.org',
    walletUrl: 'https://wallet.testnet.near.org',
    helperUrl: 'https://helper.testnet.near.org',
    explorerUrl: 'https://explorer.testnet.near.org'
  };

  export default {
    name: "StatsView",




    data(){
      return {
        ss_number_of_experiences: 0,
        ss_fee: 0,
        ss_number_of_users: 0,
        ss_users: [],
        ss_holdings: 0,
        ss_earnings: 0,
        ss_explorer: "",
      }
    },

    created(){
        this.get_stats();
        this.ss_explorer= config.explorerUrl + "/accounts/" + CONTRACT_ID;

        
    },

    methods: {
      async get_stats(){

        const near = await connect(config);
        const wallet = new WalletConnection(near, 'SharingShard');

        const contract = new Contract( wallet.account(), CONTRACT_ID, 
        { 
          viewMethods: ['get_number_of_experiences', 'get_fee', 'get_list_of_users', 'get_holdings', 'get_earnings' ],
          sender: wallet.account(),
        });

        this.ss_number_of_experiences = await contract.get_number_of_experiences();
        this.ss_fee = await contract.get_fee();
        this.ss_users = await contract.get_list_of_users();
        this.ss_number_of_users = this.ss_users.length;
        this.ss_holdings = await contract.get_holdings();
        this.ss_earnings = await contract.get_earnings();
      },
    },
  }
</script>