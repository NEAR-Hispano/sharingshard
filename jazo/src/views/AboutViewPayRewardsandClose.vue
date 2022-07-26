<template>
  <div class="about">
    <h1 class="subheading grey--text">Close an experience -- pay the reward</h1>
    <p>

<br>
</p>
    <v-container class="my-5">

      <p> ------  {{ statusT }}   ----- </p>

      <v-btn class="pa-2"
        depressed
        color="primary"
        @click=payRewardAndClose
      >
        Close an experience -- pay the reward
      </v-btn>
      
      <v-spacer></v-spacer>


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
    name: "AboutView",




    data(){
      return {
        exp_info: "",

        statusT: "inicial",
      }
    },

    methods: {
      async payRewardAndClose(){

        const video_n = 1

        const near = await connect(config);
        const wallet = new WalletConnection(near, 'SharingShard');

        const contract = new Contract( wallet.account(), CONTRACT_ID, 
        { 
          changeMethods:  ['pay_reward'],
          viewMethods: ['get_experience'],
          sender: wallet.account(),
        });


        this.exp_info = await contract.get_experience({
          video_n: video_n // video_id
        });

        this.expInfo = await contract.pay_reward(
          { 
            experience_number: video_n,
            wallet: "ss2022.testnet",
          }, 
          300000000000000,
          0
        )
        .catch((err) => {
          alert ( err );
          return err;
        }
        );

        this.statusT = "pay_reward result: " + this.expInfo;
        console.log( this.expInfo );

        this.statusT = await contract.get_experience({
          video_n: video_n // video_id
        });

      },
    },
  }
</script>