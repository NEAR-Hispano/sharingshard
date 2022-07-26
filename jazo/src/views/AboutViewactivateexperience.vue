<template>
  <div class="about">
    <h1 class="subheading grey--text">Activate an experience</h1>
    <p>

<br>
</p>
    <v-container class="my-5">

      <p> ------  {{ statusT }}   ----- </p>
      <p> Reward: {{ wRewards }} Earning: {{ wEarning }} --- total: {{ wRewards + wEarning }}</p>

      <v-btn class="pa-2"
        depressed
        color="primary"
        @click=activateExperience
      >
        Activate an experience and deposit the money for the reward
      </v-btn>
      
      <v-spacer></v-spacer>


  </v-container>
</div>


</template>

<script>
  import * as nearAPI from 'near-api-js'
  const { connect, WalletConnection, keyStores, Contract, utils } = nearAPI;

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
        new_experience: false,

        statusT: "inicial",
        wRewards: 0,
        wEarning: 0,
      }
    },

    methods: {
      async activateExperience(){

        const video_n = 214

        const near = await connect(config);
        const wallet = new WalletConnection(near, 'SharingShard');

        const contract = new Contract( wallet.account(), CONTRACT_ID, 
        { 
          changeMethods:  ['activate_experience'],
          viewMethods: ['get_experience'],
          sender: wallet.account(),
        });


        this.exp_info = await contract.get_experience({
          video_n: video_n // video_id
        });

        this.wRewards = this.exp_info.reward * 0.9
        this.wEarning = this.exp_info.reward * 0.1

//const amountInYocto = utils.format.parseNearAmount("10");


        this.expInfo = await contract.activate_experience(
          { 
            video_n : video_n,
          }, 
          300000000000000,
          utils.format.parseNearAmount("11")
        )
        .catch((err) => {
          alert ( err );
          return err;
        }
        );

        this.statusT = "set_moment_comment result: " + this.expInfo;
        console.log( this.expInfo );
      },


    },
  }

</script>