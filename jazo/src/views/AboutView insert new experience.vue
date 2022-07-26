<template>
  <div class="about">
    <h1 class="subheading grey--text">Add a New Experience</h1>
    <p>

near call dev-1658426475128-11579451230587 set_experience --args '{"wallet": "zavala55.testnet",  "experience_name": "Aprendizajes en el Silicon Valley", "description": "Narración de la Experiencia de Virgilio Raiden en su visita a SV", "url":"https://youtube.com/embed/WCUGI8PGcGw" , "reward": 10,  "moment": "This is a moment 1", "time":100,  "expire_date":20, "topic":2}' --accountId zavala55.testnet
<br>
Add new experience (returns experience's code number):
</p>
    <v-container class="my-5">

      <p> ------  {{ statusT }}   ----- </p>

      <v-btn class="pa-2"
        depressed
        color="primary"
        @click=addNewExperience
      >
        Insert a new experience
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
      newExp: {
        title: "titulo 1",
        description: "Description 1",
        owner: "zavala55.testnet",
        url: "https://youtube.com/embed/H1h24B0xFyI",
        rewards: 10,
        moment: "No hay momento",
        time: 100,
        expire_date: 100,
        topic: 1,
      },


        statusT: "inicial"
      }
    },

    methods: {
      async addNewExperience(){
        const near = await connect(config);
        const wallet = new WalletConnection(near, 'SharingShard');

        const contract = new Contract( wallet.account(), CONTRACT_ID, 
        { 
          changeMethods:  ['set_experience'],
          sender: wallet.account(),
        });

        this.expInfo = await contract.set_experience(
          { 
          experience_name: this.newExp.title, 
          description: this.newExp.description,
          url: this.newExp.url,
          reward: this.newExp.rewards,
          moment: this.newExp.moment,
          time: this.newExp.time,
          expire_date: this.newExp.expire_date,
          topic: this.newExp.topic,
          }, 
          300000000000000,
          0
        );

        this.statusT = "Experiencia: " + this.expInfo;
        console.log( this.expInfo );
      },


    },
  }

</script>