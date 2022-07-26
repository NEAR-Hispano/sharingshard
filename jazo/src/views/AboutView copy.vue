<template>
  <div class="about">
    <h1 class="subheading grey--text">Modify the content of a Moment</h1>
    <p>

<br>
</p>
    <v-container class="my-5">

      <p> ------  {{ statusT }}   ----- </p>

      <v-btn class="pa-2"
        depressed
        color="primary"
        @click=addMoment
      >
        Insert a moment description
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

      new_experience: false,
      setMomment: {
        video_n: 1,
        comment: "Test1: We are looking for ... ",
      },


        statusT: "inicial"
      }
    },

    methods: {
      async addMoment(){
        const near = await connect(config);
        const wallet = new WalletConnection(near, 'SharingShard');

        const contract = new Contract( wallet.account(), CONTRACT_ID, 
        { 
          changeMethods:  ['set_moment_comment'],
          sender: wallet.account(),
        });

// near call <CONTRACT OWNER WALLET> set_moment_comment --args '{"video_n": u128, "comment": "<MOMENT COMMENT>"}' --accountId <CALLER WALLET>



        this.expInfo = await contract.set_moment_comment(
          { 
          video_n : this.setMomment.video_n,
          comment: this.setMomment.comment
          }, 
          300000000000000,
          0
        );

        this.statusT = "set_moment_comment result: " + this.expInfo;
        console.log( this.expInfo );
      },


    },
  }

</script>